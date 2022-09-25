use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;

#[derive(Debug)]
pub enum JniError {
    FindClass,
    NewObject,
    CallMethod,
    GetString,
    Other(jni::errors::Error),
}

pub fn find_class<'a>(env: JNIEnv<'a>, name: &'a str) -> Result<JClass<'a>, JniError> {
    let class = match env.find_class(name) {
        Ok(clazz) => clazz,
        Err(_) => {
            return Err(JniError::FindClass);
        }
    };
    Ok(class)
}

pub fn new_object<'a>(
    env: JNIEnv<'a>,
    class: JClass<'a>,
    sig: &'a str,
    args: &[JValue<'a>],
) -> Result<JObject<'a>, JniError> {
    let class_obj = match env.new_object(class, sig, args) {
        Ok(obj) => obj,
        Err(_) => {
            return Err(JniError::NewObject);
        }
    };
    Ok(class_obj)
}

pub fn call_method<'a>(
    env: JNIEnv<'a>,
    class_obj: JObject<'a>,
    name: &'a str,
    sig: &'a str,
    args: &[JValue<'a>],
) -> Result<JValue<'a>, JniError> {
    let value = match env.call_method(class_obj, name, sig, args) {
        Ok(obj) => obj,
        Err(_) => {
            return Err(JniError::CallMethod);
        }
    };
    Ok(value)
}

pub fn get_string<'a>(env: JNIEnv<'a>, jstr: JString<'a>) -> Result<String, JniError> {
    let str = match env.get_string(jstr) {
        Ok(string) => string.into(),
        Err(_) => {
            return Err(JniError::GetString);
        }
    };
    Ok(str)
}

impl From<jni::errors::Error> for JniError {
    fn from(e: jni::errors::Error) -> Self {
        JniError::Other(e)
    }
}
