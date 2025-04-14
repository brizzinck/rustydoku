#!/bin/bash

# -----------------------------------------------------------------------------
# This script generates a PKCS12 keystore for signing your Android application.
# It automatically fills in the distinguished name (DN) using environment variables.
#
# IMPORTANT:
# - Copy this script, make it executable (chmod +x keystore-generate.sh)
# - NEVER commit real credentials (keystore passwords, etc.) into your repository!
# -----------------------------------------------------------------------------

# Set environment variables for keystore generation.
# Replace these placeholder values with your secure, actual credentials.
export RUSTYDOKU_KEYSTORE_PATH="example.jks"
export RUSTYDOKU_KEYSTORE_PASSWORD="P@$$W0rd"
export RUSTYDOKU_KEY_ALIAS="alias-example"

# Set the distinguished name (DN) for the certificate.
# The DN format: "CN=First Last, OU=Org Unit, O=Organization, L=City, ST=State, C=Country (2-letter code)"
export RUSTYDOKU_DNAME="CN=Lol Kek, OU=LOL, O=KEK, L=Kyiv, ST=Kyiv, C=UA"

# Generate the keystore using keytool.
keytool -genkeypair -storetype PKCS12 -v \
  -keystore "$RUSTYDOKU_KEYSTORE_PATH" \
  -storepass "$RUSTYDOKU_KEYSTORE_PASSWORD" \
  -dname "$RUSTYDOKU_DNAME" \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -alias "$RUSTYDOKU_KEY_ALIAS"
