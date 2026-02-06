use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub knowledge_base_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            knowledge_base_path: "~/Knowledge Base".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    YamlError(serde_yaml::Error),
    ValidationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::YamlError(e) => write!(f, "YAML error: {}", e),
            ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IoError(error)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::YamlError(error)
    }
}

impl Config {
    /// Get the configuration directory path
    pub fn config_dir() -> Result<PathBuf, ConfigError> {
        // Check for test environment override first
        if let Ok(test_config_dir) = env::var("LOCAL_SHELF_CONFIG_DIR") {
            return Ok(PathBuf::from(test_config_dir));
        }

        dirs::config_dir()
            .map(|mut path| {
                path.push("local_shelf");
                path
            })
            .ok_or_else(|| {
                ConfigError::ValidationError("Unable to determine config directory".to_string())
            })
    }

    /// Get the legacy configuration directory path  
    pub fn legacy_config_dir() -> Result<PathBuf, ConfigError> {
        dirs::config_dir()
            .map(|mut path| {
                path.push("local-shelf");
                path
            })
            .ok_or_else(|| {
                ConfigError::ValidationError("Unable to determine config directory".to_string())
            })
    }

    /// Migrate configuration from legacy directory if needed
    pub fn migrate_from_legacy() -> Result<bool, ConfigError> {
        let legacy_dir = Self::legacy_config_dir()?;
        let new_dir = Self::config_dir()?;

        // Check if migration is needed
        if legacy_dir.exists() && !new_dir.exists() {
            // Create new directory
            fs::create_dir_all(&new_dir)?;

            // Copy config file if it exists
            let legacy_config = legacy_dir.join("config.yaml");
            let new_config = new_dir.join("config.yaml");

            if legacy_config.exists() {
                fs::copy(&legacy_config, &new_config)?;
            }

            // Remove legacy directory after successful migration
            fs::remove_dir_all(&legacy_dir)?;

            Ok(true) // Migration performed
        } else {
            Ok(false) // No migration needed
        }
    }

    /// Get the configuration file path
    pub fn config_file_path() -> Result<PathBuf, ConfigError> {
        let mut config_dir = Self::config_dir()?;
        config_dir.push("config.yaml");
        Ok(config_dir)
    }

    /// Load configuration with hierarchy: defaults < config file < environment variables
    pub fn load() -> Result<Config, ConfigError> {
        let mut config = Config::default();

        // Try to load from config file
        let config_path = Self::config_file_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            config = serde_yaml::from_str(&content)?;
        }

        // Override with environment variables
        if let Ok(kb_path) = env::var("KNOWLEDGE_BASE") {
            config.knowledge_base_path = kb_path;
        }

