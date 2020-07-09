use std::cell::RefCell;
use std::error::Error;
use std::os::raw::{c_int, c_char};
use std::ffi::CString;

//NOTE: Error handling is done by returning an std::ptr::null_mut
//      instead of a proper pointer to something.
//      Therefore, you must always check for an error before using
//      a return from a function, if the function can error.
//      Check with `last_error_message`.

thread_local!{
    static LAST_ERROR: RefCell<Option<(Box<dyn Error>, i32)>> = RefCell::new(None);
}

/// Update the most recent error, clearing whatever may have been there before.
pub fn update_last_error<E: Error + 'static>(err: E, code: i32) {
    println!("Setting LAST_ERROR: {}", err);
    {
        // Print a pseudo-backtrace for this error, following back each error's
        // cause until we reach the root error.
        let mut cause = err.cause();
        while let Some(parent_err) = cause {
            println!("Caused by: {}", parent_err);
            cause = parent_err.cause();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some((Box::new(err), code));
    });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<(Box<dyn Error>, i32)> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

#[repr(C)]
pub struct RustError {
    pub err: bool,
    pub message: *mut c_char,
    pub code: i32,
}

#[no_mangle]
pub unsafe extern "C" fn last_error_message() -> RustError {
    let error = match take_last_error() {
        Some(err) => {
            RustError {
                err: true,
                message: CString::new(format!("{}", err.0)).unwrap().into_raw(),
                code: err.1,
            }
        },
        None => {
            RustError {
                err: true,
                message: CString::new("").unwrap().into_raw(),
                code: 0,
            }
        }
    };

    error
}
