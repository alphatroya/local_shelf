pub mod config;
pub mod file_discovery;

use config::{Config, ConfigError};
use file_discovery::{FileDiscovery, FileDiscoveryError};

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("File discovery error: {0}")]
    FileDiscovery(#[from] FileDiscoveryError),
}

fn main() -> Result<(), AppError> {
    // Initialize configuration on first run
    Config::initialize()?;

    // Load configuration
    let config = Config::load()?;

    println!("Local Shelf starting...");
    println!("Knowledge Base path: {}", config.get_knowledge_base_path());

    // Discover markdown files in Downloads
    println!("Scanning ~/Downloads for markdown files...");
    let markdown_files = FileDiscovery::discover_markdown_files()?;

    if markdown_files.is_empty() {
        println!("No markdown files found in ~/Downloads");
    } else {
        println!(
            "Found {} markdown file(s) in ~/Downloads:",
            markdown_files.len()
        );
        for file in &markdown_files {
            println!("  - {}", file.display());
        }
    }

    Ok(())
}
