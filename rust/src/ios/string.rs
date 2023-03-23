use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::str;

pub fn to_string(ptr: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(ptr).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}

pub fn to_pointer(string: String) -> *const c_char {
    let cstr = CString::new(string.as_bytes()).unwrap();
    let ptr = cstr.as_ptr();
    mem::forget(cstr);
    ptr
}
