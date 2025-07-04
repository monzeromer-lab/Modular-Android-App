use std::sync::{Arc, Mutex};
use jni::JavaVM;
use crate::sensors::SensorManager;
use crate::notifications::NotificationManager;

pub struct NativeActivity {
    sensor_manager: Arc<Mutex<SensorManager>>,
    notification_manager: Arc<Mutex<NotificationManager>>,
    java_vm: Arc<Mutex<JavaVM>>,
    status_callback: Option<Box<dyn Fn(String) + Send + Sync>>,
}

impl NativeActivity {
    pub fn new(java_vm: JavaVM) -> Result<Self, Box<dyn std::error::Error>> {
        let sensor_manager = Arc::new(Mutex::new(SensorManager::new()));
        let notification_manager = Arc::new(Mutex::new(NotificationManager::new()));
        let java_vm = Arc::new(Mutex::new(java_vm));
        
        Ok(Self {
            sensor_manager,
            notification_manager,
            java_vm,
            status_callback: None,
        })
    }
    
    pub fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "android")]
        log::info!("Initializing native activity");
        
        // Start sensor monitoring
        self.start_sensor_monitoring();
        
        // Initialize notifications
        self.initialize_notifications();
        
        #[cfg(target_os = "android")]
        log::info!("Native activity initialized successfully");
        Ok(())
    }
    
    pub fn start_sensor_monitoring(&self) {
        let sensor_manager = self.sensor_manager.clone();
        let java_vm = self.java_vm.clone();
        
        let mut sensor_manager = sensor_manager.lock().unwrap();
        sensor_manager.set_callback(move |sensor_index, value| {
            let sensor_names = ["Accelerometer", "Gyroscope", "Magnetometer"];
            let sensor_name = sensor_names[sensor_index as usize];
            #[cfg(target_os = "android")]
            log::info!("{}: {:.2}", sensor_name, value);
            
            // Call back to Java with sensor data
            if let Ok(java_vm_guard) = java_vm.lock() {
                if let Ok(mut env) = java_vm_guard.attach_current_thread_as_daemon() {
                    if let Ok(class) = env.find_class("com/example/modularandroidapp/RustBridge") {
                        if let Ok(message) = env.new_string(format!("{}: {:.2}", sensor_name, value)) {
                            let _ = env.call_static_method(
                                class,
                                "onSensorData",
                                "(Ljava/lang/String;)V",
                                &[(&message).into()],
                            );
                        }
                    }
                }
            }
        });
        sensor_manager.start_sensor_monitoring();
    }
    
    pub fn initialize_notifications(&self) {
        let notification_manager = self.notification_manager.clone();
        let java_vm = self.java_vm.clone();
        
        let mut notification_manager = notification_manager.lock().unwrap();
        notification_manager.set_callback(move |id, title, message, timestamp| {
            #[cfg(target_os = "android")]
            log::info!("Notification: [{}] {}: {}", timestamp, title, message);
            
            // Call back to Java with notification data
            if let Ok(java_vm_guard) = java_vm.lock() {
                if let Ok(mut env) = java_vm_guard.attach_current_thread_as_daemon() {
                    if let Ok(class) = env.find_class("com/example/modularandroidapp/RustBridge") {
                        if let Ok(title_str) = env.new_string(&title) {
                            if let Ok(message_str) = env.new_string(&message) {
                                let _ = env.call_static_method(
                                    class,
                                    "onNotification",
                                    "(ILjava/lang/String;Ljava/lang/String;)V",
                                    &[id.into(), (&title_str).into(), (&message_str).into()],
                                );
                            }
                        }
                    }
                }
            }
        });
    }
    
    pub fn send_test_notification(&self) {
        let notification_manager = self.notification_manager.lock().unwrap();
        notification_manager.send_system_notification(
            "Test Notification".to_string(),
            "This is a test notification from Rust".to_string(),
        );
    }
    
    pub fn update_status(&self, message: String) {
        #[cfg(target_os = "android")]
        log::info!("Status update: {}", message);
        
        // Call back to Java with status update
        if let Ok(java_vm_guard) = self.java_vm.lock() {
            if let Ok(mut env) = java_vm_guard.attach_current_thread_as_daemon() {
                if let Ok(class) = env.find_class("com/example/modularandroidapp/RustBridge") {
                    if let Ok(message_str) = env.new_string(&message) {
                        let _ = env.call_static_method(
                            class,
                            "onStatusUpdate",
                            "(Ljava/lang/String;)V",
                            &[(&message_str).into()],
                        );
                    }
                }
            }
        }
    }
    
    pub fn set_status_callback(&mut self, callback: impl Fn(String) + Send + Sync + 'static) {
        self.status_callback = Some(Box::new(callback));
    }
    
    pub fn get_sensor_data(&self) -> String {
        let sensor_manager = self.sensor_manager.lock().unwrap();
        let accel = sensor_manager.get_accelerometer_data();
        let gyro = sensor_manager.get_gyroscope_data();
        let mag = sensor_manager.get_magnetometer_data();
        
        format!(
            "Accel: ({:.2}, {:.2}, {:.2})\nGyro: ({:.2}, {:.2}, {:.2})\nMag: ({:.2}, {:.2}, {:.2})",
            accel.x, accel.y, accel.z,
            gyro.x, gyro.y, gyro.z,
            mag.x, mag.y, mag.z
        )
    }
} 