use std::ffi::{CStr, c_char};

use log::{debug, error, info, trace, warn};

// Logs a message from C++
// TRACE
#[unsafe(no_mangle)]
pub extern "C" fn cxx_trace(payload: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(payload) };
    trace!("[CXX] {}", c_str.to_string_lossy())
}

// Logs a message from C++
// DEBUG
#[unsafe(no_mangle)]
pub extern "C" fn cxx_debug(payload: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(payload) };
    debug!("[CXX] {}", c_str.to_string_lossy())
}

// Logs a message from C++
// INFO
#[unsafe(no_mangle)]
pub extern "C" fn cxx_info(payload: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(payload) };
    info!("[CXX] {}", c_str.to_string_lossy())
}

// Logs a message from C++
// WARNING
#[unsafe(no_mangle)]
pub extern "C" fn cxx_warn(payload: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(payload) };
    warn!("[CXX] {}", c_str.to_string_lossy())
}

// Logs a message from C++
// ERROR
#[unsafe(no_mangle)]
pub extern "C" fn cxx_error(payload: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(payload) };
    error!("[CXX] {}", c_str.to_string_lossy())
}
