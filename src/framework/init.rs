/// Configuration options for initializing the Wasm application.
pub struct AppConfig {
    pub name: String,
    pub debug_mode: bool,
    pub max_threads: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "DefaultApp".to_string(),
            debug_mode: false,
            max_threads: 4,
        }
    }
}

/// Initializes the Wasm application with an optional configuration.
///
/// If a configuration is provided, it initializes the application using the
/// provided settings. If not, it defaults to initializing with default settings.
///
/// # Arguments
/// * `config` - An optional `AppConfig` configuration.
///
/// # Example
/// ```
/// let config = AppConfig {
///     name: String::from("CustomApp"),
///     debug_mode: true,
///     max_threads: 8,
/// };
/// init_app(Some(config));
/// ```
pub fn init_app(config: Option<AppConfig>) {
    let app_config = config.unwrap_or_else(AppConfig::default);

    println!("Initializing Wasm application: {}", app_config.name);
    if app_config.debug_mode {
        println!("Debug mode is enabled.");
    } else {
        println!("Debug mode is disabled.");
    }
    println!("Max threads: {}", app_config.max_threads);
}
