#!/bin/bash

# -----------------------------------------------------------------------------
# This script temporarily replaces Cargo.toml with a generated version that 
# includes signing credentials, builds and runs the Android app using cargo-apk,
# and then restores the original Cargo.toml.

source "$(dirname "$0")/env.dev.sh"

# Backup the original Cargo.toml.
cp Cargo.toml Cargo.toml.bak

# Generate a temporary Cargo.toml by substituting placeholders with environment variables.
# Your Cargo.toml (or template) should contain placeholders like ${RUSTYDOKU_KEYSTORE_PATH}
envsubst < Cargo.toml.bak > Cargo.toml

echo "Generated Cargo.toml with signing credentials."

# Build and run the Android app using cargo-apk.
# The --manifest-path Cargo.toml argument tells cargo-apk to use the generated manifest.
x build --release --platform android --store play

# Restore the original Cargo.toml from backup.
mv Cargo.toml.bak Cargo.toml

echo "Restored original Cargo.toml."
