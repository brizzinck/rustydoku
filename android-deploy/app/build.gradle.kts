plugins {
    id("com.android.application")
    kotlin("android")
}

android {
    namespace = "org.skalse.rustydoku"
    compileSdk = 33

    defaultConfig {
        applicationId = "org.skalse.rustydoku"
        minSdk = 26
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"

        ndk {
            abiFilters += listOf("armeabi-v7a", "arm64-v8a")
        }
    }

    signingConfigs {
        create("release") {
            storeFile = file(System.getenv("RUSTYDOKU_KEYSTORE_PATH"))
            storePassword = System.getenv("RUSTYDOKU_KEYSTORE_PASSWORD")
            keyAlias = System.getenv("RUSTYDOKU_KEY_ALIAS")
            keyPassword = System.getenv("RUSTYDOKU_KEYSTORE_PASSWORD")
        }
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            signingConfig = signingConfigs.getByName("release")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    sourceSets["main"].jniLibs.srcDirs("src/main/jniLibs")

    packaging {
        jniLibs {
            pickFirsts += listOf("**/libc++_shared.so")
            useLegacyPackaging = true
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.10.1")
    implementation("androidx.appcompat:appcompat:1.6.1")
}
