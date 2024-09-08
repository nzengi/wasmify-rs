use crate::framework::logging::log_info;
use web3::types::U256;

/// Optimizes the gas usage for a smart contract based on historical gas prices.
/// 
/// # Arguments
/// * `historical_gas_prices` - A list of historical gas prices.
/// * `current_gas_limit` - The current gas limit for the contract.
///
/// # Returns
/// U256 - The optimized gas limit.
pub fn optimize_gas_usage(historical_gas_prices: Vec<U256>, current_gas_limit: U256) -> U256 {
    let average_gas_price = if !historical_gas_prices.is_empty() {
        let sum: U256 = historical_gas_prices.iter().sum();
        sum / U256::from(historical_gas_prices.len())
    } else {
        U256::from(0)
    };

    let adjustment_factor = if average_gas_price > U256::from(100) {
        U256::from(90) // Reduce gas usage if average gas price is high
    } else {
        U256::from(110) // Increase gas usage if average gas price is low
    };

    let optimized_gas_limit = current_gas_limit * adjustment_factor / U256::from(100);
    log_info(&format!("Optimized gas limit: {:?}", optimized_gas_limit)); // Corrected log usage

    optimized_gas_limit
}

// Unit test example
#[cfg(test)]
mod tests {
    use super::*;
    use web3::types::U256;

    #[test]
    fn test_gas_optimization_high_prices() {
        let historical_gas_prices = vec![U256::from(120), U256::from(130)];
        let optimized_gas = optimize_gas_usage(historical_gas_prices, U256::from(10000));
        assert!(optimized_gas < U256::from(10000));
    }

    #[test]
    fn test_gas_optimization_low_prices() {
        let historical_gas_prices = vec![U256::from(80), U256::from(90)];
        let optimized_gas = optimize_gas_usage(historical_gas_prices, U256::from(10000));
        assert!(optimized_gas > U256::from(10000));
    }
}
