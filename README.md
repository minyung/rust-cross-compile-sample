# rust-cross-compile-sample 
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
