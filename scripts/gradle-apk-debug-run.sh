#!/bin/bash

set -e

# This script builds the Rustydoku Android app in debug mode and installs it on a connected Android device.
# It compiles the Rust library for Android using `cargo-ndk`, prepares native libraries, builds the APK using Gradle,
# and uses `adb` to install the APK on a physical device or emulator.

# Step 1: Load environment variables.
# These include ANDROID_HOME, ANDROID_NDK_HOME, JAVA_HOME, and signing info.
source "$(dirname "$1")/env.dev.sh"

# Step 2: Build the Rust library for Android.
# This compiles the Rust code into `.so` shared libraries for ARMv7 and ARM64.
cargo ndk -t armeabi-v7a -t arm64-v8a \
    -o android-deploy/app/src/main/jniLibs \
    build --release

# Step 3: Copy libc++_shared.so libraries required by Rust into the jniLibs folder.
# These are needed at runtime and must be included in the APK.
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" \
    android-deploy/app/src/main/jniLibs/arm64-v8a/
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so" \
    android-deploy/app/src/main/jniLibs/armeabi-v7a/

# Step 4: Build the debug APK using Gradle.
# This step packages the Android project along with the native libraries into an installable APK.
cd android-deploy
./gradlew :app:assembleDebug --warning-mode all
cd ..

# Step 5: Install the APK on the connected Android device using adb.
# The `-r` flag allows reinstalling without uninstalling first.
echo "Installing APK to connected device..."
adb install -r android-deploy/app/build/outputs/apk/debug/app-debug.apk

# Step 6: Done!
echo "Done!"
