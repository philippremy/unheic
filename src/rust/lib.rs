/// cbindgen:ignore
mod backend;

/// cbindgen:ignore
mod bridge;

/// EXPOSED!
mod extern_ffi;

/// cbindgen:ignore
mod resource;

/// cbindgen:ignore
mod ui;

/// cbindgen:ignore
mod util;

// Re-export the util types for error handling
/// cbindgen:ignore
pub(crate) use util::error::*;

fn start_unheic_internal() -> Result<()> {
    // Set main thread name
    util::thread::set_thread_name("Main Thread");

    // Initialize logging
    // Must be kept until the end of the program
    let _log_handle = util::logging::init_logging()?;
    log::debug!("Logging initialized.");

    // Initialize panic handler
    util::panic::register_panic_handler()?;
    log::debug!("Panic Handler initialized.");

    // Log application info
    util::info::log_app_info();

    // Start the application
    ui::app::start_app()?;
    log::debug!("Returned from start_app().");

    Ok(())
}
