use jni::JavaVM;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::jni_bridge::call_java_method;

pub struct AsyncWorker;

impl AsyncWorker {
    pub fn run_async_task(java_vm: Arc<Mutex<JavaVM>>, delay_ms: i64) {
        log::info!("AsyncWorker: Starting async task with delay: {}ms", delay_ms);
        
        // Simulate some work
        thread::sleep(Duration::from_millis(delay_ms as u64));
        
        // Call back to Java with result
        if let Ok(java_vm_guard) = java_vm.lock() {
            let result = format!("Async task completed after {}ms", delay_ms);
            log::info!("AsyncWorker: {}", result);
            
            call_java_method(&java_vm_guard, "onRustAsyncResult", &result);
        }
    }
    
    pub fn run_periodic_task(java_vm: Arc<Mutex<JavaVM>>, interval_ms: i64, count: i32) {
        log::info!("AsyncWorker: Starting periodic task with interval: {}ms, count: {}", interval_ms, count);
        
        for i in 1..=count {
            thread::sleep(Duration::from_millis(interval_ms as u64));
            
            if let Ok(java_vm_guard) = java_vm.lock() {
                let result = format!("Periodic task iteration {}/{}", i, count);
                log::info!("AsyncWorker: {}", result);
                
                call_java_method(&java_vm_guard, "onRustAsyncResult", &result);
            }
        }
    }
    
    pub fn run_long_running_task(java_vm: Arc<Mutex<JavaVM>>) {
        log::info!("AsyncWorker: Starting long running task");
        
        // Simulate a long-running operation
        for i in 1..=10 {
            thread::sleep(Duration::from_millis(500));
            
            if let Ok(java_vm_guard) = java_vm.lock() {
                let progress = format!("Long running task progress: {}%", i * 10);
                log::info!("AsyncWorker: {}", progress);
                
                call_java_method(&java_vm_guard, "onRustAsyncResult", &progress);
            }
        }
        
        // Final completion message
        if let Ok(java_vm_guard) = java_vm.lock() {
            call_java_method(&java_vm_guard, "onRustAsyncResult", "Long running task completed!");
        }
    }
} 