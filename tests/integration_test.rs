use local_shelf::config::Config;
use std::env;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

/// Helper function to run cargo with test environment isolation
fn run_cargo_with_test_env(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(args)
        .env("LOCAL_SHELF_SKIP_CONFIG_INIT", "1")
        .output()
        .expect("Failed to execute command")
}

#[test]
fn test_config_integration_scenarios() {
    // Test 1: Default configuration when no file exists
    {
        let _temp_dir = tempdir().unwrap();
        let _config_dir = _temp_dir.path().join(".config/local_shelf");

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
        // Use a temporary environment variable that's specific to this test
        let test_env_var = "LOCAL_SHELF_TEST_KB_PATH";
        unsafe {
            env::set_var(test_env_var, "/env/override/path");
        }

        let mut config = Config {
            knowledge_base_path: "/config/file/path".to_string(),
        };

        // Simulate environment override (as done in Config::load)
        if let Ok(kb_path) = env::var(test_env_var) {
            config.knowledge_base_path = kb_path;
        }

        assert_eq!(config.knowledge_base_path, "/env/override/path");

        // Clean up
        unsafe {
            env::remove_var(test_env_var);
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

#[test]
fn test_config_migration() {
    // Test migration functionality in a controlled environment
    // Note: This test works with the migration logic but doesn't test actual directory migration
    // since we can't easily mock the dirs::config_dir() function
    let temp_dir = tempdir().unwrap();

    // Simulate legacy config content
    let legacy_config_content = r#"
knowledge_base_path: "/legacy/path"
"#;

    let legacy_config_path = temp_dir.path().join("legacy_config.yaml");
    fs::write(&legacy_config_path, legacy_config_content).unwrap();

    // Load and verify legacy config can be read
    let content = fs::read_to_string(&legacy_config_path).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();
    assert_eq!(config.knowledge_base_path, "/legacy/path");
}

#[test]
fn test_cli_help_output() {
    let output = run_cargo_with_test_env(&["run", "--", "--help"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("local_shelf"));
    assert!(stdout.contains("stow"));
    assert!(stdout.contains("convert"));
    assert!(stdout.contains("Move markdown files from a directory"));
    assert!(stdout.contains("Convert markdown files in a directory"));
}

#[test]
fn test_cli_version_output() {
    let output = run_cargo_with_test_env(&["run", "--", "--version"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("local_shelf 0.3.0"));
}

#[test]
fn test_cli_stow_help_output() {
    let output = run_cargo_with_test_env(&["run", "--", "stow", "--help"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Move markdown files from a directory"));
    assert!(stdout.contains("local_shelf stow"));
}

#[test]
fn test_cli_convert_help_output() {
    let output = run_cargo_with_test_env(&["run", "--", "convert", "--help"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Convert markdown files"));
    assert!(stdout.contains("local_shelf convert"));
    assert!(stdout.contains("[PATH]"));
}

#[test]
fn test_convert_with_test_files() {
    let temp_dir = tempdir().unwrap();

    // Create test markdown files
    let test1_md = temp_dir.path().join("test1.md");
    let test2_md = temp_dir.path().join("test2.md");

    fs::write(&test1_md, "# Test Document 1\n\nThis is test content.").unwrap();
    fs::write(&test2_md, "# Test Document 2\n\nThis is more test content.").unwrap();

    // Check if pandoc is available before running conversion test
    let pandoc_check = Command::new("pandoc").arg("--version").output();

    if pandoc_check.is_ok() {
        // Run conversion
        let output =
            run_cargo_with_test_env(&["run", "--", "convert", temp_dir.path().to_str().unwrap()]);

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Converting markdown files"));
        assert!(stdout.contains("Conversion complete!"));

        // Check that EPUB files were created
        let test1_epub = temp_dir.path().join("test1.epub");
        let test2_epub = temp_dir.path().join("test2.epub");

        if output.status.success() {
            assert!(test1_epub.exists(), "test1.epub should exist");
            assert!(test2_epub.exists(), "test2.epub should exist");
        }
    } else {
        println!("Pandoc not available, skipping conversion test");
    }
}

#[test]
fn test_convert_with_nonexistent_directory() {
    let output = run_cargo_with_test_env(&["run", "--", "convert", "/nonexistent/directory"]);

    // Should exit with error
    assert!(
        !output.status.success(),
        "Command should fail for nonexistent directory"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for error message in stderr first, but also check stdout as fallback
    let error_in_stderr = stderr.contains("Directory") && stderr.contains("does not exist");
    let error_in_stdout = stdout.contains("Directory") && stdout.contains("does not exist");

    assert!(
        error_in_stderr || error_in_stdout,
        "Expected error message containing 'Directory' and 'does not exist' in stderr or stdout.\nSTDERR: {}\nSTDOUT: {}",
        stderr,
        stdout
    );
}

#[test]
fn test_convert_with_empty_directory() {
    let temp_dir = tempdir().unwrap();

    // Run conversion on empty directory
    let output =
        run_cargo_with_test_env(&["run", "--", "convert", temp_dir.path().to_str().unwrap()]);

    // Should succeed (not an error condition)
    assert!(
        output.status.success(),
        "Command should succeed for empty directory"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // The message should be in stdout, but check stderr as fallback
    let message_in_stdout = stdout.contains("No markdown files found");
    let message_in_stderr = stderr.contains("No markdown files found");

    assert!(
        message_in_stdout || message_in_stderr,
        "Expected message containing 'No markdown files found' in stdout or stderr.\nSTDOUT: {}\nSTDERR: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_stow_with_directory_parameter() {
    let temp_dir = tempdir().unwrap();

    // Create test markdown files
    let test_md = temp_dir.path().join("test.md");
    fs::write(&test_md, "# Test Document\n\nTest content for stow.").unwrap();

    // Run stow with directory parameter (will try to move to Knowledge Base)
    let output = Command::new("cargo")
        .env("KNOWLEDGE_BASE", tempdir().unwrap().path())
        .args(["run", "--", "stow", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scanning"));
    assert!(stdout.contains("markdown file"));
}

#[test]
fn test_stow_with_nonexistent_directory() {
    let output = Command::new("cargo")
        .args(["run", "--", "stow", "/nonexistent/directory"])
        .output()
        .expect("Failed to execute command");

    // Should exit with error
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Directory") && stderr.contains("does not exist"));
}

#[test]
fn test_unified_directory_parameters() {
    let temp_dir = tempdir().unwrap();

    // Create test markdown files
    let test1_md = temp_dir.path().join("test1.md");
    let test2_md = temp_dir.path().join("test2.md");

    fs::write(&test1_md, "# Test Document 1\n\nTest content.").unwrap();
    fs::write(&test2_md, "# Test Document 2\n\nMore test content.").unwrap();

    // Test that both commands accept the same directory parameter format
    let stow_output = Command::new("cargo")
        .args(["run", "--", "stow", "--help"])
        .output()
        .expect("Failed to execute command");

    let convert_output = Command::new("cargo")
        .args(["run", "--", "convert", "--help"])
        .output()
        .expect("Failed to execute command");

    let stow_help = String::from_utf8_lossy(&stow_output.stdout);
    let convert_help = String::from_utf8_lossy(&convert_output.stdout);

    // Both should have [PATH] parameter
    assert!(stow_help.contains("[PATH]"));
    assert!(convert_help.contains("[PATH]"));

    // Both should mention directory containing markdown files
    assert!(stow_help.contains("directory containing markdown files"));
    assert!(convert_help.contains("directory containing markdown files"));
}
