use std::{
    ffi::{CStr, c_char},
    panic::panic_any,
};

// This function enables C++ to initiate a Rust panic when handling an
// unhandled exception
#[unsafe(no_mangle)]
pub unsafe extern "C" fn transfer_exception_to_rust(what: *const c_char) -> ! {
    let cstr = unsafe { CStr::from_ptr(what) };
    panic_any(format!("{}", cstr.to_str().unwrap()))
}
