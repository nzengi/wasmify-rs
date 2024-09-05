// Import necessary functions from the library.
use wasmify_rs::contracts::gas::{estimate_gas, check_gas_limit, optimize_gas_dynamically};
use wasmify_rs::framework::async_operations::perform_optimized_operations;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test to verify gas estimation based on contract size and complexity.
    #[test]
    fn test_estimate_gas() {
        let contract_size = 100;  // 100 bytes contract
        let complexity = 5;       // Complexity factor 5

        let gas_estimate = estimate_gas(contract_size, complexity);

        // Base Gas: 21_000, Size Gas: 100 * 10 = 1_000, Complexity Gas: 5 * 50 = 250
        // Expected Estimated Gas: 21_000 + 1_000 + 250 = 22_250
        assert_eq!(gas_estimate.estimated_gas, 22_250);
        // Maximum gas: estimated_gas * 2 = 22_250 * 2 = 44_500
        assert_eq!(gas_estimate.max_gas, 44_500);
    }

    /// Test to verify if a sufficient gas limit passes the check.
    #[test]
    fn test_check_gas_limit_sufficient() {
        let contract_size = 100;
        let complexity = 5;

        let gas_estimate = estimate_gas(contract_size, complexity);

        // Providing 30,000 gas should be enough, so the result should be OK
        let result = check_gas_limit(30_000, &gas_estimate);
        assert!(result.is_ok());
    }

    /// Test to ensure insufficient gas limit triggers an error.
    #[test]
    fn test_check_gas_limit_insufficient() {
        let contract_size = 100;
        let complexity = 5;

        let gas_estimate = estimate_gas(contract_size, complexity);

        // Providing 20,000 gas should not be enough, so it should trigger an error
        let result = check_gas_limit(20_000, &gas_estimate);
        assert!(result.is_err());
    }

    /// Test to check if dynamic gas optimization adds a safety margin.
    #[test]
    fn test_optimize_gas_dynamically() {
        let contract_size = 100;
        let complexity = 5;

        // Optimized gas should be 22,250 * 1.2 = 26,700
        let optimized_gas = optimize_gas_dynamically(contract_size, complexity);
        assert_eq!(optimized_gas, 26_700);
    }

    /// Asynchronous tests for parallel gas operations.
    #[cfg(test)]
    mod async_tests {
        use super::*;
        use tokio::runtime::Runtime;

        /// Test for parallel execution of gas estimation and optimization.
        #[test]
        fn test_perform_optimized_operations() {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                perform_optimized_operations().await;
            });
        }
    }
}
