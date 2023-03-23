use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use crate::ios::string;
use crate::ios::utils::logger::Logger;

#[no_mangle]
pub extern "C" fn rust_init() {
    Logger::init();
}

#[no_mangle]
pub extern "C" fn print_test_log() {
    info!("hello, minyung!")
}

#[no_mangle]
pub extern "C" fn get_hello_string(cstr: *const c_char, num: c_int) -> *const c_char {
    let str = string::to_string(cstr);
    let output = format!("Hello, {} {}", str, num);

    string::to_pointer(output)
}

#[no_mangle]
pub unsafe extern "C" fn free_hello_string(cstr: *const c_char) {
    libc::free(cstr as *mut c_void);
}
