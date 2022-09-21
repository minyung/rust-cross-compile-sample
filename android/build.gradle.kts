plugins {
    id("org.jlleitschuh.gradle.ktlint") version "11.0.0"
}

subprojects {
    apply(plugin = "org.jlleitschuh.gradle.ktlint")

    repositories {
        mavenCentral()
    }

    ktlint {
        android.set(true)
    }
}

buildscript {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }

    dependencies {
        classpath("com.android.tools.build:gradle:7.3.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.5.30-RC")
        classpath("org.mozilla.rust-android-gradle:plugin:0.9.3")
    }
}
tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
}
