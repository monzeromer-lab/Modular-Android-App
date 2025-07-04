pub mod jni_bridge;
pub mod async_worker;
pub mod utils;
pub mod sensors;
pub mod notifications;
pub mod native_activity;

use std::sync::Once;

#[cfg(target_os = "android")]
use log::LevelFilter;

static INIT: Once = Once::new();

#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: *mut std::ffi::c_void, _reserved: *mut std::ffi::c_void) -> std::ffi::c_int {
    INIT.call_once(|| {
        // Initialize logging for Android
        #[cfg(target_os = "android")]
        {
            android_logger::init_once(
                android_logger::Config::default()
                    .with_tag("mainlogic")
                    .with_max_level(LevelFilter::Debug),
            );
        }
    });
    
    // Return JNI version
    jni::JNIVersion::V6.into()
}

#[no_mangle]
pub extern "C" fn JNI_OnUnload(_vm: *mut std::ffi::c_void, _reserved: *mut std::ffi::c_void) {
    #[cfg(target_os = "android")]
    log::info!("Rust library unloading");
} 