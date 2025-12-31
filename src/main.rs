pub mod config;

use config::{Config, ConfigError};

fn main() -> Result<(), ConfigError> {
    // Initialize configuration on first run
    Config::initialize()?;

    // Load configuration
    let config = Config::load()?;

    println!("Local Shelf starting...");
    println!("Knowledge Base path: {}", config.get_knowledge_base_path());

    Ok(())
}
