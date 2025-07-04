use jni::{
    objects::{JClass, JString},
    JNIEnv, JavaVM,
};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::async_worker::AsyncWorker;
use crate::native_activity::NativeActivity;
use crate::utils;

static mut JAVA_VM: Option<Arc<Mutex<JavaVM>>> = None;
static mut NATIVE_ACTIVITY: Option<Arc<Mutex<NativeActivity>>> = None;

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_rustSum(
    _env: JNIEnv,
    _class: JClass,
    a: jni::sys::jint,
    b: jni::sys::jint,
) -> jni::sys::jint {
    log::info!("Rust: rustSum called with {} + {}", a, b);
    a + b
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_rustNotifyJava(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let message_str: String = env.get_string(&message).unwrap().into();
    log::info!("Rust: rustNotifyJava called with message: {}", message_str);
    
    // Call back to Java
    if let Ok(class) = env.find_class("com/example/modularandroidapp/RustBridge") {
        if let Ok(java_message) = env.new_string(&message_str) {
            let _ = env.call_static_method(
                class,
                "onRustEvent",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                &[(&java_message).into(), (&java_message).into()],
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_rustAsyncCallback(
    env: JNIEnv,
    _class: JClass,
    delay_ms: jni::sys::jlong,
) {
    log::info!("Rust: rustAsyncCallback called with delay: {}ms", delay_ms);
    
    // Store JavaVM for use in background thread
    unsafe {
        JAVA_VM = Some(Arc::new(Mutex::new(env.get_java_vm().unwrap())));
    }
    
    // Spawn background thread
    let java_vm = unsafe { JAVA_VM.as_ref().unwrap().clone() };
    thread::spawn(move || {
        AsyncWorker::run_async_task(java_vm, delay_ms);
    });
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_rustGetVersion(
    mut env: JNIEnv,
    _class: JClass,
) -> jni::sys::jstring {
    let version = "1.0.0";
    log::info!("Rust: rustGetVersion called, returning: {}", version);
    env.new_string(version).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_rustProcessData(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jni::sys::jstring {
    let input_str: String = env.get_string(&input).unwrap().into();
    log::info!("Rust: rustProcessData called with input: {}", input_str);
    
    // Process the input data
    let processed = utils::process_input_data(&input_str);
    
    env.new_string(&processed).unwrap().into_raw()
}

// New functions for native activity

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_initializeNativeActivity(
    env: JNIEnv,
    _class: JClass,
) -> jni::sys::jboolean {
    log::info!("Rust: initializeNativeActivity called");
    
    let java_vm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(e) => {
            log::error!("Failed to get JavaVM: {}", e);
            return 0;
        }
    };
    
    match NativeActivity::new(java_vm) {
        Ok(native_activity) => {
            unsafe {
                NATIVE_ACTIVITY = Some(Arc::new(Mutex::new(native_activity)));
            }
            
            // Initialize the native activity
            if let Some(activity_guard) = unsafe { NATIVE_ACTIVITY.as_ref() } {
                if let Ok(activity) = activity_guard.lock() {
                    if let Err(e) = activity.initialize() {
                        log::error!("Failed to initialize native activity: {}", e);
                        return 0;
                    }
                    
                    log::info!("Native activity initialized successfully");
                    return 1;
                }
            }
        }
        Err(e) => {
            log::error!("Failed to create native activity: {}", e);
        }
    }
    
    0
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_sendTestNotification(
    _env: JNIEnv,
    _class: JClass,
) {
    log::info!("Rust: sendTestNotification called");
    
    unsafe {
        if let Some(activity_guard) = NATIVE_ACTIVITY.as_ref() {
            if let Ok(activity) = activity_guard.lock() {
                activity.send_test_notification();
            } else {
                log::error!("Failed to lock native activity for notification");
            }
        } else {
            log::error!("Native activity not initialized");
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_updateStatus(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let message_str: String = env.get_string(&message).unwrap().into();
    log::info!("Rust: updateStatus called with message: {}", message_str);
    
    unsafe {
        if let Some(activity_guard) = NATIVE_ACTIVITY.as_ref() {
            if let Ok(activity) = activity_guard.lock() {
                activity.update_status(message_str);
            } else {
                log::error!("Failed to lock native activity for status update");
            }
        } else {
            log::error!("Native activity not initialized");
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_getSensorData(
    mut env: JNIEnv,
    _class: JClass,
) -> jni::sys::jstring {
    log::info!("Rust: getSensorData called");
    
    unsafe {
        if let Some(activity_guard) = NATIVE_ACTIVITY.as_ref() {
            if let Ok(activity) = activity_guard.lock() {
                let sensor_data = activity.get_sensor_data();
                env.new_string(&sensor_data).unwrap().into_raw()
            } else {
                log::error!("Failed to lock native activity for sensor data");
                env.new_string("Error: Failed to access sensor data").unwrap().into_raw()
            }
        } else {
            log::error!("Native activity not initialized");
            env.new_string("Error: Native activity not initialized").unwrap().into_raw()
        }
    }
}

// Helper function to call Java from Rust
pub fn call_java_method(java_vm: &JavaVM, method_name: &str, data: &str) {
    if let Ok(mut env) = java_vm.attach_current_thread_as_daemon() {
        if let Ok(class) = env.find_class("com/example/modularandroidapp/RustBridge") {
            if let Ok(java_data) = env.new_string(data) {
                let _ = env.call_static_method(
                    class,
                    method_name,
                    "(Ljava/lang/String;)V",
                    &[(&java_data).into()],
                );
            }
        }
    }
} 