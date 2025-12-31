use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::config::{Config, ConfigError};

/// Error types for file operations
#[derive(Debug, thiserror::Error)]
pub enum FileOperationError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Directory creation failed: {0}")]
    DirectoryCreationFailed(String),
    #[error("File integrity check failed: {0}")]
    IntegrityCheckFailed(String),
    #[error("Move operation failed: {0}")]
    MoveOperationFailed(String),
}

/// Public interface for file operations
pub struct FileOperations;

impl FileOperations {
    /// Move a file from source to the Knowledge Base pages directory
    ///
    /// Handles collision detection and resolution by appending hash postfixes.
    /// Creates destination directories if they don't exist.
    ///
    /// # Arguments
    /// * `source_path` - Path to the source file to move
    /// * `config` - Configuration containing Knowledge Base path
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - Final destination path where file was moved
    /// * `Err(FileOperationError)` - Error if operation failed
    pub fn move_to_pages(
        source_path: &Path,
        config: &Config,
    ) -> Result<PathBuf, FileOperationError> {
        // Validate source file exists
        if !source_path.exists() {
            return Err(FileOperationError::FileNotFound(
                source_path.display().to_string(),
            ));
        }

        // Construct destination directory
        let pages_dir = Self::get_pages_directory(config)?;

        // Ensure destination directory exists
        Self::ensure_directory_exists(&pages_dir)?;

        // Get source filename
        let filename = source_path.file_name().ok_or_else(|| {
            FileOperationError::MoveOperationFailed("Invalid source file path".to_string())
        })?;

        // Resolve destination path with collision handling
        let dest_path = Self::resolve_destination_path(&pages_dir, filename)?;

        // Perform atomic move operation
        Self::atomic_move(source_path, &dest_path)?;

        Ok(dest_path)
    }

    /// Get the pages directory path from config
    ///
    /// Constructs the full path to {{Knowledge Base}}/pages
    fn get_pages_directory(config: &Config) -> Result<PathBuf, FileOperationError> {
        let kb_path = config.get_knowledge_base_path();
        let mut pages_path = PathBuf::from(kb_path);
        pages_path.push("pages");
        Ok(pages_path)
    }

    /// Ensure directory exists, creating it if necessary
    fn ensure_directory_exists(dir_path: &Path) -> Result<(), FileOperationError> {
        if !dir_path.exists() {
            fs::create_dir_all(dir_path).map_err(|e| {
                FileOperationError::DirectoryCreationFailed(format!(
                    "{}: {}",
                    dir_path.display(),
                    e
                ))
            })?;
        }
        Ok(())
    }

    /// Resolve destination path with collision handling
    ///
    /// If a file already exists at the destination, generates a unique filename
    /// by appending a hash postfix derived from the current timestamp.
    fn resolve_destination_path(
        dest_dir: &Path,
        filename: &std::ffi::OsStr,
    ) -> Result<PathBuf, FileOperationError> {
        let mut dest_path = dest_dir.join(filename);

        // If no collision, return original path
        if !dest_path.exists() {
            return Ok(dest_path);
        }

        // Handle collision by generating hash postfix
        let filename_str = filename.to_str().ok_or_else(|| {
            FileOperationError::MoveOperationFailed("Invalid filename encoding".to_string())
        })?;

        // Split filename and extension
        let (name, ext) = if let Some(dot_pos) = filename_str.rfind('.') {
            (&filename_str[..dot_pos], &filename_str[dot_pos..])
        } else {
            (filename_str, "")
        };

        // Generate unique hash postfix based on current timestamp
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .hash(&mut hasher);

        let hash = hasher.finish();
        let hash_postfix = format!("_{:x}", hash);

        // Try with hash postfix, keep generating until we find a unique name
        let mut attempt = 0;
        loop {
            let new_filename = if attempt == 0 {
                format!("{}{}{}", name, hash_postfix, ext)
            } else {
                format!("{}{}_{}{}", name, hash_postfix, attempt, ext)
            };

            dest_path = dest_dir.join(&new_filename);

            if !dest_path.exists() {
                break;
            }

            attempt += 1;

            // Safety check to prevent infinite loop
            if attempt > 1000 {
                return Err(FileOperationError::MoveOperationFailed(
                    "Unable to generate unique filename after 1000 attempts".to_string(),
                ));
            }
        }

        Ok(dest_path)
    }

