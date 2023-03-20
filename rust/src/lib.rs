#[allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate core;

#[cfg(feature = "android")]
mod android;

#[cfg(feature = "ios")]
mod ios;
