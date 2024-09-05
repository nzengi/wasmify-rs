use serde::Deserialize;

/// Struct representing a function from an ABI (Application Binary Interface).
/// 
/// # Fields
/// - `name`: The name of the function.
/// - `inputs`: A list of input parameter types for the function.
/// - `outputs`: A list of output parameter types for the function.
/// - `payable`: Indicates whether the function accepts Ether.
/// - `constant`: Indicates whether the function is constant (i.e., does not change state).
#[derive(Debug, Deserialize)]
pub struct AbiFunction {
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub payable: bool,
    pub constant: bool,
}

/// Parses the given ABI JSON string and returns a vector of `AbiFunction`.
/// 
/// # Arguments
/// * `abi_json`: A string slice representing the ABI in JSON format.
/// 
/// # Returns
/// `Result<Vec<AbiFunction>, String>` - A result containing either a vector of parsed functions or an error message.
/// 
/// # Errors
/// - If the provided JSON is empty, it returns an error.
/// - If the JSON is invalid, it returns a detailed parsing error.
pub fn parse_abi(abi_json: &str) -> Result<Vec<AbiFunction>, String> {
    if abi_json.is_empty() {
        return Err("ABI JSON cannot be empty.".to_string());
    }

    // Attempt to parse the ABI JSON string into a vector of `AbiFunction` structs.
    let parsed: Vec<AbiFunction> = serde_json::from_str(abi_json)
        .map_err(|e| format!("Failed to parse ABI: {}", e))?;

    Ok(parsed)
}

