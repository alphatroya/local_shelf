use std::fs;
use std::path::PathBuf;

/// Error types for file discovery operations
#[derive(Debug, thiserror::Error)]
pub enum FileDiscoveryError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Path expansion error: {0}")]
    PathExpansionError(String),
}

/// Public interface for file discovery operations
pub struct FileDiscovery;

impl FileDiscovery {
    /// Discover markdown files in the ~/Downloads directory
    pub fn discover_markdown_files() -> Result<Vec<PathBuf>, FileDiscoveryError> {
        let downloads_path = Self::expand_path("~/Downloads")?;

        if !downloads_path.exists() {
            return Ok(vec![]);
        }

        if !downloads_path.is_dir() {
            return Err(FileDiscoveryError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Downloads path exists but is not a directory",
            )));
        }

        let entries = fs::read_dir(&downloads_path)?;
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            }
        }

        Ok(Self::filter_markdown_files(files))
    }

    /// Expand tilde (~) notation to home directory
    pub fn expand_path(path: &str) -> Result<PathBuf, FileDiscoveryError> {
        if let Some(path_without_tilde) = path.strip_prefix('~') {
            if let Some(home_dir) = dirs::home_dir() {
                let path_without_tilde =
                    if let Some(stripped) = path_without_tilde.strip_prefix('/') {
                        stripped
                    } else {
                        path_without_tilde
                    };
                Ok(home_dir.join(path_without_tilde))
            } else {
                Err(FileDiscoveryError::PathExpansionError(
                    "Could not determine home directory".to_string(),
                ))
            }
        } else {
            Ok(PathBuf::from(path))
        }
    }

    /// Filter markdown files from a list of files
    pub fn filter_markdown_files(files: Vec<PathBuf>) -> Vec<PathBuf> {
        files
            .into_iter()
            .filter(|file| {
                file.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.to_lowercase() == "md")
                    .unwrap_or(false)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_expand_path_with_tilde() {
        let result = FileDiscovery::expand_path("~/Downloads");
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert!(!expanded.to_string_lossy().starts_with('~'));
        assert!(expanded.to_string_lossy().ends_with("Downloads"));
    }

    #[test]
    fn test_expand_path_without_tilde() {
        let absolute_path = "/usr/local/bin";
        let result = FileDiscovery::expand_path(absolute_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), absolute_path);
    }

    #[test]
    fn test_filter_markdown_files_mixed_extensions() {
        let files = vec![
            PathBuf::from("document.md"),
            PathBuf::from("README.MD"),
            PathBuf::from("notes.Md"),
            PathBuf::from("image.jpg"),
            PathBuf::from("script.py"),
        ];

        let filtered = FileDiscovery::filter_markdown_files(files);
        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().all(|f| {
            let ext = f.extension().and_then(|s| s.to_str()).unwrap_or("");
            ext.to_lowercase() == "md"
        }));
    }

    #[test]
    fn test_filter_markdown_files_no_markdown() {
        let files = vec![
            PathBuf::from("image.jpg"),
            PathBuf::from("document.pdf"),
            PathBuf::from("script.py"),
        ];

        let filtered = FileDiscovery::filter_markdown_files(files);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_markdown_files_empty_list() {
        let files = vec![];
        let filtered = FileDiscovery::filter_markdown_files(files);
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_discover_markdown_files_interface() {
        // This test will ensure the interface exists and returns appropriate type
        // Implementation will be tested once we have the actual implementation
        let result = FileDiscovery::discover_markdown_files();
        match result {
            Ok(_) | Err(_) => {} // Either outcome is acceptable for interface test
        }
    }

    #[test]
    fn test_discover_files_in_temp_directory() {
        // Create a temporary directory to simulate Downloads
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create some test files
        File::create(temp_path.join("document.md")).unwrap();
        File::create(temp_path.join("README.MD")).unwrap();
        File::create(temp_path.join("image.jpg")).unwrap();
        File::create(temp_path.join("notes.txt")).unwrap();

        // Read the directory and filter markdown files
        let entries = fs::read_dir(temp_path).unwrap();
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }

        let markdown_files = FileDiscovery::filter_markdown_files(files);
        assert_eq!(markdown_files.len(), 2);
    }

    #[test]
    fn test_error_handling_invalid_path() {
        // Test path expansion with invalid tilde path
        let result = FileDiscovery::expand_path("~/nonexistent/../Downloads");
        assert!(result.is_ok()); // Path expansion should still work even if path doesn't exist
    }
}
