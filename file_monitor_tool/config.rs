use std::env;

pub struct Config {
    pub monitor_dir: String,
}

pub fn load_config() -> Config {
    // Fetch dir to monitor from environment variables or default
    let monitor_dir = env::var("MONITOR_DIR").unwrap_or_else(|_| String::from("/var/log"));
    
    Config {
        monitor_dir,
    }
}
