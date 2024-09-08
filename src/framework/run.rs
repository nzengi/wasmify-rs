use tokio;
use crate::framework::logging::{log_info, log_error};
use std::time::Duration;

/// Main entry point for the application.
/// Initializes logging, monitors contract activity, and performs gas optimizations.
#[tokio::main]
async fn main() {
    // Initialize logging
    crate::framework::logging::init_logging();
    
    log_info("Starting the application...");

    // Monitor contract activity
    if let Err(e) = crate::contracts::monitor::monitor_contract_activity(
        "0x1234567890abcdef1234567890abcdef12345678", 
        "EventName", 
        Duration::from_secs(5),
        3,
    ) {
        log_error(&format!("Monitoring failed: {:?}", e));  // Corrected log usage
    } else {
        log_info("Monitoring successful.");  // Corrected log usage
    }

    // Perform optimized asynchronous operations
    if let Err(e) = crate::framework::async_operations::perform_optimized_operations().await {
        log_error(&format!("Operation failed: {}", e));  // Corrected log usage
    }

    log_info("Application completed successfully.");
}
