// Module declarations
pub mod deploy;
pub mod interaction;
pub mod contract_update;
pub mod gas;
pub mod abi;
pub mod watch;
pub mod monitor;

// Public exports to make modules and functions available externally

/// Smart contract deployment
pub use deploy::deploy_contract;

/// Smart contract interaction functions
pub use interaction::{call_contract_function, fetch_contract_data};

/// Smart contract updates
pub use contract_update::update_contract;

/// Gas management and optimization functions
pub use gas::{estimate_gas, check_gas_limit, optimize_gas_dynamically};

/// ABI parsing functions
pub use abi::parse_abi;

/// Contract event watching
pub use watch::watch_contract_transactions;

/// Contract monitoring with logging and time tracking
pub use monitor::monitor_contract_activity;
