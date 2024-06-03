pub mod error;

use jni::objects::{JByteArray, JObject, JString};
use jni::JNIEnv;
use jni::{objects::JClass, sys::jstring};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_demo_RustGreetings_greeting(
    mut env: JNIEnv,
    _: JClass,
    java_pattern: JString,
    result_ptr: jni::objects::JObject,
) -> jstring {
    // Our Java companion code might pass-in "world" as a string, hence the name.
    let world = rust_greeting(
        env.get_string(&java_pattern)
            .expect("invalid pattern string")
            .as_ptr(),
    );
    // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
    // let world_ptr = CString::from_raw(world);
    // let output = env
    //     .new_string(world_ptr.to_str().unwrap())
    //     .expect("Couldn't create java string!");

    let response = vec![1, 2, 3];
    let res = match env.byte_array_from_slice(&response) {
        Ok(res) => set_result(&mut env, result_ptr, res),
        Err(e) => set_error(&mut env, result_ptr, e.into()),
    };

    let output = env
        .new_string(res.to_string())
        .expect("Couldn't create java string!");

    output.into_raw()
}

fn set_result(
    env: &mut JNIEnv,
    result_ptr: JObject,
    res: JByteArray,
) -> jni::sys::jint {
    env.set_field(&result_ptr, "code", "I", jni::objects::JValueGen::Int(200))
        .unwrap();
    env.set_field(
        result_ptr,
        "result",
        // "[Ljava/lang/object;",
        "[B",
        jni::objects::JValueGen::Object(&JObject::from(res)),
    )
    .unwrap();
    0
}

fn set_error(env: &mut JNIEnv, error_ptr: JObject, e: error::Error) -> jni::sys::jint {
    let error_message = e.to_string();
    let error_message_java = env.new_string(error_message).unwrap();
    env.set_field(
        &error_ptr,
        "code",
        "I",
        jni::objects::JValueGen::Int(e.get_status_code()),
    )
    .unwrap();
    env.set_field(
        &error_ptr,
        "message",
        "Ljava/lang/String;",
        jni::objects::JValueGen::Object(&error_message_java),
    )
    .unwrap();
    -1
}

#[repr(C)]
pub struct RustResponse {
    pub code: i32,
    pub message: *const i8,
    pub result: *const i8,
}
