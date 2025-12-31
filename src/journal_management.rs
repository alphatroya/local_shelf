use chrono::{Local, NaiveDate};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::{Config, ConfigError};

/// Error types for journal operations
#[derive(Debug, thiserror::Error)]
pub enum JournalError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("Directory creation failed: {0}")]
    DirectoryCreationFailed(String),
    #[error("Journal entry formatting error: {0}")]
    EntryFormattingError(String),
    #[error("Journal write operation failed: {0}")]
    WriteOperationFailed(String),
}

/// Represents a journal entry with timestamp and file link
#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntry {
    pub timestamp: String, // HH:mm format
    pub filename: String,  // filename without extension
}

impl JournalEntry {
    /// Create a new journal entry with current timestamp
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to link in the journal entry
    ///
    /// # Returns
    /// * `Ok(JournalEntry)` - New entry with current timestamp
    /// * `Err(JournalError)` - Error if filename extraction fails
    pub fn new(file_path: &Path) -> Result<Self, JournalError> {
        // Extract filename without extension
        let filename = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| {
                JournalError::EntryFormattingError(format!(
                    "Invalid filename: {}",
                    file_path.display()
                ))
            })?;

        // Generate timestamp in HH:mm format
        let now = Local::now();
        let timestamp = now.format("%H:%M").to_string();

        Ok(JournalEntry {
            timestamp,
            filename: filename.to_string(),
        })
    }

    /// Format the journal entry as markdown
    ///
    /// Returns the entry in the format: `- **HH:mm** [[Name of the file]]`
    pub fn format(&self) -> String {
        format!("- **{}** [[{}]]", self.timestamp, self.filename)
    }
}

/// Public interface for journal management operations
pub struct JournalManager;

impl JournalManager {
    /// Add journal entries for successfully moved files
    ///
    /// Creates or appends to today's journal file with timestamped entries
    /// linking to the moved files.
    ///
    /// # Arguments
    /// * `moved_files` - Vector of paths to files that were moved
    /// * `config` - Configuration containing Knowledge Base path
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - Path to the journal file that was updated
    /// * `Err(JournalError)` - Error if operation failed
    pub fn add_entries(moved_files: &[PathBuf], config: &Config) -> Result<PathBuf, JournalError> {
        if moved_files.is_empty() {
            return Err(JournalError::EntryFormattingError(
                "No files provided for journal entries".to_string(),
            ));
        }

        // Get journal file path for today
        let journal_path = Self::get_today_journal_path(config)?;

        // Ensure journals directory exists
        if let Some(parent) = journal_path.parent() {
            Self::ensure_directory_exists(parent)?;
        }

        // Create journal entries
        let entries: Result<Vec<_>, _> = moved_files
            .iter()
            .map(|path| JournalEntry::new(path))
            .collect();
        let entries = entries?;

        // Write entries to journal file
        Self::append_entries_to_journal(&journal_path, &entries)?;

        Ok(journal_path)
    }

    /// Get the path to today's journal file
    ///
    /// Constructs path in format: {{Knowledge Base}}/journals/YYYY_MM_DD.md
    fn get_today_journal_path(config: &Config) -> Result<PathBuf, JournalError> {
        let journals_dir = Self::get_journals_directory(config)?;

        // Format today's date as YYYY_MM_DD
        let today = Local::now().date_naive();
        let date_str = today.format("%Y_%m_%d").to_string();
        let filename = format!("{}.md", date_str);

        Ok(journals_dir.join(filename))
    }

    /// Get the journals directory path from config
    ///
    /// Constructs the full path to {{Knowledge Base}}/journals
    fn get_journals_directory(config: &Config) -> Result<PathBuf, JournalError> {
        let kb_path = config.get_knowledge_base_path();
        let mut journals_path = PathBuf::from(kb_path);
        journals_path.push("journals");
        Ok(journals_path)
    }

    /// Ensure directory exists, creating it if necessary
    fn ensure_directory_exists(dir_path: &Path) -> Result<(), JournalError> {
        if !dir_path.exists() {
            fs::create_dir_all(dir_path).map_err(|e| {
                JournalError::DirectoryCreationFailed(format!("{}: {}", dir_path.display(), e))
            })?;
        }
        Ok(())
    }

