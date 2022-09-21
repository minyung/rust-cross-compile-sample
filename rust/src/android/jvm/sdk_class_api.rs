use jni::objects::JString;
use jni::JNIEnv;

pub fn call_get_test_string(env: JNIEnv) -> Option<String> {
    let class_name = "com/minyung/android/mylibrary/RustLib";

    let method_name = "getTestString";
    let method_sig = "()Ljava/lang/String;";

    let rust_lib_class = match env.find_class(class_name) {
        Ok(clazz) => clazz,
        Err(err) => {
            info!("failed find_class: {:?}", err);
            return None;
        }
    };

    let class_obj = match env.new_object(rust_lib_class, "()V", &[]) {
        Ok(obj) => obj,
        Err(err) => {
            info!("failed new_object: {:?}", err);
            return None;
        }
    };

    let package_name = match env.call_method(class_obj, method_name, method_sig, &[]) {
        Ok(name) => JString::from(name.l().unwrap()),
        Err(err) => {
            info!("failed call_methods: {:?}", err);
            return None;
        }
    };

    Some(env.get_string(package_name).unwrap().into())
}
