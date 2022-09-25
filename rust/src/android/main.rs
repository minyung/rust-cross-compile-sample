extern crate android_logger;
extern crate jni;

use super::jvm::jni_helper;
use super::jvm::sdk_class_api::call_get_test_string;
use super::utils::Logger;

use jni::sys::{jint, jstring, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM, NativeMethod};
use jni::objects::{JClass, JString};
use std::os::raw::c_void;

fn get_class_name() -> String {
    "com/minyung/android/mylibrary/RustLib$Companion".to_string()
}

fn get_native_methods() -> [NativeMethod; 2] {
    [
        NativeMethod {
            name: "printTestLog".into(),
            sig: "()V".into(),
            fn_ptr: print_test_log as *mut c_void,
        },
        NativeMethod {
            name: "getHelloString".into(),
            sig: "(Ljava/lang/String;I)Ljava/lang/String;".into(),
            fn_ptr: get_hello_string as *mut c_void,
        },
    ]
}

fn print_test_log() {
    info!("hello, minyung");
}

fn get_hello_string(env: JNIEnv, _class: JClass, name: JString, number: jint) -> jstring {
    let name: String = match jni_helper::get_string(env, name) {
        Ok(value) => value.into(),
        Err(err) => {
            error!("{:?}", err);
            panic!("")
        }
    };
    let output = format!("Hello, {} {}", name, number);
    env.new_string(output).unwrap().into_inner()
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    let env: JNIEnv = vm.get_env().expect("Cannot get referece to the JNIEnv");

    Logger::init();

    info!("JNI_OnLoad");

    register_methods(env);

    let result = match call_get_test_string(env) {
        Ok(value) => value,
        Err(err) => format!("{:?}", err),
    };
    info!("call_get_test_string: {}", result);

    JNI_VERSION_1_6
}

fn register_methods(env: JNIEnv) {
    let sdk_class = match env.find_class(get_class_name()) {
        Ok(clazz) => clazz,
        Err(err) => {
            info!("error: {:?}", err);
            return;
        }
    };

    match env.register_native_methods(sdk_class, &get_native_methods()) {
        Ok(_) => {
            info!("success register method");
        }
        Err(err) => {
            info!("failed register methods: {:?}", err);
        }
    };
}