    /// Append journal entries to the specified journal file
    ///
    /// Creates the file if it doesn't exist, or appends to existing file.
    /// Entries in the same batch are written consecutively without blank lines.
    /// Uses atomic operations to prevent corruption.
    fn append_entries_to_journal(
        journal_path: &Path,
        entries: &[JournalEntry],
    ) -> Result<(), JournalError> {
        // Format all entries as strings
        let entry_lines: Vec<String> = entries.iter().map(|entry| entry.format()).collect();

        // Create the content to append
        let mut content = String::new();

        // If file exists and has content, check if we need separation
        if journal_path.exists() && fs::metadata(journal_path)?.len() > 0 {
            // Read the last byte to check if file ends with newline
            let existing_content = fs::read_to_string(journal_path)?;
            if !existing_content.ends_with('\n') {
                content.push('\n');
            }
        }

        // Join all entries with newlines (no blank lines between entries in same batch)
        content.push_str(&entry_lines.join("\n"));

        // Ensure content ends with a newline
        content.push('\n');

        // Atomic append operation
        Self::atomic_append(journal_path, &content)?;

        Ok(())
    }

    /// Perform atomic append operation to avoid corruption
    ///
    /// Uses OpenOptions to append safely to the file
    fn atomic_append(file_path: &Path, content: &str) -> Result<(), JournalError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .map_err(|e| {
                JournalError::WriteOperationFailed(format!(
                    "Failed to open journal file {}: {}",
                    file_path.display(),
                    e
                ))
            })?;

        file.write_all(content.as_bytes()).map_err(|e| {
            JournalError::WriteOperationFailed(format!(
                "Failed to write to journal file {}: {}",
                file_path.display(),
                e
            ))
        })?;

