use std::{
    env::var,
    path::PathBuf,
    str::FromStr,
    sync::{
        Arc, LazyLock, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use chrono::Local;
use flexi_logger::{Duplicate, FileSpec, LogSpecBuilder, Logger, LoggerHandle, style};
use log::LevelFilter;

use crate::{
    SynchronizationError, UnHEICError,
    resource::info::UNHEIC_APP_NAME,
    util::{directories::log_dir, thread::get_thread_name},
};

pub(crate) static LOGGING_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static CURRENT_LOG_FILE: LazyLock<Arc<RwLock<Option<PathBuf>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));
static LOG_BEGIN: LazyLock<Instant> = LazyLock::new(|| Instant::now());

pub(crate) fn init_logging() -> crate::Result<LoggerHandle> {
    // Initialize beginning
    LazyLock::force(&LOG_BEGIN);

    // Retrieve logging level or fallback
    let level = var("UNHEIC_VERBOSITY")
        .map(|level_setting| {
            LevelFilter::from_str(&level_setting)
                .ok()
                .unwrap_or(LevelFilter::Debug)
        })
        .unwrap_or(LevelFilter::Debug);

    let logspec = LogSpecBuilder::new().module("unheic", level).build();

    // Get a logging file
    let current_date = Local::now().to_rfc3339().replace(":", "-");
    let file_name = format!("{}_{}.log", UNHEIC_APP_NAME, current_date);
    let log_dir = log_dir()?;
    let log_file_path = log_dir.join(file_name);

    // Ensure the log dir exists
    if !std::fs::exists(&log_dir).map_err(|err| UnHEICError::IOError(err))? {
        std::fs::create_dir_all(&log_dir).map_err(|err| UnHEICError::IOError(err))?;
    }

    // let log_file = log_file(&log_file_path).map_err(|err| UnHEICError::IOError(err))?;

    let logger_handle = Logger::with(logspec)
        .format_for_stderr(|out, _, record| {
            let thread_name = match get_thread_name() {
                Some(name) => name,
                None => format!("Thread<{:?}>", std::thread::current().id()),
            };
            let duration_since_start = Instant::now().duration_since(*LOG_BEGIN);

            // Pass the final formatting message
            out.write_all(
                format!(
                    "[{:>14}] [ {} ] [ {} ] {}",
                    format!(
                        "{}.{:06}",
                        duration_since_start.as_secs(),
                        duration_since_start.subsec_micros(),
                    ),
                    thread_name,
                    style(record.level())
                        .paint(record.level().as_str().split_at(1).0.to_uppercase()),
                    record.args()
                )
                .as_bytes(),
            )?;
            Ok(())
        })
        .format_for_files(|out, _, record| {
            let thread_name = match get_thread_name() {
                Some(name) => name,
                None => format!("Thread<{:?}>", std::thread::current().id()),
            };
            let duration_since_start = Instant::now().duration_since(*LOG_BEGIN);

            // Pass the final formatting message
            out.write_all(
                format!(
                    "[{:>14}] [ {} ] [ {} ] {}",
                    format!(
                        "{}.{:06}",
                        duration_since_start.as_secs(),
                        duration_since_start.subsec_micros(),
                    ),
                    thread_name,
                    record.level().as_str().split_at(1).0.to_uppercase(),
                    record.args(),
                )
                .as_bytes(),
            )?;
            Ok(())
        })
        .log_to_file(FileSpec::try_from(&log_file_path).unwrap())
        .duplicate_to_stderr(Duplicate::All)
        .start()
        .unwrap();

    // Tell the whole application that a Logger was successfully set
    LOGGING_INITIALIZED.store(true, Ordering::SeqCst);

    // Set the current log file
    *CURRENT_LOG_FILE.write().map_err(|err| {
        UnHEICError::SynchronizationError(SynchronizationError::RwLockPoisoned(format!(
            "{err} ({}:{})",
            file!(),
            line!()
        )))
    })? = Some(log_file_path);

    Ok(logger_handle)
}
