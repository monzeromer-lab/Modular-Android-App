# Rust Library for Modular Android App

This Rust library provides the core logic for the modular Android application. It can be dynamically loaded and updated at runtime.

## Features

- **JNI Integration**: Seamless communication with Android via JNI
- **Async Support**: Background thread operations with Java callbacks
- **Cross-platform**: Builds for all Android architectures
- **Hot Updates**: Supports runtime library updates

## Prerequisites

- Rust 1.75+
- Android NDK r26+
- cargo-ndk 0.9+

## Installation

1. Install cargo-ndk:
   ```bash
   cargo install cargo-ndk
   ```

2. Set up Android NDK:
   ```bash
   export ANDROID_NDK_HOME=/path/to/android/ndk
   ```

## Building

### Quick Build
```bash
./build_android.sh
```

### Manual Build
```bash
# For ARM64
cargo ndk --target aarch64-linux-android --platform 23 -- build --release

# For ARMv7
cargo ndk --target armv7-linux-androideabi --platform 23 -- build --release

# For x86
cargo ndk --target i686-linux-android --platform 23 -- build --release

# For x86_64
cargo ndk --target x86_64-linux-android --platform 23 -- build --release
```

## Architecture

### JNI Bridge (`jni_bridge.rs`)
- Exports native functions for Android to call
- Handles JNI communication in both directions
- Manages JavaVM references for async operations

### Async Worker (`async_worker.rs`)
- Background thread management
- Long-running task support
- Progress reporting to Android

### Utils (`utils.rs`)
- Data processing functions
- Hash calculation
- Input validation

## API Reference

### Native Functions
- `rustSum(a: i32, b: i32) -> i32`: Simple addition
- `rustNotifyJava(message: String)`: Send message to Java
- `rustAsyncCallback(delay_ms: i64)`: Start async operation
- `rustGetVersion() -> String`: Get library version
- `rustProcessData(input: String) -> String`: Process input data

### Java Callbacks
- `onRustEvent(eventType: String, data: String)`: Event from Rust
- `onRustAsyncResult(result: String)`: Async operation result

## Security

- SHA-256 hash verification for downloaded libraries
- Internal storage only for security
- Bundled library fallback

## Development

### Adding New Functions

1. Add function to `jni_bridge.rs`:
   ```rust
   #[no_mangle]
   pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_newFunction(
       env: JNIEnv,
       _class: JClass,
       // parameters...
   ) -> ReturnType {
       // implementation
   }
   ```

2. Add corresponding method to `RustBridge.kt`:
   ```kotlin
   external fun newFunction(/* parameters */): ReturnType
   ```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

## License

MIT License 