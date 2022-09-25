use super::jni_helper;
use super::jni_helper::JniError;
use jni::objects::JString;
use jni::JNIEnv;

pub fn call_get_test_string(env: JNIEnv) -> Result<String, JniError> {
    let class_name = "com/minyung/android/mylibrary/RustLib";

    let method_name = "getTestString";
    let method_sig = "()Ljava/lang/String;";

    let rust_lib_class = jni_helper::find_class(env, class_name)?;
    let class_obj = jni_helper::new_object(env, rust_lib_class, "()V", &[])?;
    let test_string_value =
        jni_helper::call_method(env, class_obj, method_name, method_sig, &[])?.l()?;
    let test_string = jni_helper::get_string(env, JString::from(test_string_value))?;

    Ok(test_string)
}
