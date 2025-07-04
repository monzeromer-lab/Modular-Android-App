#!/bin/bash

# Build script for Modular Android App
# This script builds both the Rust library and Android app

set -e

echo "🚀 Building Modular Android App..."

# Set Java 17 for Android build
export JAVA_HOME=/usr/local/opt/openjdk@17
export PATH="/usr/local/opt/openjdk@17/bin:$PATH"

echo "📦 Building Rust library..."
cd rust
./build_android.sh
cd ..

echo "📱 Building Android app..."
cd android
./gradlew assembleDebug
cd ..

echo "✅ Build completed successfully!"
echo "📱 APK location: android/app/build/outputs/apk/debug/app-debug.apk"
echo "📊 APK size: $(ls -lh android/app/build/outputs/apk/debug/app-debug.apk | awk '{print $5}')"

echo ""
echo "🎯 To install on device:"
echo "   cd android && ./gradlew installDebug"
echo ""
echo "🎯 To run tests:"
echo "   cd android && ./gradlew test" 