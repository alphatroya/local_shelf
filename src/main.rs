pub mod config;
pub mod file_discovery;
pub mod file_operations;

use config::{Config, ConfigError};
use file_discovery::{FileDiscovery, FileDiscoveryError};
use file_operations::{FileOperationError, FileOperations};

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("File discovery error: {0}")]
    FileDiscovery(#[from] FileDiscoveryError),
    #[error("File operation error: {0}")]
    FileOperation(#[from] FileOperationError),
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
        return Ok(());
    }

    println!(
        "Found {} markdown file(s) in ~/Downloads:",
        markdown_files.len()
    );
    for file in &markdown_files {
        println!("  - {}", file.display());
    }

    // Move files to pages directory
    println!("\nMoving files to {{Knowledge Base}}/pages...");
    let mut moved_files = Vec::new();

    for file_path in &markdown_files {
        match FileOperations::move_to_pages(file_path, &config) {
            Ok(destination) => {
                println!(
                    "✓ Moved {} → {}",
                    file_path.file_name().unwrap().to_string_lossy(),
                    destination.display()
                );
                moved_files.push(destination);
            }
            Err(e) => {
                eprintln!("✗ Failed to move {}: {}", file_path.display(), e);
            }
        }
    }

    if moved_files.is_empty() {
        println!("No files were successfully moved.");
    } else {
        println!(
            "\nSuccessfully moved {} file(s) to pages directory.",
            moved_files.len()
        );
    }

    Ok(())
}
