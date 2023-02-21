import org.jetbrains.kotlin.util.capitalizeDecapitalize.toLowerCaseAsciiOnly

plugins {
    id("com.android.library")
    kotlin("android")
    id("org.mozilla.rust-android-gradle.rust-android")
}

android {
    compileSdk = 32

    defaultConfig {
        minSdk = 21
        targetSdk = 32

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro"
            )
        }
        debug {
            packagingOptions {
                jniLibs.keepDebugSymbols += "**/*.so"
            }
            isJniDebuggable = true
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    ndkVersion = "25.2.9519653"
}

cargo {
    val startTaskNames = gradle.startParameter.taskNames.toString()

    profile = if (startTaskNames.toLowerCaseAsciiOnly().contains("debug")) "debug"
    else "release"
    module = "../../rust"
    libname = "rustlib"
    targets = listOf("arm", "arm64", "x86", "x86_64")
    features {
        defaultAnd(arrayOf("android"))
    }
}
tasks.whenTaskAdded {
    if ((name == "javaPreCompileDebug") || (name == "javaPreCompileRelease")) {
        dependsOn("cargoBuild")
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.7.0")
    implementation("androidx.appcompat:appcompat:1.3.0")
    implementation("com.google.android.material:material:1.4.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.3")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
}
