use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct SensorManager {
    accelerometer_data: Arc<Mutex<SensorData>>,
    gyroscope_data: Arc<Mutex<SensorData>>,
    magnetometer_data: Arc<Mutex<SensorData>>,
    callback: Option<Box<dyn Fn(i32, f32) + Send + Sync>>,
}

#[derive(Clone)]
pub struct SensorData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: u64,
}

impl SensorManager {
    pub fn new() -> Self {
        Self {
            accelerometer_data: Arc::new(Mutex::new(SensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                timestamp: 0,
            })),
            gyroscope_data: Arc::new(Mutex::new(SensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                timestamp: 0,
            })),
            magnetometer_data: Arc::new(Mutex::new(SensorData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                timestamp: 0,
            })),
            callback: None,
        }
    }
    
    pub fn set_callback(&mut self, callback: impl Fn(i32, f32) + Send + Sync + 'static) {
        self.callback = Some(Box::new(callback));
    }
    
    pub fn start_sensor_monitoring(&mut self) {
        let accelerometer_data = self.accelerometer_data.clone();
        let gyroscope_data = self.gyroscope_data.clone();
        let magnetometer_data = self.magnetometer_data.clone();
        
        // Take ownership of the callback
        let callback = self.callback.take();
        
        thread::spawn(move || {
            loop {
                // Simulate sensor data updates
                {
                    let mut accel = accelerometer_data.lock().unwrap();
                    accel.x += 0.1;
                    accel.y += 0.05;
                    accel.z = 9.8 + (accel.x * 0.01).sin();
                    accel.timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }
                
                {
                    let mut gyro = gyroscope_data.lock().unwrap();
                    gyro.x += 0.01;
                    gyro.y += 0.02;
                    gyro.z += 0.005;
                    gyro.timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }
                
                {
                    let mut mag = magnetometer_data.lock().unwrap();
                    mag.x = 25.0 + (mag.x * 0.1).sin() * 5.0;
                    mag.y = 30.0 + (mag.y * 0.1).cos() * 3.0;
                    mag.z = 45.0 + (mag.z * 0.1).sin() * 2.0;
                    mag.timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }
                
                // Call callback with sensor data
                if let Some(callback) = &callback {
                    callback(0, 0.0); // Accelerometer
                    callback(1, 0.0); // Gyroscope
                    callback(2, 0.0); // Magnetometer
                }
                
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
    
    pub fn get_accelerometer_data(&self) -> SensorData {
        self.accelerometer_data.lock().unwrap().clone()
    }
    
    pub fn get_gyroscope_data(&self) -> SensorData {
        self.gyroscope_data.lock().unwrap().clone()
    }
    
    pub fn get_magnetometer_data(&self) -> SensorData {
        self.magnetometer_data.lock().unwrap().clone()
    }
} 