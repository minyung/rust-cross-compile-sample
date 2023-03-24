# rust-cross-compile-sample 
[![Android CI](https://github.com/minyung/rust-cross-compile-sample/actions/workflows/android-ci.yml/badge.svg)](https://github.com/minyung/rust-cross-compile-sample/actions/workflows/android-ci.yml)

Android, iOS Sample for Rust.

# Android
## Development environment
- Android Studio 2021.3.1
- ndkVersion 25.1.8937393
- [ktlint-gradle](https://github.com/JLLeitschuh/ktlint-gradle) 11.0.0
- [rust-android-gradle](https://github.com/mozilla/rust-android-gradle) 0.9.3
- minSdk 21, targetSdk 32
```
$ rustup target add armv7-linux-androideabi
$ rustup target add aarch64-linux-android
$ rustup target add i686-linux-android
$ rustup target add x86_64-linux-android
```

# iOS
## Development environment
- Xcode 13.4.1
- [swiftlint](https://github.com/realm/SwiftLint)
```
$ rustup target add aarch64-apple-ios
$ rustup target add x86_64-apple-ios
$ rustup target add aarch64-apple-ios-sim
```
