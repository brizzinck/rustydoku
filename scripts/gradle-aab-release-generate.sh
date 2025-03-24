#!/bin/bash

set -e

# This script builds the release Android App Bundle (.aab) for Rustydoku using Rust and Gradle,
# signs it using a provided keystore, and generates a .apks file using bundletool.
# The resulting .aab and .apks files can be uploaded to the Play Store or distributed manually.

# Step 1: Load environment variables (paths and signing info)
source "$(dirname "$0")/env.dev.sh"

# Step 2: Build the Rust library for Android in release mode.
# Generates optimized .so libraries for armeabi-v7a and arm64-v8a and puts them in jniLibs.
echo "Building Rust library for Android (release)..."
cargo ndk -t armeabi-v7a -t arm64-v8a \
    -o android-deploy/app/src/main/jniLibs \
    build --release

# Step 3: Copy the libc++_shared.so runtime libraries required by Rust into the correct ABI folders.
echo "Copying libc++_shared.so..."
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" \
    android-deploy/app/src/main/jniLibs/arm64-v8a/
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so" \
    android-deploy/app/src/main/jniLibs/armeabi-v7a/

# Step 4: Build the Android App Bundle (.aab) using Gradle in release mode.
# This includes compiled code, resources, and native libraries.
echo "Building release AAB..."
cd android-deploy
./gradlew :app:bundleRelease --warning-mode all
cd ..

# Define paths for AAB and APKs output.
AAB_PATH="android-deploy/app/build/outputs/bundle/release/app-release.aab"
APKS_PATH="android-deploy/app/build/outputs/bundle/release/app-release.apks"

# Step 5: Use bundletool to generate a .apks archive from the .aab using signing credentials.
# This is required for sideloading or testing the bundle locally.
echo "Generating .apks from release AAB with signing info..."
bundletool build-apks \
  --bundle="$AAB_PATH" \
  --output="$APKS_PATH" \
  --mode=universal \
  --ks="$RUSTYDOKU_KEYSTORE_PATH" \
  --ks-key-alias="$RUSTYDOKU_KEY_ALIAS" \
  --ks-pass=pass:"$RUSTYDOKU_KEYSTORE_PASSWORD" \
  --key-pass=pass:"$RUSTYDOKU_KEYSTORE_PASSWORD" \
  --overwrite

# Step 6: Done!
# You now have both an `.aab` for the Play Store and `.apks` for sideloading.
echo "Done! Release AAB and .apks are ready for upload or distribution."
