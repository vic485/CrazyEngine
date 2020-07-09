use log::{Record, Level, LevelFilter, Metadata};

use std::os::raw::c_char;
use std::ffi::CString;
use std::sync::Mutex;

use crate::RustCallback;

type LogCallback = extern "stdcall" fn(LogMessage);

#[repr(C)]
pub struct LogMessage {
    pub level: i32,
    pub message: *mut c_char,
    pub target: *mut c_char,
}

static FFI_LOGGER: FFILogger = FFILogger;

lazy_static! {
    static ref INTERNAL_FFI_LOGGER: Mutex<Option<RustCallback<LogCallback>>> = Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn register_logger() {
    match log::set_logger(&FFI_LOGGER).map(|()| log::set_max_level(LevelFilter::max())) {
        Ok(_) => {},
        Err(_) => {},
    }
}

#[no_mangle]
pub extern "C" fn logger_register_func(callback: LogCallback) {
    let mut func = INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!");
    *func = Some(RustCallback::new(callback));
}

pub struct FFILogger;

impl log::Log for FFILogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            //Log to c# through callback
            let message = format!("{}", record.args());

            let log_data = LogMessage {
                level: record.level() as i32,
                message: CString::new(message).unwrap().into_raw(),
                target: CString::new(record.target()).unwrap().into_raw(),
            };

            let option_sc = INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!");
            match &*option_sc {
                Some(sc) => {
                    let func = sc.callback;
                    func(log_data);
                },
                None => {},
            }
        }
    }

    fn flush(&self) {}
}
