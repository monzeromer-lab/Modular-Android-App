# Modular Android App with Rust Integration

A sophisticated Android application that demonstrates modular architecture with Rust backend integration, Slint UI framework, native activity support, sensor monitoring, notifications, and runtime library updates.

## 🚀 Features

### Core Functionality

- **Rust Backend**: High-performance native code with JNI bridge
- **Slint UI Integration**: Modern UI framework with business logic
- **Native Activity**: Android native activity support
- **Sensor Monitoring**: Real-time accelerometer, gyroscope, and magnetometer data
- **Notification System**: Android notifications from Rust
- **Async Callbacks**: Background task processing with Java callbacks
- **Runtime Updates**: Dynamic library loading and updates
- **SHA-256 Verification**: Secure library integrity checks

### Technical Stack

- **Android**: Kotlin, Gradle, Android SDK
- **Rust**: Native library with JNI bindings
- **Slint**: UI framework with Android backend
- **Build System**: Multi-platform build scripts
- **CI/CD**: GitHub Actions automation

## 📁 Project Structure

```bash
Modular Android App/
├── android/                 # Android application
│   ├── app/                # Main app module
│   │   ├── src/main/
│   │   │   ├── java/      # Kotlin source code
│   │   │   ├── res/       # Android resources
│   │   │   └── jniLibs/   # Native libraries
│   │   └── build.gradle   # App-level build config
│   ├── build.gradle        # Project-level build config
│   └── gradle.properties   # Gradle properties
├── rust/                   # Rust backend library
│   ├── src/               # Rust source code
│   │   ├── jni_bridge.rs  # JNI interface
│   │   ├── native_activity.rs # Native activity
│   │   ├── sensors.rs     # Sensor management
│   │   ├── notifications.rs # Notification system
│   │   ├── async_worker.rs # Background tasks
│   │   ├── slint_ui.rs    # Slint UI integration
│   │   └── utils.rs       # Utility functions
│   ├── ui/                # Slint UI files
│   │   └── app.slint      # Main UI definition
│   └── Cargo.toml         # Rust dependencies
├── build.sh               # Main build script
├── DEVELOPMENT.md         # Development guide
└── LICENSE               # Project license
```

## 🛠️ Building the Project

### Prerequisites

- Android SDK (API 21+)
- Android NDK
- Rust toolchain
- Java 17
- Android device or emulator

### Quick Build

```bash
# Clone the repository
git clone <repository-url>
cd "Modular Android App"

# Build and install
./build.sh
```

### Manual Build Steps

1. **Build Rust Library**:

   ```bash
   cd rust
   ./build_android.sh
   ```

2. **Build Android App**:

   ```bash
   cd android
   ./gradlew assembleDebug
   ```

3. **Install on Device**:

   ```bash
   adb install app/build/outputs/apk/debug/app-debug.apk
   ```

## 🔧 Configuration

### Environment Variables

- `SLINT_BACKEND`: Set to `winit` for software rendering
- `ANDROID_NDK_HOME`: Path to Android NDK
- `ANDROID_SDK_HOME`: Path to Android SDK

### Build Scripts

- `build.sh`: Main build script for the entire project
- `rust/build_android.sh`: Rust-specific Android build
- `android/gradlew`: Gradle wrapper for Android builds

## 📱 App Features

### Main Interface

The app provides a comprehensive test interface with the following buttons:

1. **Test Rust Sum**: Basic Rust function call (10 + 20)
2. **Update Library**: Dynamic library update with SHA-256 verification
3. **Test Async Callback**: Background task with Java callback
4. **Check Library Status**: Current library state
5. **Initialize Native Activity**: Initialize Slint UI and native components
6. **Send Test Notification**: Send Android notification from Rust
7. **Update Status by Rust**: Update UI status from Rust
8. **Get Sensor Data**: Retrieve real-time sensor data

### Native Activity Features

- **Slint UI Integration**: Modern UI with business logic
- **Sensor Monitoring**: Real-time accelerometer, gyroscope, magnetometer
- **Notification System**: Android notifications from Rust
- **Background Processing**: Async tasks with Java callbacks
- **Status Updates**: Real-time status updates from Rust

## 🔍 Troubleshooting

### Common Issues

1. **App Crashes on Native Functions**:
   - Ensure native activity is initialized first
   - Check logcat for detailed error messages
   - Verify Rust library is properly loaded

2. **Build Failures**:
   - Ensure Java 17 is installed and set as JAVA_HOME
   - Verify Android SDK and NDK paths
   - Check Rust toolchain installation

3. **Slint UI Issues**:
   - Ensure SLINT_BACKEND environment variable is set
   - Check Slint version compatibility
   - Verify Android backend features

### Debug Information

- Check logcat for detailed logs: `adb logcat | grep mainlogic`
- Rust logs are tagged with "mainlogic"
- Android logs are tagged with "MainActivity" and "RustBridge"

## 🧪 Testing

### Manual Testing

1. Install the app on a device
2. Test each button in sequence
3. Check logcat for any errors
4. Verify sensor data updates
5. Test notification delivery

### Automated Testing

- Unit tests for Rust functions
- Integration tests for JNI bridge
- UI tests for Android components

## 🔒 Security

### Library Verification

- SHA-256 checksums for downloaded libraries
- Secure download URLs (HTTPS)
- Integrity verification before loading

### Permissions

- Internet access for library updates
- Notification permissions for alerts
- Sensor access for monitoring

## 📈 Performance

### Optimizations

- Release builds with LTO enabled
- Optimized Rust code with panic=abort
- Efficient JNI bridge design
- Background processing for heavy tasks

### Memory Management

- Proper cleanup in onDestroy
- Arc<Mutex<>> for thread-safe sharing
- Efficient string handling in JNI

## 🤝 Contributing

### Development Setup

1. Fork the repository
2. Set up development environment
3. Create feature branch
4. Make changes and test thoroughly
5. Submit pull request

### Code Style

- Rust: Follow rustfmt guidelines
- Kotlin: Follow Android coding standards
- Slint: Follow Slint UI guidelines

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Slint team for the excellent UI framework
- Rust community for JNI support
- Android team for native activity support
- Contributors and testers

## 📞 Support

For issues and questions:

1. Check the troubleshooting section
2. Review logcat output
3. Create an issue with detailed information
4. Include device information and Android version

---

**Note**: This is a demonstration project showing advanced Android + Rust integration techniques. Use as a reference for similar projects.
