/// Represents a gas estimate for contract deployment.
#[derive(Debug)]
pub struct GasEstimate {
    pub estimated_gas: u64,
    pub max_gas: u64,
}

/// Estimates the gas required to deploy a contract based on its size and complexity.
///
/// # Arguments
///
/// * `contract_size` - The size of the contract in bytes.
/// * `complexity` - The complexity factor of the contract.
///
/// # Returns
/// GasEstimate - Contains both the estimated and maximum gas values.
pub fn estimate_gas(contract_size: usize, complexity: u64) -> GasEstimate {
    let base_gas = 21_000;
    let size_gas = contract_size as u64 * 10;  // 10 gas per byte
    let complexity_gas = complexity * 50;     // 50 gas per complexity unit

    let estimated_gas = base_gas + size_gas + complexity_gas;
    let max_gas = estimated_gas * 2;

    GasEstimate {
        estimated_gas,
        max_gas,
    }
}

/// Checks if the provided gas limit is sufficient for contract deployment.
///
/// # Arguments
///
/// * `gas_limit` - The gas limit provided.
/// * `estimate` - A GasEstimate struct with the estimated gas.
///
/// # Returns
/// Result<(), String> - Returns Ok if the gas limit is sufficient, otherwise returns an error.
pub fn check_gas_limit(gas_limit: u64, estimate: &GasEstimate) -> Result<(), String> {
    if gas_limit < estimate.estimated_gas {
        return Err(format!(
            "Insufficient gas limit: {} provided, but {} estimated.",
            gas_limit, estimate.estimated_gas
        ));
    }
    Ok(())
}

/// Dynamically optimizes the gas for a contract based on its size and complexity.
///
/// # Arguments
///
/// * `contract_size` - The size of the contract in bytes.
/// * `complexity` - The complexity factor of the contract.
///
/// # Returns
/// u64 - The dynamically optimized gas limit.
pub fn optimize_gas_dynamically(contract_size: usize, complexity: u64) -> u64 {
    let estimate = estimate_gas(contract_size, complexity);
    let gas_limit = (estimate.estimated_gas as f64 * 1.2) as u64;  // Adds a 20% buffer
    gas_limit
}
