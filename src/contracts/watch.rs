/// Errors that can occur during contract transaction watching.
#[derive(Debug)]
pub enum WatchError {
    InvalidContractAddress,
    MonitoringFailed,
}

/// Watches for transactions related to a specific smart contract.
///
/// # Arguments
/// * `contract_address` - The address of the contract to watch (ownership is moved to the function).
/// * `poll_interval` - The time interval (in seconds) to poll for transactions.
/// * `callback` - A function to call when a transaction is detected.
///
/// # Returns
/// A result indicating whether the watching process was successful or an error occurred.
pub fn watch_contract_transactions<F>(
    contract_address: String,  // Ownership of contract_address is moved.
    poll_interval: std::time::Duration,
    callback: F,
) -> Result<(), WatchError>
where
    F: Fn(&str) + Send + 'static,
{
    if contract_address.is_empty() {
        return Err(WatchError::InvalidContractAddress);
    }

    println!("Starting to watch contract at address {}...", contract_address);

    // Simulating continuous polling in a new thread.
    std::thread::spawn(move || {
        for _ in 0..5 {
            std::thread::sleep(poll_interval);
            let simulated_transaction = format!("Transaction detected for contract: {}", contract_address);
            callback(&simulated_transaction);
        }
        println!("Stopped watching contract at address {}.", contract_address);
    });

    Ok(())
}
