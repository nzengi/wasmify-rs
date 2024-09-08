use crate::framework::logging::{log_info, log_warn, log_error};
use std::time::{Instant, Duration};
use std::thread;

/// Errors that can occur during contract monitoring.
#[derive(Debug)]
pub enum MonitorError {
    InvalidContractAddress,
    MonitoringFailed,
    TransientError, // For retry mechanism
}

/// Monitors contract activity, logs the status, and periodically checks for events.
/// Implements retry mechanism in case of transient errors.
///
/// # Arguments
/// * `contract_address` - The address of the contract to be monitored.
/// * `event_name` - The event to monitor.
/// * `poll_interval` - How often to check for contract events.
/// * `retries` - Number of retries in case of failure.
///
/// # Returns
/// Result<(), MonitorError> - Returns Ok if monitoring succeeds, otherwise returns an error.
pub fn monitor_contract_activity(
    contract_address: &str,
    event_name: &str,
    poll_interval: Duration,
    retries: u8,
) -> Result<(), MonitorError> {
    // Input validation: ensure address and event name are not empty
    if contract_address.is_empty() || event_name.is_empty() {
        return Err(MonitorError::InvalidContractAddress);
    }

    log_info(&format!("Monitoring started for contract: {}", contract_address));
    let start_time = Instant::now();  // Start timestamp

    let mut attempts = 0;
    loop {
        attempts += 1;
        log_info(&format!("Attempt {}: Polling contract for event '{}'", attempts, event_name));  // Corrected logging
        thread::sleep(poll_interval);

        // Simulate checking for new events (real logic would go here)
        let error_happens = false;
        let transient_error = false;

        if error_happens {
            log_warn(&format!("Warning: An issue occurred while monitoring contract: {}", contract_address));  // Corrected logging
            return Err(MonitorError::MonitoringFailed);
        } else if transient_error {
            log_error("Transient error occurred. Retrying...");

            if attempts >= retries {
                return Err(MonitorError::TransientError);
            }

            continue; // Retry if we haven't exhausted attempts
        } else {
            log_info("No new events detected.");
        }

        if attempts >= 5 {
            break;
        }
    }

    let elapsed_time = start_time.elapsed();
    log_info(&format!(
        "Monitoring completed successfully for contract: {} in {:?} seconds.",
        contract_address, elapsed_time
    ));
    Ok(())
}

// Unit test example
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_invalid_address() {
        let result = monitor_contract_activity("", "TestEvent", Duration::from_secs(5), 3);
        assert!(matches!(result, Err(MonitorError::InvalidContractAddress)));
    }

    #[test]
    fn test_successful_monitoring() {
        let result = monitor_contract_activity("0x123", "TestEvent", Duration::from_secs(5), 3);
        assert!(matches!(result, Ok(())));
    }
}
