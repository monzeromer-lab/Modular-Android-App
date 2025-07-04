# Development Guide

## 🚀 Current Status

The project is fully functional with the following features implemented and tested:

### ✅ Working Features

- **Rust Sum Function**: Basic JNI communication (10 + 20)
- **Library Update System**: Dynamic library loading with SHA-256 verification
- **Slint UI Integration**: Modern UI framework with Android backend
- **Native Activity**: Android native activity support
- **Sensor Monitoring**: Real-time accelerometer, gyroscope, magnetometer data
- **Notification System**: Android notifications from Rust
- **Async Callbacks**: Background task processing with Java callbacks
- **Status Updates**: Real-time status updates from Rust

### 🔧 Recent Fixes

- **JNI Bridge**: Fixed uninitialized native activity access
- **Error Handling**: Added proper error handling for all native functions
- **Build System**: Resolved Gradle and resource conflicts
- **Slint Integration**: Upgraded to Slint 1.12 with proper Android backend
- **XML Resources**: Fixed unescaped ampersand in activity_main.xml
- **Kotlin Methods**: Resolved method name conflicts in RustBridge.kt

## 🛠️ Development Environment

### Required Tools

- **Java 17**: `export JAVA_HOME=/path/to/java17`
- **Android SDK**: API 21+ with NDK
- **Rust Toolchain**: Latest stable version
- **Android Device**: Physical device recommended for testing

### Environment Variables

```bash
export ANDROID_SDK_HOME=/path/to/android/sdk
export ANDROID_NDK_HOME=/path/to/android/ndk
export SLINT_BACKEND=winit
```

## 📁 Code Structure

### Android Components

```bash
android/app/src/main/java/com/example/modularandroidapp/
├── MainActivity.kt          # Main UI and test interface
├── RustBridge.kt           # JNI interface wrapper
├── LibraryManager.kt       # Library loading and updates
├── DownloadService.kt      # File download service
└── NotificationService.kt  # Notification management
```

### Rust Components

```bash
rust/src/
├── lib.rs                  # Library entry point and JNI initialization
├── jni_bridge.rs          # JNI function implementations
├── native_activity.rs     # Native activity and Slint UI
├── sensors.rs             # Sensor data management
├── notifications.rs       # Notification system
├── async_worker.rs        # Background task processing
├── slint_ui.rs           # Slint UI integration
└── utils.rs              # Utility functions
```

## 🔧 Build Process

### Complete Build

```bash
./build.sh
```

### Step-by-Step Build

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

## 🧪 Testing

### Manual Testing Checklist

1. **Basic Functions**:
   - [ ] Test Rust Sum (should return 30)
   - [ ] Check Library Status (should show "loaded")
   - [ ] Update Library (should show progress)

2. **Native Activity**:

   - [ ] Initialize Native Activity (should succeed)
   - [ ] Send Test Notification (should show notification)
   - [ ] Update Status by Rust (should update UI)
   - [ ] Get Sensor Data (should return sensor values)

3. **Async Features**:

   - [ ] Test Async Callback (should show progress after 2 seconds)

### Debug Commands

```bash
# View Rust logs
adb logcat | grep mainlogic

# View Android logs
adb logcat | grep MainActivity

# View all app logs
adb logcat | grep -E "(mainlogic|MainActivity|RustBridge)"
```

## 🐛 Known Issues and Solutions

### Issue: App Crashes on Native Functions

**Cause**: Native activity not initialized
**Solution**: Always call "Initialize Native Activity" first

### Issue: Build Failures

**Cause**: Java version mismatch
**Solution**: Ensure Java 17 is installed and set as JAVA_HOME

### Issue: Slint UI Not Rendering

**Cause**: Backend not properly configured
**Solution**: Set `SLINT_BACKEND=winit` environment variable

### Issue: Sensor Data Not Updating

**Cause**: Sensor monitoring not started
**Solution**: Initialize native activity to start sensor monitoring

## 🔄 Development Workflow

### Adding New Features

1. **Rust Functions**:

   ```rust
   // In rust/src/jni_bridge.rs
   #[no_mangle]
   pub extern "C" fn Java_com_example_modularandroidapp_RustBridge_newFunction(
       env: JNIEnv,
       _class: JClass,
       // parameters
   ) -> jni::sys::jstring {
       // Implementation
   }
   ```

2. **Kotlin Wrapper**:

   ```kotlin
   // In RustBridge.kt
   external fun newFunction(): String
   
   fun newFunctionWrapper(): String {
       return try {
           newFunction()
       } catch (e: Exception) {
           Log.e(TAG, "Error calling newFunction", e)
           "Error"
       }
   }
   ```

3. **UI Integration**:

   ```kotlin
   // In MainActivity.kt
   binding.btnNewFeature.setOnClickListener {
       lifecycleScope.launch {
           val result = withContext(Dispatchers.IO) {
               rustBridge.newFunctionWrapper()
           }
           updateStatus("New feature: $result")
       }
   }
   ```

### Testing New Features

1. **Unit Testing**:

   ```bash
   cd rust
   cargo test
   ```

2. **Integration Testing**:

   - Build and install app
   - Test feature manually
   - Check logcat for errors

3. **Performance Testing**:
   - Monitor memory usage
   - Check CPU usage
   - Verify responsiveness

## 📊 Performance Monitoring

### Memory Usage

- Monitor with `adb shell dumpsys meminfo`
- Check for memory leaks in JNI calls
- Verify proper cleanup in onDestroy

### CPU Usage

- Monitor with `adb shell top`
- Check background thread usage
- Verify async task efficiency

### Battery Impact

- Monitor sensor usage
- Check notification frequency
- Verify background processing

## 🔒 Security Considerations

### Library Verification

- SHA-256 checksums for all downloads
- HTTPS-only downloads
- Internal storage only

### Permissions

- Minimal required permissions
- Runtime permission requests
- Secure data handling

### Code Security

- No hardcoded secrets
- Secure JNI communication
- Input validation

## 🚀 Deployment

### Release Build

```bash
cd android
./gradlew assembleRelease
```

### Signing

```bash
# Create keystore
keytool -genkey -v -keystore app.keystore -alias app -keyalg RSA -keysize 2048 -validity 10000

# Sign APK
jarsigner -verbose -sigalg SHA1withRSA -digestalg SHA1 -keystore app.keystore app-release-unsigned.apk app
```

### Distribution

- Upload to Google Play Store
- Distribute via APK file
- Use internal testing track

## 📈 Future Enhancements

### Planned Features

- [ ] WebSocket communication
- [ ] Database integration
- [ ] Cloud synchronization
- [ ] Advanced UI animations
- [ ] Multi-language support

### Performance Improvements

- [ ] Memory optimization
- [ ] Battery optimization
- [ ] Startup time reduction
- [ ] Background processing optimization

### Security Enhancements

- [ ] Certificate pinning
- [ ] Runtime integrity checks
- [ ] Secure storage
- [ ] Network security

## 🤝 Contributing

### Code Style

- **Rust**: Follow rustfmt guidelines
- **Kotlin**: Follow Android coding standards
- **Slint**: Follow Slint UI guidelines

### Commit Messages

- Use conventional commits
- Include issue numbers
- Describe changes clearly

### Pull Requests

- Include tests
- Update documentation
- Follow review process

## 📞 Support

### Getting Help

1. Check this development guide
2. Review troubleshooting section
3. Check logcat output
4. Create detailed issue report

### Issue Reporting

Include:

- Device information
- Android version
- Steps to reproduce
- Logcat output
- Expected vs actual behavior

---

**Note**: This development guide is updated regularly. Check for the latest version when working on the project.
