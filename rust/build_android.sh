#!/bin/bash

# Build script for Android Rust library
# Requires: cargo-ndk, Android NDK

set -e

echo "Building Rust library for Android..."

# Check if cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null; then
    echo "cargo-ndk not found. Installing..."
    cargo install cargo-ndk
fi

# Set Android NDK path (adjust as needed)
export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Library/Android/sdk/ndk/26.1.10909125}
export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME

# Set Slint backend to use winit with software renderer to avoid Skia dependencies
export SLINT_BACKEND=winit-software

if [ ! -d "$ANDROID_NDK_HOME" ]; then
    echo "Android NDK not found at $ANDROID_NDK_HOME"
    echo "Please set ANDROID_NDK_HOME environment variable"
    exit 1
fi

echo "Using Android NDK: $ANDROID_NDK_HOME"
echo "Using Slint backend: $SLINT_BACKEND"

# Create output directory
mkdir -p ../android/app/src/main/jniLibs

# Build for all target architectures
TARGETS=("aarch64-linux-android" "i686-linux-android" "x86_64-linux-android")

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    
    # Determine ABI directory name
    case $target in
        "aarch64-linux-android")
            abi="arm64-v8a"
            ;;
        "armv7-linux-androideabi")
            abi="armeabi-v7a"
            ;;
        "i686-linux-android")
            abi="x86"
            ;;
        "x86_64-linux-android")
            abi="x86_64"
            ;;
    esac
    
    # Build the library
    cargo ndk --target $target --platform 23 -- build --release
    
    # Copy the built library to the Android project
    mkdir -p "../android/app/src/main/jniLibs/$abi"
    cp "target/$target/release/libmainlogic.so" "../android/app/src/main/jniLibs/$abi/"
    
    echo "Built and copied library for $abi"
done

echo "Build completed successfully!"
echo "Libraries copied to android/app/src/main/jniLibs/"

# Show file sizes
echo "Library sizes:"
for abi in arm64-v8a armeabi-v7a x86 x86_64; do
    if [ -f "../android/app/src/main/jniLibs/$abi/libmainlogic.so" ]; then
        size=$(ls -lh "../android/app/src/main/jniLibs/$abi/libmainlogic.so" | awk '{print $5}')
        echo "  $abi: $size"
    fi
done 