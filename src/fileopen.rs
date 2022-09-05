use macroquad::miniquad::native::android::{self, ndk_sys, ndk_utils};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

struct GlobalData {
    openfile: ndk_sys::jobject,
    data: Option<Arc<Mutex<Option<Vec<u8>>>>>,
}
unsafe impl Send for GlobalData {}
unsafe impl Sync for GlobalData {}

static GLOBALS: Lazy<Mutex<GlobalData>> = Lazy::new(|| {
    Mutex::new(GlobalData {
        openfile: std::ptr::null_mut(),
        data: None,
    })
});

#[no_mangle]
pub unsafe extern "C" fn Java_playground_FileOpen_FileOpenInit() {
    let env = android::attach_jni_env();

    let mut globals = GLOBALS.lock().unwrap();
    let openfile = ndk_utils::new_object!(env, "playground/FileOpen", "()V");
    assert!(!openfile.is_null());
    globals.openfile = ndk_utils::new_global_ref!(env, openfile);
}

#[no_mangle]
pub unsafe extern "C" fn Java_playground_FileOpen_FileOpenOnReceive(
    env: *mut ndk_sys::JNIEnv,
    _: ndk_sys::jobject,
    array: ndk_sys::jbyteArray,
) {
    let mut globals = GLOBALS.lock().unwrap();

    let len = ((**env).GetArrayLength.unwrap())(env, array);
    let elements = ((**env).GetByteArrayElements.unwrap())(env, array, std::ptr::null_mut());
    let data = std::slice::from_raw_parts(elements as *mut u8, len as usize);

    if let Some(ref mut d) = globals.data {
        *d.lock().unwrap() = Some(data.to_vec());
    }
    ((**env).ReleaseByteArrayElements.unwrap())(env, array, elements, 0);
}

pub fn find_file(data: Arc<Mutex<Option<Vec<u8>>>>) {
    let env = unsafe { android::attach_jni_env() };
    let mut globals = GLOBALS.lock().unwrap();

    globals.data = Some(data);
    unsafe {
        ndk_utils::call_void_method!(env, globals.openfile, "OpenFileDialog", "()V");
    }
}
