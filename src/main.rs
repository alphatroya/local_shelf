pub mod config;
pub mod file_discovery;
pub mod file_operations;
pub mod journal_management;

use clap::{Parser, Subcommand};
use config::{Config, ConfigError};
use file_discovery::FileDiscoveryError;
use file_operations::{FileOperationError, FileOperations};
use journal_management::{JournalError, JournalManager};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("File discovery error: {0}")]
    FileDiscovery(#[from] FileDiscoveryError),
    #[error("File operation error: {0}")]
    FileOperation(#[from] FileOperationError),
    #[error("Journal error: {0}")]
    Journal(#[from] JournalError),
    #[error("Conversion error: {0}")]
    Conversion(String),
}

#[derive(Parser)]
#[command(
    name = "local_shelf",
    version = "0.2.0",
    about = "A tool for organizing and converting markdown files",
    author = "Local Shelf Contributors"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Move markdown files from a directory to Knowledge Base pages directory
    #[command(name = "stow")]
    Stow {
        /// Directory containing markdown files to move (defaults to current directory)
        #[arg(help = "Path to directory containing markdown files")]
        path: Option<PathBuf>,
    },
    /// Convert markdown files in a directory to EPUB format
    #[command(name = "convert")]
    Convert {
        /// Directory containing markdown files to convert (defaults to current directory)
        #[arg(help = "Path to directory containing markdown files")]
        path: Option<PathBuf>,
    },
}

fn check_pandoc() -> Result<(), AppError> {
    match Command::new("pandoc").arg("--version").output() {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::Conversion(
            "pandoc is not installed or not in PATH. Please install pandoc first: https://pandoc.org/installing.html".to_string(),
        )),
    }
}

fn discover_markdown_files_in_directory(directory: &PathBuf) -> Result<Vec<PathBuf>, AppError> {
    if !directory.exists() || !directory.is_dir() {
        return Err(AppError::FileDiscovery(FileDiscoveryError::IoError(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory '{}' does not exist", directory.display()),
            ),
        )));
    }

    let mut markdown_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Some(extension) = path.extension()
                && extension == "md"
            {
                markdown_files.push(path);
            }
        }
    }

    Ok(markdown_files)
}

fn convert_markdown_to_epub(md_file: &PathBuf) -> Result<PathBuf, AppError> {
    let epub_file = md_file.with_extension("epub");

    let output = Command::new("pandoc")
        .arg(md_file)
        .arg("-o")
        .arg(&epub_file)
        .arg("--from")
        .arg("markdown")
        .arg("--to")
        .arg("epub")
        .output()
        .map_err(|e| AppError::Conversion(format!("Failed to run pandoc: {}", e)))?;

    if output.status.success() {
        Ok(epub_file)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(AppError::Conversion(format!(
            "Pandoc failed for {}: {}",
            md_file.display(),
            error_msg
        )))
    }
}

fn handle_convert_command(path: Option<PathBuf>) -> Result<(), AppError> {
    // Use current directory if no path provided
    let target_directory = path.unwrap_or_else(|| PathBuf::from("."));

    // Check if pandoc is installed
    check_pandoc()?;

    println!(
        "Converting markdown files in: {}",
        target_directory.display()
    );
    println!("================================");

    // Discover markdown files in target directory
    let markdown_files = discover_markdown_files_in_directory(&target_directory)?;

    if markdown_files.is_empty() {
        println!("No markdown files found in {}", target_directory.display());
        return Ok(());
    }

    let mut converted = 0;
    let mut skipped = 0;

    for md_file in &markdown_files {
        println!(
            "Converting: {} -> {}",
            md_file.display(),
            md_file.with_extension("epub").display()
        );

        match convert_markdown_to_epub(md_file) {
            Ok(epub_file) => {
                println!(
                    "✓ Successfully converted: {}",
                    epub_file.file_name().unwrap().to_string_lossy()
                );
                converted += 1;
            }
            Err(e) => {
                eprintln!(
                    "✗ Failed to convert {}: {}",
                    md_file.file_name().unwrap().to_string_lossy(),
                    e
                );
                skipped += 1;
            }
        }
        println!();
    }

    println!("================================");
    println!("Conversion complete!");
    println!("Files converted: {}", converted);
    println!("Files skipped: {}", skipped);

    Ok(())
}

fn handle_stow_command(path: Option<PathBuf>) -> Result<(), AppError> {
    // Initialize configuration on first run
    Config::initialize()?;

    // Load configuration
    let config = Config::load()?;

    // Use specified directory or default to current directory
    let target_directory = path.unwrap_or_else(|| PathBuf::from("."));

    println!("Local Shelf starting...");
    println!("Knowledge Base path: {}", config.get_knowledge_base_path());

    // Discover markdown files in specified directory
    println!(
        "Scanning {} for markdown files...",
        target_directory.display()
    );
    let markdown_files = discover_markdown_files_in_directory(&target_directory)?;

    if markdown_files.is_empty() {
        println!("No markdown files found in {}", target_directory.display());
        return Ok(());
    }

    println!(
        "Found {} markdown file(s) in {}:",
        markdown_files.len(),
        target_directory.display()
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
        return Ok(());
    }

    println!(
        "\nSuccessfully moved {} file(s) to pages directory.",
        moved_files.len()
    );

    // Add journal entries for moved files
    println!("Creating journal entries...");
    match JournalManager::add_entries(&moved_files, &config) {
        Ok(journal_path) => {
            println!(
                "✓ Added {} journal entr{} to {}",
                moved_files.len(),
                if moved_files.len() == 1 { "y" } else { "ies" },
                journal_path.display()
            );
        }
        Err(e) => {
            eprintln!("✗ Failed to create journal entries: {}", e);
        }
    }

    Ok(())
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Stow { path } => handle_stow_command(path),
        Commands::Convert { path } => handle_convert_command(path),
    }
}
