#[macro_use] extern crate lazy_static;

use std::ffi::CString;
use std::boxed::Box;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;

type TestCallback = extern "stdcall" fn(i32);
type GlLoadCallback = extern "stdcall" fn(*mut c_char) -> *mut i32;

pub struct RustCallback<T> {
    callback: T,
}

impl<T> RustCallback<T> {
    fn new(read_fn: T) -> RustCallback<T> {
        RustCallback {
            callback: read_fn,
        }
    }
}

pub fn set_output_arg<T>(out: *mut T, value: T) {
    unsafe { *out.as_mut().unwrap() = value };
}

#[no_mangle]
pub extern "C" fn string_cleanup(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }

        CString::from_raw(s)
    };
}

//Can't mangle generic functions
// #[no_mangle]
// pub extern "C" fn create<T>(callback: T, sc: *mut *mut RustCallback<T>) -> u32 {
//     set_output_arg(sc, Box::into_raw(Box::new(RustCallback::new(callback))));
//     0
// }

#[no_mangle]
pub extern "C" fn create_test_callback(callback: TestCallback, sc: *mut *mut RustCallback<TestCallback>) -> u32 {
    set_output_arg(sc, Box::into_raw(Box::new(RustCallback::new(callback))));
    0
}


#[no_mangle]
pub extern "C" fn test_callback(sc: *mut RustCallback<TestCallback>) -> u32 {
    let callback = unsafe { sc.as_mut().unwrap() };

    let f = callback.callback;
    f(69);

    0
}

lazy_static! {
    static ref GL_LOAD_CALLBACK: Mutex<Option<RustCallback<GlLoadCallback>>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn create_gl_load_callback(callback: GlLoadCallback) -> u32 {
    // GL_LOAD_CALLBACK = Some(RustCallback::new(callback));
    let mut data = GL_LOAD_CALLBACK.lock().unwrap();
    *data = Some(RustCallback::new(callback));
    0
}

#[no_mangle]
pub extern "C" fn gl_callback() {
    let guard = GL_LOAD_CALLBACK.lock().unwrap();
    match &*guard {
        Some(sc) => {
            println!("Callback exists!");
            let proc_func = sc.callback;
            gl::load_with(|s| {
                let proc = CString::new(s).unwrap(); //We know this string won't error
                proc_func(proc.into_raw()) as _
            });
        },
        None => {
            println!("Callback does not exist!");
        },
    }
}

#[no_mangle]
pub extern "C" fn test_render() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::ClearColor(127.0 / 255.0, 103.0 / 255.0, 181.0 / 255.0, 1.0);
    }
}