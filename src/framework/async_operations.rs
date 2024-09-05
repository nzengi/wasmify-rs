use tokio::task;
use crate::contracts::gas::{estimate_gas, optimize_gas_dynamically};

/// Performs optimized gas operations asynchronously and in parallel.
///
/// This function spawns two asynchronous tasks:
/// 1. Estimates the gas for a contract.
/// 2. Optimizes the gas dynamically for another contract.
///
/// The tasks are executed concurrently to improve performance.
pub async fn perform_optimized_operations() {
    let handle1 = task::spawn(async { estimate_gas(100, 5) });
    let handle2 = task::spawn(async { optimize_gas_dynamically(200, 8) });

    let (result1, result2) = tokio::join!(handle1, handle2);

    match (result1, result2) {
        (Ok(gas_estimate), Ok(optimized_gas)) => {
            log::info!("Gas estimate: {:?}", gas_estimate);
            log::info!("Optimized gas: {:?}", optimized_gas);
        }
        _ => {
            log::error!("Error occurred during parallel gas operations.");
        }
    }
}