    /// Perform atomic move operation with basic rollback capability
    ///
    /// Uses copy + delete approach for cross-filesystem moves
    fn atomic_move(source: &Path, destination: &Path) -> Result<(), FileOperationError> {
        // First, try a simple rename (works for same filesystem)
        if let Ok(()) = fs::rename(source, destination) {
            return Ok(());
        }

        // If rename fails (likely cross-filesystem), use copy + delete
        fs::copy(source, destination)?;

        // Verify the copy was successful by checking file exists and size matches
        Self::verify_file_integrity(source, destination)?;

        // Only delete source after successful copy and verification
        fs::remove_file(source).map_err(|e| {
            FileOperationError::MoveOperationFailed(format!(
                "Failed to remove source file after copy: {}",
                e
            ))
        })?;

        Ok(())
    }

    /// Verify file integrity after copy operation
    fn verify_file_integrity(source: &Path, destination: &Path) -> Result<(), FileOperationError> {
        let source_metadata = fs::metadata(source)?;
        let dest_metadata = fs::metadata(destination)?;

        if source_metadata.len() != dest_metadata.len() {
            // Clean up partial copy
            let _ = fs::remove_file(destination);
            return Err(FileOperationError::IntegrityCheckFailed(format!(
                "File size mismatch: source {} bytes, destination {} bytes",
                source_metadata.len(),
                dest_metadata.len()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_config(kb_path: &str) -> Config {
        Config {
            knowledge_base_path: kb_path.to_string(),
        }
    }

    #[test]
    fn test_get_pages_directory() {
        let config = create_test_config("/test/kb");
        let pages_dir = FileOperations::get_pages_directory(&config).unwrap();
        assert_eq!(pages_dir, PathBuf::from("/test/kb/pages"));
    }

    #[test]
    fn test_ensure_directory_exists_creates_directory() {
        let temp_dir = tempdir().unwrap();
        let new_dir_path = temp_dir.path().join("new_directory");

        assert!(!new_dir_path.exists());

        FileOperations::ensure_directory_exists(&new_dir_path).unwrap();

        assert!(new_dir_path.exists());
        assert!(new_dir_path.is_dir());
    }

    #[test]
    fn test_ensure_directory_exists_with_existing_directory() {
        let temp_dir = tempdir().unwrap();

        // Should not fail for existing directory
        FileOperations::ensure_directory_exists(temp_dir.path()).unwrap();
    }

    #[test]
    fn test_resolve_destination_path_no_collision() {
        let temp_dir = tempdir().unwrap();
        let filename = std::ffi::OsStr::new("test.md");

        let dest_path =
            FileOperations::resolve_destination_path(temp_dir.path(), filename).unwrap();

        assert_eq!(dest_path, temp_dir.path().join("test.md"));
    }

    #[test]
    fn test_resolve_destination_path_with_collision() {
        let temp_dir = tempdir().unwrap();
        let filename = std::ffi::OsStr::new("test.md");

        // Create existing file to cause collision
        let existing_file_path = temp_dir.path().join("test.md");
        File::create(&existing_file_path).unwrap();

        let dest_path =
            FileOperations::resolve_destination_path(temp_dir.path(), filename).unwrap();

        // Should generate a different filename with hash postfix
        assert_ne!(dest_path, existing_file_path);
        assert!(dest_path.to_string_lossy().contains("test_"));
        assert!(dest_path.to_string_lossy().ends_with(".md"));
    }

    #[test]
    fn test_resolve_destination_path_multiple_collisions() {
        let temp_dir = tempdir().unwrap();
        let filename = std::ffi::OsStr::new("test.md");

        // Create multiple existing files to cause collisions
        File::create(temp_dir.path().join("test.md")).unwrap();

        // Generate first collision-resolved name
        let first_dest =
            FileOperations::resolve_destination_path(temp_dir.path(), filename).unwrap();
        File::create(&first_dest).unwrap();

        // Generate second collision-resolved name
        let second_dest =
            FileOperations::resolve_destination_path(temp_dir.path(), filename).unwrap();

        // All three should be different
        let original = temp_dir.path().join("test.md");
        assert_ne!(first_dest, original);
        assert_ne!(second_dest, original);
        assert_ne!(second_dest, first_dest);
    }

    #[test]
    fn test_atomic_move_success() {
        let temp_dir = tempdir().unwrap();

        // Create source file
        let source_path = temp_dir.path().join("source.md");
        let mut source_file = File::create(&source_path).unwrap();
        writeln!(source_file, "Test content").unwrap();
        drop(source_file);

        // Create destination path
        let dest_path = temp_dir.path().join("destination.md");

        // Perform move
        FileOperations::atomic_move(&source_path, &dest_path).unwrap();

        // Verify move
        assert!(!source_path.exists());
        assert!(dest_path.exists());

        // Verify content
        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content, "Test content\n");
    }

    #[test]
    fn test_verify_file_integrity_success() {
        let temp_dir = tempdir().unwrap();

        // Create two identical files
        let file1_path = temp_dir.path().join("file1.md");
        let file2_path = temp_dir.path().join("file2.md");

        let content = "Test content for integrity check";
        fs::write(&file1_path, content).unwrap();
        fs::write(&file2_path, content).unwrap();

        // Verification should pass
        FileOperations::verify_file_integrity(&file1_path, &file2_path).unwrap();
    }

    #[test]
    fn test_verify_file_integrity_failure() {
        let temp_dir = tempdir().unwrap();

        // Create two files with different sizes
        let file1_path = temp_dir.path().join("file1.md");
        let file2_path = temp_dir.path().join("file2.md");

        fs::write(&file1_path, "Short content").unwrap();
        fs::write(
            &file2_path,
            "Much longer content that should fail integrity check",
        )
        .unwrap();

        // Verification should fail
        let result = FileOperations::verify_file_integrity(&file1_path, &file2_path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FileOperationError::IntegrityCheckFailed(_)
        ));
    }

    #[test]
    fn test_move_to_pages_full_integration() {
        let temp_dir = tempdir().unwrap();

        // Create config pointing to temp directory
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Create source file
        let source_path = temp_dir.path().join("test_article.md");
        fs::write(&source_path, "# Test Article\n\nThis is a test article.").unwrap();

        // Perform move to pages
        let dest_path = FileOperations::move_to_pages(&source_path, &config).unwrap();

        // Verify results
        assert!(!source_path.exists()); // Source should be gone
        assert!(dest_path.exists()); // Destination should exist

        // Check destination is in pages directory
        let expected_pages_dir = temp_dir.path().join("pages");
        assert!(dest_path.starts_with(&expected_pages_dir));

        // Verify content
        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content, "# Test Article\n\nThis is a test article.");
    }

    #[test]
    fn test_move_to_pages_creates_pages_directory() {
        let temp_dir = tempdir().unwrap();

        // Config points to temp directory but pages subdirectory doesn't exist yet
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Create source file
        let source_path = temp_dir.path().join("test.md");
        fs::write(&source_path, "test content").unwrap();

        // Pages directory should not exist initially
        let pages_dir = temp_dir.path().join("pages");
        assert!(!pages_dir.exists());

        // Perform move
        FileOperations::move_to_pages(&source_path, &config).unwrap();

        // Pages directory should now exist
        assert!(pages_dir.exists());
        assert!(pages_dir.is_dir());
    }

    #[test]
    fn test_move_to_pages_file_not_found() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Try to move non-existent file
        let non_existent_path = temp_dir.path().join("does_not_exist.md");
        let result = FileOperations::move_to_pages(&non_existent_path, &config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FileOperationError::FileNotFound(_)
        ));
    }

    #[test]
    fn test_move_to_pages_with_collision() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config(&temp_dir.path().display().to_string());

        // Create pages directory and existing file
        let pages_dir = temp_dir.path().join("pages");
        fs::create_dir_all(&pages_dir).unwrap();
        let existing_file = pages_dir.join("article.md");
        fs::write(&existing_file, "existing content").unwrap();

        // Create source file with same name
        let source_path = temp_dir.path().join("article.md");
        fs::write(&source_path, "new content").unwrap();

        // Perform move
        let dest_path = FileOperations::move_to_pages(&source_path, &config).unwrap();

        // Should create new file with hash postfix
        assert_ne!(dest_path, existing_file);
        assert!(dest_path.to_string_lossy().contains("article_"));
        assert!(dest_path.to_string_lossy().ends_with(".md"));

        // Both files should exist with different content
        assert_eq!(
            fs::read_to_string(&existing_file).unwrap(),
            "existing content"
        );
        assert_eq!(fs::read_to_string(&dest_path).unwrap(), "new content");
    }
}
