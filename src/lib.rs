//! # wasmify-rs
//!
//! `wasmify-rs` is a Rust library for WebAssembly optimization and smart contract monitoring.

/// Smart contract-related modules and functionalities.
pub mod contracts;

/// Framework modules for optimization and asynchronous operations.
pub mod framework;

// Exported functions and modules for external use.
pub use contracts::deploy::deploy_contract;
pub use contracts::abi::parse_abi;
pub use contracts::gas::{estimate_gas, check_gas_limit, optimize_gas_dynamically};
pub use contracts::interaction::{call_contract_function, fetch_contract_data};
pub use contracts::watch::watch_contract_events;
pub use contracts::contract_update::update_contract;
pub use contracts::monitor::monitor_contract_activity;
pub use crate::framework::async_operations::perform_optimized_operations;
pub use framework::logging::{log_info, log_warn, log_error, log_debug};
pub use std::time::{Instant, Duration};
pub use chrono::Local;

use env_logger;
use log::LevelFilter;
use std::io::Write;

/// Initializes the logging configuration for the application.
///
/// This function configures the logger to log information at the `Info` level
/// and above, including timestamps and logging source details.
/// It also allows easy integration with the `env_logger` crate.
pub fn init_logging() {
    env_logger::builder()
        .filter(None, LevelFilter::Info) // Info-level logs will be displayed
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] - {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

/// Main entry point for the application.
/// Initializes logging, monitors contract activity, and performs gas optimizations.
#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();
    
    log_info("Starting the application...");

    // Monitor contract activity
    if let Err(e) = monitor_contract_activity(
        "0x1234567890abcdef1234567890abcdef12345678", 
        "EventName", 
        Duration::from_secs(5),
        3,
    ) {
        log_error(&format!("Monitoring failed: {:?}", e)); // Use log_error for logging errors
    } else {
        log_info("Monitoring successful.");
    }

    // Perform parallel optimized operations
    if let Err(e) = perform_optimized_operations().await {
        log_error(&format!("Operation failed: {}", e));
    }

    log_info("Application completed successfully.");
}
