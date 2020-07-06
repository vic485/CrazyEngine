use std::ffi::CString;
use std::boxed::Box;
use std::os::raw::c_char;

pub struct RustCallback {
    callback: extern "stdcall" fn(i32),
}

impl RustCallback {
    fn new(read_fn: extern "stdcall" fn(i32)) -> RustCallback {
        RustCallback {
            callback: read_fn,
        }
    }
}

pub fn set_output_arg<T>(out: *mut T, value: T) {
    unsafe { *out.as_mut().unwrap() = value };
}

#[no_mangle]
pub extern "C" fn create(callback: extern "stdcall" fn(i32), sc: *mut *mut RustCallback) -> u32 {
    set_output_arg(sc, Box::into_raw(Box::new(RustCallback::new(callback))));
    0
}

#[no_mangle]
pub extern "C" fn test(sc: *mut RustCallback) -> u32 {
    let callback = unsafe { sc.as_mut().unwrap() };

    println!("First test");

    let f = callback.callback;
    f(69);

    println!("Second test");

    0
}

#[no_mangle]
pub extern "C" fn test_string() -> *mut c_char {
    let c_str = CString::new("piss").expect("0 byte detected in middle of string!");
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn test_string_cleanup(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }

        CString::from_raw(s)
    };
}
