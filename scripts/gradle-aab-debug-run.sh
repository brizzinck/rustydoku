#!/bin/bash
set -e

# ───────────────────────────────────────────────────────────────
# Build Android App Bundle (.aab) and install on connected device
# Requires: cargo-ndk, bundletool, Android SDK & NDK
# Env variables required:
#   RUSTYDOKU_KEYSTORE_PATH
#   RUSTYDOKU_KEYSTORE_PASSWORD
#   RUSTYDOKU_KEY_ALIAS
# ───────────────────────────────────────────────────────────────

source "$(dirname "$0")/env.dev.sh"

echo "Building Rust library for Android..."
cargo ndk -t armeabi-v7a -t arm64-v8a -o android-deploy/app/src/main/jniLibs build --release

echo "Copying libc++_shared.so..."
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" android-deploy/app/src/main/jniLibs/arm64-v8a/
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so" android-deploy/app/src/main/jniLibs/armeabi-v7a/

echo "Building AAB..."
cd android-deploy
./gradlew :app:bundleDebug --warning-mode all
cd ..

AAB_PATH="android-deploy/app/build/outputs/bundle/debug/app-debug.aab"
APKS_PATH="android-deploy/app/build/outputs/bundle/debug/app-debug.apks"

echo "Generating .apks from AAB with signing info..."
bundletool build-apks \
  --bundle="$AAB_PATH" \
  --output="$APKS_PATH" \
  --mode=universal \
  --ks="$RUSTYDOKU_KEYSTORE_PATH" \
  --ks-key-alias="$RUSTYDOKU_KEY_ALIAS" \
  --ks-pass=pass:"$RUSTYDOKU_KEYSTORE_PASSWORD" \
  --key-pass=pass:"$RUSTYDOKU_KEYSTORE_PASSWORD" \
  --overwrite

echo "Installing to device..."
bundletool install-apks \
  --apks=android-deploy/app/build/outputs/bundle/debug/app-debug.apks \
  --adb=$ANDROID_HOME/platform-tools/adb 


echo "Done! Installed from AAB via .apks."
