#!/bin/bash

# ───────────────────────────────────────────────────────────────
#  EXAMPLE ENV FILE FOR LOCAL DEVELOPMENT
#
# This file contains placeholder values for environment variables
# required to build and deploy the Android version of Rustydoku.
#
# You CAN commit this file as a reference (no secrets inside).
# ❌ DO NOT commit your real `env.dev.sh` with secrets or local paths.
#
# Create your own copy:
#     cp scripts/env.dev.example.sh scripts/env.dev.sh
# Then edit it with your actual local paths and credentials.
#
# To load this in scripts:
#     source ./scripts/env.dev.sh
# ───────────────────────────────────────────────────────────────

# Android SDK & NDK paths (replace with your real paths)
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_ROOT="$ANDROID_HOME/ndk/25.2.9519653"
export ANDROID_NDK_HOME="$ANDROID_NDK_ROOT"
export NDK_HOME="$ANDROID_NDK_ROOT"
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$PATH"

# Java path (replace with your system's Java path)
export JAVA_HOME="/usr/lib/jvm/default"

# Android signing configuration (DO NOT use real credentials here)
export RUSTYDOKU_KEYSTORE_PATH="/path/to/jey.jks"
export RUSTYDOKU_KEYSTORE_PASSWORD="P@$$0rd"
export RUSTYDOKU_KEY_ALIAS="rusty-alias"
