use log::{Record, Level, LevelFilter, Metadata};

use std::os::raw::c_char;
use std::ffi::CString;
use std::sync::Mutex;

use crate::RustCallback;

type LogCallback = extern "stdcall" fn(*mut c_char);

static FFI_LOGGER: FFILogger = FFILogger;

lazy_static! {
    static ref INTERNAL_FFI_LOGGER: Mutex<InternalFFILogger> = Mutex::new(InternalFFILogger::create());
}

#[no_mangle]
pub extern "C" fn register_logger() {
    match log::set_logger(&FFI_LOGGER).map(|()| log::set_max_level(LevelFilter::max())) {
        Ok(_) => {},
        Err(_) => {},
    }
}

#[no_mangle]
pub extern "C" fn logger_register_error(callback: LogCallback) {
    INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!").error_callback = Some(RustCallback::new(callback));
}

#[no_mangle]
pub extern "C" fn logger_register_warn(callback: LogCallback) {
    INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!").warn_callback = Some(RustCallback::new(callback));
}

#[no_mangle]
pub extern "C" fn logger_register_info(callback: LogCallback) {
    INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!").info_callback = Some(RustCallback::new(callback));
}

#[no_mangle]
pub extern "C" fn logger_register_debug(callback: LogCallback) {
    INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!").debug_callback = Some(RustCallback::new(callback));
}

#[no_mangle]
pub extern "C" fn logger_register_trace(callback: LogCallback) {
    INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!").trace_callback = Some(RustCallback::new(callback));
}

pub struct InternalFFILogger {
    pub error_callback: Option<RustCallback<LogCallback>>,
    pub warn_callback: Option<RustCallback<LogCallback>>,
    pub info_callback: Option<RustCallback<LogCallback>>,
    pub debug_callback: Option<RustCallback<LogCallback>>,
    pub trace_callback: Option<RustCallback<LogCallback>>,
}

impl InternalFFILogger {
    pub fn create() -> Self {
        Self {
            error_callback: None,
            warn_callback: None,
            info_callback: None,
            debug_callback: None,
            trace_callback: None,
        }
    }

    pub fn log(&mut self, record: &Record) {
        let log_msg = format!("{}", record.args());
        match record.level() {
            Error => {
                if let Some(sc) = &self.error_callback {
                    let func = sc.callback;
                    func(CString::new(log_msg).unwrap().into_raw());
                }
            },
            Warn => {
                if let Some(sc) = &self.warn_callback {
                    let func = sc.callback;
                    func(CString::new(log_msg).unwrap().into_raw());
                }
            },
            Info => {
                if let Some(sc) = &self.info_callback {
                    let func = sc.callback;
                    func(CString::new(log_msg).unwrap().into_raw());
                }
            },
            Debug => {
                if let Some(sc) = &self.debug_callback {
                    let func = sc.callback;
                    func(CString::new(log_msg).unwrap().into_raw());
                }
            },
            Trace => {
                if let Some(sc) = &self.trace_callback {
                    let func = sc.callback;
                    func(CString::new(log_msg).unwrap().into_raw());
                }
            },
        }
    }
}

pub struct FFILogger;

impl log::Log for FFILogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            //Log to c# through callback
            INTERNAL_FFI_LOGGER.lock().expect("Failed to get a reference to logger!").log(record);
        }
    }

    fn flush(&self) {}
}
