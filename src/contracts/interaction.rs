/// Represents errors that may occur during contract interactions.
#[derive(Debug)]
pub enum InteractionError {
    /// The provided contract address is invalid or empty.
    InvalidContractAddress,
    /// The function name provided is invalid or empty.
    InvalidFunctionName,
    /// The requested data key was not found.
    DataKeyNotFound,
}

/// Calls a function on a smart contract.
///
/// # Arguments
///
/// * `contract_address` - The address of the contract.
/// * `function_name` - The name of the function to call.
/// * `_params` - Parameters to pass to the function (currently unused).
/// * `gas_limit` - The gas limit for the function call.
///
/// # Returns
/// Result<String, InteractionError> - Returns a success message or an error if the function call fails.
///
/// # Example
/// ```
/// let result = call_contract_function("0x123...", "transfer", vec![], 21000);
/// assert!(result.is_ok());
/// ```
pub fn call_contract_function(
    contract_address: &str,
    function_name: &str,
    _params: Vec<String>,
    gas_limit: u64,
) -> Result<String, InteractionError> {
    if contract_address.is_empty() {
        return Err(InteractionError::InvalidContractAddress);
    }

    if function_name.is_empty() {
        return Err(InteractionError::InvalidFunctionName);
    }

    println!(
        "Calling function '{}' on contract '{}' with gas limit {}...",
        function_name, contract_address, gas_limit
    );

    Ok("Function call executed successfully.".to_string())
}

/// Fetches data from a smart contract.
///
/// # Arguments
///
/// * `contract_address` - The address of the contract.
/// * `data_key` - The key of the data to fetch.
///
/// # Returns
/// Result<String, InteractionError> - Returns the data or an error if the fetch fails.
///
/// # Example
/// ```
/// let result = fetch_contract_data("0x123...", "balance");
/// assert!(result.is_ok());
/// ```
pub fn fetch_contract_data(
    contract_address: &str,
    data_key: &str,
) -> Result<String, InteractionError> {
    if contract_address.is_empty() {
        return Err(InteractionError::InvalidContractAddress);
    }

    if data_key.is_empty() {
        return Err(InteractionError::DataKeyNotFound);
    }

    println!("Fetching data '{}' from contract '{}'...", data_key, contract_address);

    Ok("Data fetched successfully.".to_string())
}
