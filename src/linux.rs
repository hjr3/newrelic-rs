use std::ffi::CString;

mod ffi {
    use std::os::raw::{c_char, c_long, c_int, c_void};

    pub const NEWRELIC_AUTOSCOPE: c_long = 1;
    pub const NEWRELIC_ROOT_SEGMENT: c_long = 0;

    extern "C" {
        pub fn newrelic_init(license: *const c_char, app_name: *const c_char, language: *const c_char, language_version: *const c_char);
        pub fn newrelic_register_message_handler(handler: unsafe extern fn(*mut c_void) -> *mut c_void);
        pub fn newrelic_message_handler(raw_message: *mut c_void) -> *mut c_void;
        pub fn newrelic_transaction_begin() -> c_long;
        pub fn newrelic_transaction_set_name(transaction_id: c_long, name: *const c_char) -> c_int;
        pub fn newrelic_transaction_end(transaction_id: c_long) -> c_int;
        pub fn newrelic_segment_generic_begin(transaction_id: c_long, parent_segment_id: c_long, name: *const c_char) -> c_long;
        pub fn newrelic_segment_external_begin(transaction_id: c_long, parent_segment_id: c_long, host: *const c_char, name: *const c_char) -> c_long;
        pub fn newrelic_segment_end(transaction_id: c_long, segment_id: c_long) -> c_int;
    }
}

pub fn init(license: &str, app_name: &str, language: &str, language_version: &str) {
    let license = CString::new(license).unwrap();
    let app_name = CString::new(app_name).unwrap();
    let language = CString::new(language).unwrap();
    let language_version = CString::new(language_version).unwrap();

    unsafe {
        // we are required to register the message handler prior to calling init
        ffi::newrelic_register_message_handler(ffi::newrelic_message_handler);
        ffi::newrelic_init(license.as_ptr(), app_name.as_ptr(), language.as_ptr(), language_version.as_ptr());
    }
}

/// Identify the beginning of a transaction.
pub fn transaction_begin() -> Result<i64, ()> {
    let transaction_id = unsafe {
        ffi::newrelic_transaction_begin()
    };

    match transaction_id {
        id if id > 0 => Ok(id),
        _ => Err(()),
    }
}

/// Set a name for the transaction
///
/// Must be called after `transaction_begin()` and before `transaction_end()`.
pub fn transaction_set_name(transaction_id: i64, name: &str) -> Result<(), ()> {
    let name = CString::new(name).unwrap();
    let rc = unsafe {
        ffi::newrelic_transaction_set_name(transaction_id, name.as_ptr())
    };

    match rc {
        0 => Ok(()),
        _ => Err(()),
    }
}

/// Identify the end of a transaction
pub fn transaction_end(transaction_id: i64) -> Result<(), ()> {
    let rc = unsafe {
        ffi::newrelic_transaction_end(transaction_id)
    };

    match rc {
        0 => Ok(()),
        _ => Err(()),
    }
}

/// Identify the beginning of a segment that performs a generic operation.
///
/// This type of segment does not create metrics, but can show up in a
/// transaction trace if a transaction is slow enough.
pub fn segment_generic_begin(transaction_id: i64, name: &str) -> Result<i64, ()> {
    let name = CString::new(name).unwrap();

    let transaction_id = unsafe {
        ffi::newrelic_segment_generic_begin(transaction_id, ffi::NEWRELIC_AUTOSCOPE, name.as_ptr())
    };

    match transaction_id {
        id if id > 0 => Ok(id),
        _ => Err(()),
    }
}

/// Identify the beginning of a segment that performs an external service.
pub fn segment_external_begin(transaction_id: i64, host: &str, name: &str) -> Result<i64, ()> {
    let host = CString::new(host).unwrap();
    let name = CString::new(name).unwrap();

    let segment_id = unsafe {
        ffi::newrelic_segment_external_begin(transaction_id, ffi::NEWRELIC_AUTOSCOPE, host.as_ptr(), name.as_ptr())
    };

    match segment_id {
        id if id > 0 => Ok(id),
        _ => Err(()),
    }
}

/// Identify the end of a segment
pub fn segment_end(transaction_id: i64, segment_id: i64) -> Result<(), ()> {
    let rc = unsafe {
        ffi::newrelic_segment_end(transaction_id, segment_id)
    };

    match rc {
        0 => Ok(()),
        _ => Err(()),
    }
}
