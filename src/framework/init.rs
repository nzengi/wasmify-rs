/// Initializes the Wasm application with optional configuration.
///
/// If a configuration is provided, it initializes the application using the
/// provided settings. If not, it defaults to initializing with default settings.
///
/// # Arguments
/// * `config` - An optional configuration string.
pub fn init_app(config: Option<&str>) {
    if let Some(cfg) = config {
        println!("Initializing Wasm application with config: {}", cfg);
    } else {
        println!("Wasm application initialized with default settings.");
    }
}
