#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate core;

#[cfg(target_os = "android")]
mod android;
