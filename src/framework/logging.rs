use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;
use chrono::Local;

/// Initializes the logging configuration for the application.
///
/// This function configures the logger to log information at the `Info` level
/// and above, including timestamps and logging source details.
pub fn init_logging() {
    Builder::new()
        .target(Target::Stdout)  // Logs will be sent to stdout
        .filter_level(LevelFilter::Info)  // Logs of Info level and above will be displayed
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] - {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();  // Initialize the logger
}