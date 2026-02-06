## 1. CLI Parameter Updates
- [x] 1.1 Add optional directory parameter to `stow` subcommand structure  
- [x] 1.2 Update `Commands::Stow` enum variant to accept `path: Option<PathBuf>`
- [x] 1.3 Update help text for `stow` command to document directory parameter
- [x] 1.4 Ensure `convert` command structure remains consistent

## 2. Stow Command Implementation Changes
- [x] 2.1 Modify `handle_stow_command()` to accept optional directory parameter
- [x] 2.2 Update file discovery logic to use specified directory or default to current directory
- [x] 2.3 Replace hardcoded `~/Downloads` scanning with parameterized directory scanning
- [x] 2.4 Ensure all existing file operations work with any specified directory

## 3. Parameter Validation and Error Handling
- [x] 3.1 Add directory existence validation for both commands
- [x] 3.2 Implement consistent error messages for invalid directories
- [x] 3.3 Ensure proper error handling when directory permissions are insufficient
- [x] 3.4 Validate that directory parameter accepts both relative and absolute paths

## 4. Testing Updates
- [x] 4.1 Update existing `stow` tests to work with new directory parameter
- [x] 4.2 Add tests for `stow` with custom directory parameter
- [x] 4.3 Add tests for `stow` with default behavior (current directory)
- [x] 4.4 Ensure `convert` tests continue to pass without modification
- [x] 4.5 Add integration tests for CLI parameter parsing changes
- [x] 4.6 Test edge cases: empty directories, permission issues, non-existent paths

## 5. Documentation Updates  
- [x] 5.1 Update CLI help text to reflect unified parameter structure
- [x] 5.2 Ensure version information and other help text remains accurate
- [x] 5.3 Document the breaking change for users who relied on `~/Downloads` default

## 6. Code Quality and Validation
- [x] 6.1 Run `cargo fmt` on all modified code
- [x] 6.2 Run `cargo clippy` and resolve any linting issues
- [x] 6.3 Ensure all existing tests continue to pass
- [x] 6.4 Verify both commands work identically when given the same directory parameter