#[macro_use]
extern crate log;

#[cfg(target_os = "android")]
extern crate jni;
#[cfg(target_os = "android")]
extern crate android_logger;

#[cfg(target_os = "android")]
mod android;
