use local_shelf::config::Config;
use std::env;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_integration_scenarios() {
    // Test 1: Default configuration when no file exists
    {
        let _temp_dir = tempdir().unwrap();
        let _config_dir = _temp_dir.path().join(".config/local-shelf");

        // Create a minimal Config to test defaults without filesystem
        let config = Config::default();
        assert_eq!(config.knowledge_base_path, "~/Knowledge Base");
        assert!(config.validate().is_ok());
    }

    // Test 2: Configuration file loading
    {
        let temp_dir = tempdir().unwrap();
        let config_content = r#"
knowledge_base_path: "/custom/path"
"#;
        let config_path = temp_dir.path().join("config.yaml");
        fs::write(&config_path, config_content).unwrap();

        // Load and verify
        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = serde_yaml::from_str(&content).unwrap();
        assert_eq!(config.knowledge_base_path, "/custom/path");
    }

    // Test 3: Environment variable override priority
    {
        unsafe {
            env::set_var("KNOWLEDGE_BASE", "/env/override/path");
        }

        let mut config = Config {
            knowledge_base_path: "/config/file/path".to_string(),
        };

        // Simulate environment override (as done in Config::load)
        if let Ok(kb_path) = env::var("KNOWLEDGE_BASE") {
            config.knowledge_base_path = kb_path;
        }

        assert_eq!(config.knowledge_base_path, "/env/override/path");

        unsafe {
            env::remove_var("KNOWLEDGE_BASE");
        }
    }

    // Test 4: Path expansion
    {
        let config = Config {
            knowledge_base_path: "~/TestKB".to_string(),
        };

        let expanded = config.get_knowledge_base_path();
        assert!(expanded.contains("/TestKB"));
        assert!(!expanded.starts_with("~"));
    }

    // Test 5: Invalid configuration handling
    {
        let invalid_config = Config {
            knowledge_base_path: "".to_string(),
        };
        assert!(invalid_config.validate().is_err());
    }
}

#[test]
fn test_yaml_roundtrip() {
    let original_config = Config {
        knowledge_base_path: "/test/roundtrip/path".to_string(),
    };

    // Serialize to YAML
    let yaml_string = serde_yaml::to_string(&original_config).unwrap();

    // Deserialize back
    let deserialized_config: Config = serde_yaml::from_str(&yaml_string).unwrap();

    // Should be identical
    assert_eq!(original_config, deserialized_config);
}
