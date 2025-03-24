#!/bin/bash
set -e

# This script builds the Rustydoku Android app in debug mode.
# It compiles the Rust code for Android architectures using `cargo-ndk`,
# copies the required native C++ shared libraries into the Gradle project,
# and runs the Gradle build to produce a debug APK.

# Step 1: Load environment variables.
# These include paths to Android SDK/NDK and signing keys.
source "$(dirname "$0")/env.dev.sh"

# Step 2: Build the Rust library for Android.
# This will generate the shared libraries (`.so`) for the specified targets.
cargo ndk -t armeabi-v7a -t arm64-v8a \
    -o android-deploy/app/src/main/jniLibs \
    build --release

# Step 3: Copy libc++_shared.so for each target.
# These are required by the Rust dynamic libraries and must be packaged into the APK.
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" \
    android-deploy/app/src/main/jniLibs/arm64-v8a/
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so" \
    android-deploy/app/src/main/jniLibs/armeabi-v7a/

# Step 4: Build the APK using Gradle.
# This compiles and packages the Kotlin/Java/Native code into a debug APK.
cd android-deploy
./gradlew :app:assembleDebug
cd ..

# Step 5: Print path to generated APK.
# The resulting APK can be found in the standard debug output directory.
echo "APK generated at: android-deploy/app/build/outputs/apk/debug/app-debug.apk"
