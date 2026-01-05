
Yes, you can read configuration data at runtime and make it globally accessible
to all modules in your Rust application.

The idiomatic approach in safe, multi-threaded Rust involves using thread-safe
lazy initialization mechanisms.

The modern, standard library solution (since Rust 1.70) is to use

std::sync::OnceLock<T> or std::sync::LazyLock<T>

(since Rust 1.80) to initialize your configuration struct once and then provide immutable,
global access to it.

Recommended Approach: std::sync::LazyLock

The LazyLock type provides the cleanest and safest way to achieve this.
It ensures that the initialization code runs only once, the first time
the variable is accessed, and is thread-safe.

1. Define your Configuration Struct
First, define a struct to hold your configuration variables.
It's best practice to use types like String that can be dynamically populated at runtime

```rust
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
```

2. Use LazyLock for Global Access
Declare a pub static variable using LazyLock (or OnceLock in older Rust versions)
in your configuration module.
Note: LazyLock is stable as of Rust 1.80.
If you are using an older version, the lazy_static crate provides similar functionality via a macro.

```rust
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
```

3. Access the Configuration in Any Module

Now, any module in your application can access the configuration by importing
APP_CONFIG and treating it like a normal reference, without needing unsafe blocks
or explicit locking mechanism

```rust
// In another_module.rs
use crate::config::APP_CONFIG;

pub fn print_config() {
    // Access the config from a different module
    println!("Another module sees Log Level: {}", APP_CONFIG.log_level);
}
```

Alternatives and Best Practices

   Avoid static mut: 
   Directly using static mut is possible but highly discouraged in safe Rust as it can lead to 
   data races and undefined behavior.
   
   Pass as parameters: 
   The most idiomatic Rust approach is to avoid global state entirely and pass a reference or a 
   smart pointer (like Arc<Config>) to functions or structs that need it. 
   The lazy global method serves as a convenient alternative when passing parameters becomes cumbersome.

   Use the config crate: 
   For complex configuration management (layered files, environment variables, etc.), 
   consider using the popular config crate
