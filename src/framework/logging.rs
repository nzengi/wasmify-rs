use log::{info, warn, error, debug};
use env_logger;
use chrono::Local;
use std::io::Write;

/// Initializes the logging configuration for the application.
/// This function configures the logger to log information at the `Info` level and above,
/// and includes timestamps and logging source details.
/// It also supports structured logging with customizable format and log levels.
pub fn init_logging() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info) // Set the minimum log level to Info
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}:{}] - {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
                record.file().unwrap_or("unknown"),       // File name
                record.line().unwrap_or(0),               // Line number
                record.level(),                           // Log level (Info, Warn, etc.)
                record.args()                             // Log message
            )
        })
        .init();
}

/// Logs an info-level message.
/// 
/// # Arguments
/// * `message` - The message to log at the info level.
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Logs a warning-level message.
/// 
/// # Arguments
/// * `message` - The message to log at the warning level.
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Logs an error-level message.
/// 
/// # Arguments
/// * `message` - The message to log at the error level.
pub fn log_error(message: &str) {
    error!("{}", message);
}

/// Logs a debug-level message.
/// 
/// # Arguments
/// * `message` - The message to log at the debug level.
pub fn log_debug(message: &str) {
    debug!("{}", message);
}