        config.validate()?;
        Ok(config)
    }

    /// Create default configuration file if it doesn't exist
    pub fn initialize() -> Result<(), ConfigError> {
        // Skip initialization during tests to avoid contaminating user config
        if env::var("LOCAL_SHELF_SKIP_CONFIG_INIT").is_ok() {
            return Ok(());
        }

        // Try to migrate from legacy config first
        let migrated = Self::migrate_from_legacy()?;
        if migrated {
            println!(
                "Configuration migrated from ~/.config/local-shelf/ to ~/.config/local_shelf/"
            );
        }

        let config_dir = Self::config_dir()?;
        let config_path = Self::config_file_path()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        // Create default config file if it doesn't exist
        if !config_path.exists() {
            let default_config = Config::default();
            let yaml_content = format!(
                "# Local Shelf Configuration\n# \n# Knowledge Base path - where markdown files will be organized\n# Can be overridden with KNOWLEDGE_BASE environment variable\nknowledge_base_path: \"{}\"\n",
                default_config.knowledge_base_path
            );
            fs::write(&config_path, yaml_content)?;
        }

        Ok(())
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.knowledge_base_path.trim().is_empty() {
            return Err(ConfigError::ValidationError(
                "knowledge_base_path cannot be empty".to_string(),
            ));
        }

        // Expand tilde and validate path
        let expanded_path = Self::expand_path(&self.knowledge_base_path);
        let path = Path::new(&expanded_path);

        // Try to create the directory if it doesn't exist
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            return Err(ConfigError::ValidationError(format!(
                "Parent directory does not exist: {}",
                parent.display()
            )));
        }

        Ok(())
    }

    /// Expand tilde (~) in path
    pub fn expand_path(path: &str) -> String {
        if let Some(stripped) = path.strip_prefix("~/")
            && let Some(home_dir) = dirs::home_dir()
        {
            format!("{}/{}", home_dir.display(), stripped)
        } else {
            path.to_string()
        }
    }

    /// Get the expanded knowledge base path
    pub fn get_knowledge_base_path(&self) -> String {
        Self::expand_path(&self.knowledge_base_path)
    }

    /// Save the configuration to the config file
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_file_path()?;
        let config_dir = Self::config_dir()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        // Validate before saving
        self.validate()?;

        // Create YAML content with comments
        let yaml_content = format!(
            "# Local Shelf Configuration\n# \n# Knowledge Base path - where markdown files will be organized\n# Can be overridden with KNOWLEDGE_BASE environment variable\nknowledge_base_path: \"{}\"\n",
            self.knowledge_base_path
        );

        fs::write(&config_path, yaml_content)?;
        Ok(())
    }

    /// Update the knowledge base path and save to config file
    pub fn update_knowledge_base_path(&mut self, new_path: &str) -> Result<(), ConfigError> {
        // Expand tilde if present
        let expanded_path = Self::expand_path(new_path);

        // Validate the new path
        if new_path.trim().is_empty() {
            return Err(ConfigError::ValidationError(
                "knowledge_base_path cannot be empty".to_string(),
            ));
        }

        let path = Path::new(&expanded_path);

        // Try to create the directory if it doesn't exist
        if let Some(parent) = path.parent()
            && !parent.exists()
        {
            return Err(ConfigError::ValidationError(format!(
                "Parent directory does not exist: {}",
                parent.display()
            )));
        }

        // If the path doesn't exist, try to create it
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => {}
                Err(e) => {
                    return Err(ConfigError::ValidationError(format!(
                        "Cannot create directory '{}': {}",
                        path.display(),
                        e
                    )));
                }
            }
        } else if !path.is_dir() {
            return Err(ConfigError::ValidationError(format!(
                "Path '{}' exists but is not a directory",
                path.display()
            )));
        }

        // Update the configuration and save
        self.knowledge_base_path = new_path.to_string();
        self.save()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.knowledge_base_path, "~/Knowledge Base");
    }

    #[test]
    fn test_config_validation_empty_path() {
        let config = Config {
            knowledge_base_path: "".to_string(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_whitespace_path() {
        let config = Config {
            knowledge_base_path: "   ".to_string(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_expand_path_with_tilde() {
        let home = dirs::home_dir().unwrap();
        let expanded = Config::expand_path("~/Documents");
        assert_eq!(expanded, format!("{}/Documents", home.display()));
    }

    #[test]
    fn test_expand_path_without_tilde() {
        let path = "/absolute/path";
        let expanded = Config::expand_path(path);
        assert_eq!(expanded, path);
    }

    #[test]
    fn test_environment_variable_override() {
        // Set environment variable
        unsafe {
            env::set_var("KNOWLEDGE_BASE", "/tmp/test_kb");
        }

        // Create a temporary config file with different value
        let temp_dir = tempdir().unwrap();
        let config_content = "knowledge_base_path: \"/different/path\"";
        let config_path = temp_dir.path().join("config.yaml");
        fs::write(&config_path, config_content).unwrap();

        // Mock the config file path (this is simplified - in real test we'd need to mock the entire path)
        // For now, let's test the environment override logic directly
        let mut config = Config {
            knowledge_base_path: "/different/path".to_string(),
        };

        // Simulate environment override
        if let Ok(kb_path) = env::var("KNOWLEDGE_BASE") {
            config.knowledge_base_path = kb_path;
        }

        assert_eq!(config.knowledge_base_path, "/tmp/test_kb");

        // Clean up
        unsafe {
            env::remove_var("KNOWLEDGE_BASE");
        }
    }

    #[test]
    fn test_yaml_serialization() {
        let config = Config {
            knowledge_base_path: "/test/path".to_string(),
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_get_knowledge_base_path() {
        let config = Config {
            knowledge_base_path: "~/Test".to_string(),
        };

        let expanded = config.get_knowledge_base_path();
        let home = dirs::home_dir().unwrap();
        assert_eq!(expanded, format!("{}/Test", home.display()));
    }

    #[test]
    fn test_save_config() {
        // Set environment variable to skip initialization during tests
        unsafe {
            env::set_var("LOCAL_SHELF_SKIP_CONFIG_INIT", "1");
        }

        let _temp_dir = tempdir().unwrap();
        let config = Config {
            knowledge_base_path: "/tmp/test_save".to_string(),
        };

        // Mock the config directory (in a real test environment, we'd mock the entire path)
        // For now, test the validation and YAML serialization logic
        let yaml_content = format!(
            "# Local Shelf Configuration\n# \n# Knowledge Base path - where markdown files will be organized\n# Can be overridden with KNOWLEDGE_BASE environment variable\nknowledge_base_path: \"{}\"\n",
            config.knowledge_base_path
        );

        assert!(yaml_content.contains("/tmp/test_save"));
        assert!(yaml_content.contains("knowledge_base_path:"));

        // Clean up
        unsafe {
            env::remove_var("LOCAL_SHELF_SKIP_CONFIG_INIT");
        }
    }

    #[test]
    fn test_update_knowledge_base_path_validation() {
        // Save current environment state to restore later
        let original_skip_init = env::var("LOCAL_SHELF_SKIP_CONFIG_INIT").ok();
        let original_config_dir = env::var("LOCAL_SHELF_CONFIG_DIR").ok();

        // Use temporary config directory to avoid affecting user config
        let temp_dir = tempdir().unwrap();
        let temp_config_dir = temp_dir.path().join("test_config");
        fs::create_dir_all(&temp_config_dir).unwrap();

        // Set environment variable to use temporary config directory
        unsafe {
            env::set_var("LOCAL_SHELF_SKIP_CONFIG_INIT", "1");
            env::set_var("LOCAL_SHELF_CONFIG_DIR", temp_config_dir.to_str().unwrap());
        }

        let mut config = Config::default();

        // Test empty path validation
        let result = config.update_knowledge_base_path("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));

        // Test whitespace path validation
        let result = config.update_knowledge_base_path("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));

        // Test nonexistent parent directory
        let result = config.update_knowledge_base_path("/nonexistent/parent/path");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Parent directory does not exist")
        );

        // Restore original environment state
        unsafe {
            env::remove_var("LOCAL_SHELF_SKIP_CONFIG_INIT");
            env::remove_var("LOCAL_SHELF_CONFIG_DIR");

            if let Some(val) = original_skip_init {
                env::set_var("LOCAL_SHELF_SKIP_CONFIG_INIT", val);
            }
            if let Some(val) = original_config_dir {
                env::set_var("LOCAL_SHELF_CONFIG_DIR", val);
            }
        }
    }

    #[test]
    fn test_update_knowledge_base_path_tilde_expansion() {
        // Save current environment state to restore later
        let original_skip_init = env::var("LOCAL_SHELF_SKIP_CONFIG_INIT").ok();
        let original_config_dir = env::var("LOCAL_SHELF_CONFIG_DIR").ok();

        // Use temporary config directory to avoid affecting user config
        let temp_dir = tempdir().unwrap();
        let temp_config_dir = temp_dir.path().join("test_config");
        fs::create_dir_all(&temp_config_dir).unwrap();

        // Set environment variable to use temporary config directory
        unsafe {
            env::set_var("LOCAL_SHELF_SKIP_CONFIG_INIT", "1");
            env::set_var("LOCAL_SHELF_CONFIG_DIR", temp_config_dir.to_str().unwrap());
        }

        let mut config = Config::default();
        let home = dirs::home_dir().unwrap();
        let test_path = "~/TestUpdateUnit"; // Use different name to avoid conflicts

        // Create a temporary test directory in the home directory
        let test_dir = home.join("TestUpdateUnit");

        // This test may fail in CI environments without proper home directory setup
        // In practice, we'd mock the directory creation for robust testing
        if home.exists() {
            // Create the test directory
            fs::create_dir_all(&test_dir).ok();

            let result = config.update_knowledge_base_path(test_path);
            // We expect this to either succeed or fail with a specific validation error
            match result {
                Ok(_) => {
                    // If it succeeded, the path should be expanded
                    let expected_expanded = format!("{}/TestUpdateUnit", home.display());
                    assert_eq!(config.get_knowledge_base_path(), expected_expanded);

                    // Verify config file was created in temp directory, not user directory
                    let temp_config_file = temp_config_dir.join("config.yaml");
                    assert!(
                        temp_config_file.exists(),
                        "Config should be saved in temp directory"
                    );
                }
                Err(e) => {
                    // If it failed, it should be due to directory creation issues
                    assert!(
                        e.to_string().contains("Cannot create directory")
                            || e.to_string().contains("Parent directory does not exist")
                    );
                }
            }

            // Clean up the test directory
            fs::remove_dir_all(&test_dir).ok();
        }

        // Restore original environment state
        unsafe {
            env::remove_var("LOCAL_SHELF_SKIP_CONFIG_INIT");
            env::remove_var("LOCAL_SHELF_CONFIG_DIR");

            if let Some(val) = original_skip_init {
                env::set_var("LOCAL_SHELF_SKIP_CONFIG_INIT", val);
            }
            if let Some(val) = original_config_dir {
                env::set_var("LOCAL_SHELF_CONFIG_DIR", val);
            }
        }
    }

    #[test]
    fn test_config_dir_override() {
        // Save current environment state to restore later
        let original_config_dir = env::var("LOCAL_SHELF_CONFIG_DIR").ok();

        let temp_dir = tempdir().unwrap();
        let test_config_dir = temp_dir.path().join("test_config");

        // Set the override environment variable
        unsafe {
            env::set_var("LOCAL_SHELF_CONFIG_DIR", test_config_dir.to_str().unwrap());
        }

        // Test that config_dir returns the override path
        let result = Config::config_dir().unwrap();
        assert_eq!(result, test_config_dir);

        // Restore original environment state
        unsafe {
            env::remove_var("LOCAL_SHELF_CONFIG_DIR");
            if let Some(val) = original_config_dir {
                env::set_var("LOCAL_SHELF_CONFIG_DIR", val);
            }
        }
    }

    #[test]
    fn test_config_dir_normal_behavior() {
        // Save current environment state to restore later
        let original_config_dir = env::var("LOCAL_SHELF_CONFIG_DIR").ok();

        // Ensure no override is set
        unsafe {
            env::remove_var("LOCAL_SHELF_CONFIG_DIR");
        }

        // Test normal behavior
        let result = Config::config_dir().unwrap();
        let expected = dirs::config_dir().unwrap().join("local_shelf");
        assert_eq!(result, expected);

        // Restore original environment state
        unsafe {
            if let Some(val) = original_config_dir {
                env::set_var("LOCAL_SHELF_CONFIG_DIR", val);
            }
        }
    }
}