        file.flush().map_err(|e| {
            JournalError::WriteOperationFailed(format!(
                "Failed to flush journal file {}: {}",
                file_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Parse date from journal filename (for testing and validation)
    ///
    /// # Arguments
    /// * `filename` - Journal filename in YYYY_MM_DD.md format
    ///
    /// # Returns
    /// * `Ok(NaiveDate)` - Parsed date
    /// * `Err(JournalError)` - Error if format is invalid
    pub fn parse_journal_date(filename: &str) -> Result<NaiveDate, JournalError> {
        let stem = filename.strip_suffix(".md").ok_or_else(|| {
            JournalError::EntryFormattingError(format!(
                "Journal filename must end with .md: {}",
                filename
            ))
        })?;

        NaiveDate::parse_from_str(stem, "%Y_%m_%d").map_err(|e| {
            JournalError::EntryFormattingError(format!(
                "Invalid journal date format '{}': {}",
                stem, e
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_config(kb_path: &str) -> Config {
        Config {
            knowledge_base_path: kb_path.to_string(),
        }
    }

    #[test]
    fn test_journal_entry_creation() {
        let file_path = PathBuf::from("test_article.md");
        let entry = JournalEntry::new(&file_path).unwrap();

        assert_eq!(entry.filename, "test_article");
        assert!(entry.timestamp.len() == 5); // HH:MM format
        assert!(entry.timestamp.contains(':'));
    }

    #[test]
    fn test_journal_entry_formatting() {
        let entry = JournalEntry {
            timestamp: "14:30".to_string(),
            filename: "my_article".to_string(),
        };

        let formatted = entry.format();
        assert_eq!(formatted, "- **14:30** [[my_article]]");
    }

    #[test]
    fn test_journal_entry_with_complex_filename() {
        let file_path = PathBuf::from("Complex File Name-With_Special.Characters.md");
        let entry = JournalEntry::new(&file_path).unwrap();

        assert_eq!(entry.filename, "Complex File Name-With_Special.Characters");
    }

    #[test]
    fn test_get_journals_directory() {
        let config = create_test_config("/test/kb");
        let journals_dir = JournalManager::get_journals_directory(&config).unwrap();
        assert_eq!(journals_dir, PathBuf::from("/test/kb/journals"));
    }

    #[test]
    fn test_get_today_journal_path() {
        let config = create_test_config("/test/kb");
        let journal_path = JournalManager::get_today_journal_path(&config).unwrap();

        let today = Local::now().date_naive().format("%Y_%m_%d").to_string();
        let expected_filename = format!("{}.md", today);

        assert_eq!(
            journal_path.file_name().unwrap().to_str().unwrap(),
            expected_filename
        );
        assert!(
            journal_path
                .to_string_lossy()
                .contains("/test/kb/journals/")
        );
    }

    #[test]
    fn test_ensure_directory_exists() {
        let temp_dir = tempdir().unwrap();
        let new_dir = temp_dir.path().join("journals");

        assert!(!new_dir.exists());

        JournalManager::ensure_directory_exists(&new_dir).unwrap();

        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn test_parse_journal_date() {
        let valid_filename = "2024_03_15.md";
        let date = JournalManager::parse_journal_date(valid_filename).unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    }

    #[test]
    fn test_parse_journal_date_invalid_format() {
        let invalid_filename = "2024-03-15.md";
        let result = JournalManager::parse_journal_date(invalid_filename);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_journal_date_no_extension() {
        let invalid_filename = "2024_03_15";
        let result = JournalManager::parse_journal_date(invalid_filename);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_entries_to_new_journal() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        let moved_files = vec![PathBuf::from("article1.md"), PathBuf::from("article2.md")];

        let journal_path = JournalManager::add_entries(&moved_files, &config).unwrap();

        // Verify journal file was created
        assert!(journal_path.exists());

        // Verify content
        let content = fs::read_to_string(&journal_path).unwrap();
        assert!(content.contains("[[article1]]"));
        assert!(content.contains("[[article2]]"));
        assert!(content.matches("- **").count() == 2); // Two entries
    }

    #[test]
    fn test_add_entries_to_existing_journal() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Create journals directory and existing journal file
        let journals_dir = temp_dir.path().join("journals");
        fs::create_dir_all(&journals_dir).unwrap();

        let today = Local::now().date_naive().format("%Y_%m_%d").to_string();
        let journal_file = journals_dir.join(format!("{}.md", today));
        fs::write(&journal_file, "# Existing content\n").unwrap();

        // Add new entries
        let moved_files = vec![PathBuf::from("new_article.md")];
        let journal_path = JournalManager::add_entries(&moved_files, &config).unwrap();

        // Verify content was appended
        let content = fs::read_to_string(&journal_path).unwrap();
        assert!(content.contains("# Existing content"));
        assert!(content.contains("[[new_article]]"));
    }

    #[test]
    fn test_add_entries_empty_files_list() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        let moved_files: Vec<PathBuf> = vec![];
        let result = JournalManager::add_entries(&moved_files, &config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JournalError::EntryFormattingError(_)
        ));
    }

    #[test]
    fn test_atomic_append_creates_file() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test_journal.md");

        let content = "- **14:30** [[test_file]]\n";
        JournalManager::atomic_append(&test_file, content).unwrap();

        assert!(test_file.exists());
        let file_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(file_content, content);
    }

    #[test]
    fn test_atomic_append_to_existing_file() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test_journal.md");

        // Create initial content
        fs::write(&test_file, "Initial content\n").unwrap();

        // Append new content
        let new_content = "- **15:45** [[new_entry]]\n";
        JournalManager::atomic_append(&test_file, new_content).unwrap();

        let final_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(
            final_content,
            "Initial content\n- **15:45** [[new_entry]]\n"
        );
    }

    #[test]
    fn test_journal_entries_batch_processing() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Create multiple files with different names
        let moved_files = vec![
            PathBuf::from("first_article.md"),
            PathBuf::from("second_article.md"),
            PathBuf::from("third_article.md"),
        ];

        let journal_path = JournalManager::add_entries(&moved_files, &config).unwrap();
        let content = fs::read_to_string(&journal_path).unwrap();

        // Verify all files are linked
        assert!(content.contains("[[first_article]]"));
        assert!(content.contains("[[second_article]]"));
        assert!(content.contains("[[third_article]]"));

        // Verify proper formatting
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3);
        for line in lines {
            assert!(line.starts_with("- **"));
            assert!(line.contains("]]"));
        }
    }

    #[test]
    fn test_consecutive_app_runs_spacing() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Simulate first app run
        let first_files = vec![PathBuf::from("first_file.md")];
        let journal_path = JournalManager::add_entries(&first_files, &config).unwrap();

        // Simulate second app run (different batch)
        let second_files = vec![PathBuf::from("second_file.md")];
        JournalManager::add_entries(&second_files, &config).unwrap();

        // Check the actual content
        let content = fs::read_to_string(&journal_path).unwrap();
        println!("Journal content:\n'{}'", content);

        let lines: Vec<&str> = content.lines().collect();
        println!("Number of lines: {}", lines.len());
        for (i, line) in lines.iter().enumerate() {
            println!("Line {}: '{}'", i, line);
        }

        // This should be 2 lines (2 entries), not 3 (with blank line)
        assert_eq!(
            lines.len(),
            2,
            "Expected 2 entries without blank line, got {} lines",
            lines.len()
        );
    }

    #[test]
    fn test_append_to_file_without_ending_newline() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Create journal file manually without ending newline
        let journals_dir = temp_dir.path().join("journals");
        fs::create_dir_all(&journals_dir).unwrap();

        let today = Local::now().date_naive().format("%Y_%m_%d").to_string();
        let journal_file = journals_dir.join(format!("{}.md", today));

        // Write content WITHOUT trailing newline
        fs::write(&journal_file, "- **15:00** [[existing_entry]]").unwrap();

        // Add new entry
        let new_files = vec![PathBuf::from("new_file.md")];
        JournalManager::add_entries(&new_files, &config).unwrap();

        let content = fs::read_to_string(&journal_file).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        // Should have 2 lines (existing + new), properly separated
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("existing_entry"));
        assert!(lines[1].contains("new_file"));
    }
}
