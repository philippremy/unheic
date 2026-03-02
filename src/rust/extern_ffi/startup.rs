use std::{
    ffi::{c_char, c_int},
    panic::catch_unwind,
};

use crate::{IntoCExitStatus, start_unheic_internal};

// We want to export an entry function which can be called from C
// Catch any unwinding panics
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_unheic(_argc: c_int, _argv: *const *const c_char) -> c_int {
    match catch_unwind(|| {
        // Converts a Result into an exit code
        start_unheic_internal().into_c_exit_status()
    }) {
        Ok(exit_status) => exit_status,
        Err(_) => -1,
    }
}
