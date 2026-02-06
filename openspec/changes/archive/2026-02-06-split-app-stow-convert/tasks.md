## 1. Version and Dependency Updates
- [x] 1.1 Bump version in Cargo.toml from 0.1.0 to 0.2.0 (breaking change)
- [x] 1.2 Add clap dependency to Cargo.toml for CLI argument parsing

## 2. CLI Structure Implementation
- [x] 2.1 Create CLI structure with subcommands (stow, convert)
- [x] 2.2 Define command-line arguments and options for each subcommand
- [x] 2.3 Implement argument validation and help text

## 3. Stow Command Implementation  
- [x] 3.1 Extract current main.rs logic into stow subcommand handler
- [x] 3.2 Ensure stow command preserves all existing functionality
- [x] 3.3 Maintain compatibility with existing configuration system
- [x] 3.4 Update error handling to work within subcommand structure

## 4. Convert Command Implementation
- [x] 4.1 Create conversion module based on convert_md_to_epub.sh script
- [x] 4.2 Implement pandoc dependency detection using standard PATH lookup
- [x] 4.3 Add markdown file discovery for conversion target directory
- [x] 4.4 Implement EPUB conversion with progress reporting
- [x] 4.5 Add conversion statistics and error handling
- [x] 4.6 Support single directory processing only (non-recursive)

## 5. Main Application Restructure
- [x] 5.1 Refactor main.rs to dispatch to subcommand handlers
- [x] 5.2 Implement unified error handling across subcommands
- [x] 5.3 Add version and help information for main CLI
- [x] 5.4 Ensure proper exit codes for different scenarios

## 6. Testing Updates
- [x] 6.1 Update all existing tests to work with stow subcommand structure
- [x] 6.2 Create comprehensive unit tests for conversion functionality
- [x] 6.3 Add integration tests for CLI argument parsing
- [x] 6.4 Test error conditions for both subcommands (missing pandoc, invalid paths, etc.)
- [x] 6.5 Add tests for CLI help text and version display

## 7. Code Quality & Documentation
- [x] 7.1 Run cargo fmt on all modified code
- [x] 7.2 Run cargo clippy and resolve any new linting issues
- [x] 7.3 Update README or help documentation if needed
- [x] 7.4 Ensure all public APIs have proper documentation

## 8. Validation
- [x] 8.1 Test stow command maintains exact behavior of current implementation
- [x] 8.2 Test convert command matches shell script functionality
- [x] 8.3 Verify CLI help and error messages are user-friendly
- [x] 8.4 Confirm all tests pass with new structure
- [x] 8.5 Verify version number displays correctly in CLI help and --version flag