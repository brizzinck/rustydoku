#!/bin/bash

# -----------------------------------------------------------------------------
# This script temporarily replaces Cargo.toml with a generated version that 
# includes signing credentials, builds and runs the Android app using cargo-apk,
# and then restores the original Cargo.toml.
#
# IMPORTANT:
# - Do not commit real credentials into your repository.
# - Ensure that the following environment variables are set with your secure values:
#     RUSTYDOKU_KEYSTORE_PATH, RUSTYDOKU_KEYSTORE_PASSWORD, RUSTYDOKU_KEY_ALIAS
# -----------------------------------------------------------------------------

# Set environment variables for signing (replace these with your actual secure values).
export RUSTYDOKU_KEYSTORE_PATH="example.jks"
export RUSTYDOKU_KEYSTORE_PASSWORD="P@$$W0rd"
export RUSTYDOKU_KEY_ALIAS="alias-example"

# Backup the original Cargo.toml.
cp Cargo.toml Cargo.toml.bak

# Generate a temporary Cargo.toml by substituting placeholders with environment variables.
# Your Cargo.toml (or template) should contain placeholders like ${RUSTYDOKU_KEYSTORE_PATH}
envsubst < Cargo.toml.bak > Cargo.toml

echo "Generated Cargo.toml with signing credentials."

# Build and run the Android app using cargo-apk.
# The --manifest-path Cargo.toml argument tells cargo-apk to use the generated manifest.
cargo apk run -p rustydoku --release --lib --manifest-path Cargo.toml

# Restore the original Cargo.toml from backup.
mv Cargo.toml.bak Cargo.toml

echo "Restored original Cargo.toml."
