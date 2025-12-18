// In a module like `config.rs`
#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub log_level: String,
}

impl Config {
    // A method to load configuration from file/environment variables
    fn load() -> Self {
        // Use the `config` crate or environment variables here
        // For this example, we'll use environment variables via std::env
        let api_key = std::env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string());
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "WARN".to_string());

        Config { api_key, log_level }
    }
}

use std::sync::LazyLock;

// This static variable can be accessed from anywhere in your application
pub static APP_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    // The code inside this closure runs only once, on the first access.
    Config::load()
});
// In main.rs or another module
mod config;

fn main() {
    // Access the config (initialization happens here, if not already done)
    println!("API Key: {}", config::APP_CONFIG.api_key);
    println!("Log Level: {}", config::APP_CONFIG.log_level);

    // Call another function in a different module
    another_module::print_config();
}

// In another_module.rs
use crate::config::APP_CONFIG;

pub fn print_config() {
    // Access the config from a different module
    println!("Another module sees Log Level: {}", APP_CONFIG.log_level);
}
