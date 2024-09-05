use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

/// Initializes logging configuration for the application.
///
/// This function sets up a logging system that allows logging at various levels:
/// * Error
/// * Warn
/// * Info
/// * Debug
/// * Trace
///
/// The logs are directed to standard output (stdout) and formatted to include timestamps,
/// log levels, and the message content. The default log level is set to `Info`, but this
/// can be overridden using the `RUST_LOG` environment variable.
///
/// # Example
///
/// ```
/// init_logging();
/// log::info!("This is an info log message.");
/// ```
///
/// This setup helps with debugging and monitoring in both development and production environments.
pub fn init_logging() {
    let mut builder = Builder::new();

    // Set the logging level to Info by default. This can be overridden via the RUST_LOG environment variable.
    builder.filter(None, LevelFilter::Info);

    // Direct logs to stdout.
    builder.target(Target::Stdout);

    // Customize the log format to include timestamps, log level, and the message.
    builder.format(|buf, record| {
        writeln!(
            buf,
            "[{} {}] - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    });

    // Initialize the logger.
    builder.init();
}
