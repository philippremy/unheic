use std::sync::atomic::Ordering;

use log::{SetLoggerError, error};
use thiserror::Error;

use crate::util::logging::LOGGING_INITIALIZED;

pub(crate) type Result<T> = std::result::Result<T, UnHEICError>;

pub(crate) trait IntoCExitStatus {
    fn into_c_exit_status(self) -> i32;
}

impl IntoCExitStatus for Result<()> {
    #[inline(always)]
    fn into_c_exit_status(self) -> i32 {
        match self {
            Ok(_) => 0i32,
            Err(err) => {
                // We have to use regular stdout if the Logger is not yet
                // initialized
                if LOGGING_INITIALIZED.load(Ordering::SeqCst) {
                    error!("Returning to C in an error state: {err}.");
                } else {
                    eprintln!("Returning to C in an error state: {err}.");
                }
                err.discriminant()
            }
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Error)]
#[repr(i32)]
pub(crate) enum UnHEICError {
    #[error("An error in a utility function occured: {0}")]
    UtilityError(UtilityError) = 1 << 0,
    #[error("A general I/O error occured while running the application: {0}")]
    IOError(#[from] std::io::Error) = 1 << 1,
    #[error("An error occured while setting the global logger: {0}")]
    LoggingError(#[from] SetLoggerError) = 1 << 2,
    #[error("A synchronization error occured: {0}")]
    SynchronizationError(SynchronizationError) = 1 << 3,
    #[error("A gpui error occured: {0}")]
    GPUIError(GPUIError) = 1 << 4,
    #[error("An error occured while parsing the app theme: {0}")]
    ThemeParsingError(String) = 1 << 5,
    #[error("An error occured during thumbnail generation: {0}")]
    ThumbnailGenerationError(ThumbnailError)
}

impl UnHEICError {
    #[inline(always)]
    fn discriminant(&self) -> i32 {
        // SAFETY: Because `Self` is marked `repr(i32)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `i32` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const Self>::from(self).cast::<i32>() }
    }
}

#[rustfmt::skip]
#[derive(Debug, Error)]
pub(crate) enum UtilityError {
    #[error("Unable to retrieve base directories on this system. This means that the App is unable to find a valid home directory.")]
    HomeDirectoryNotFound
}

#[rustfmt::skip]
#[derive(Debug, Error)]
pub(crate) enum SynchronizationError {
    #[error("RwLock is poisoned: {0}")]
    RwLockPoisoned(String)
}

#[rustfmt::skip]
#[derive(Debug, Error)]
pub(crate) enum GPUIError {
    #[error("Unable to open a gpui window: {0}")]
    OpenWindowFailed(String)
}

#[rustfmt::skip]
#[derive(Debug, Error)]
pub(crate) enum ConversionError {
}

#[rustfmt::skip]
#[derive(Debug, Error)]
pub(crate) enum ThumbnailError {
    #[error("The RwLock for the thumbnail is poisoned: {0}")]
    RwLockPoisoned(String)
}
