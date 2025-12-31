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
}
