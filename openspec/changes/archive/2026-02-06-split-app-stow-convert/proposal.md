# Change: Split Application Into Stow and Convert Commands

## Why
The current local_shelf application only provides file organization functionality (moving markdown files from Downloads to Knowledge Base). Users need additional functionality to convert markdown files to EPUB format. The existing `convert_md_to_epub.sh` script demonstrates this need but is separate from the main application. Consolidating both functionalities into a single CLI with subcommands will provide a cohesive user experience.

## What Changes
- **Version bump from 0.1.0 to 0.2.0** due to breaking CLI changes
- Refactor current implementation into `stow` subcommand (preserve existing behavior)
- Add `convert` subcommand based on the existing shell script functionality  
- Implement CLI argument parsing using a command-line parsing library
- Restructure main.rs to dispatch to appropriate subcommand handlers
- Update all existing tests to work with the new structure
- Add comprehensive tests for the new conversion functionality
- Maintain backward compatibility where possible

## Impact
- Affected specs: cli, conversion (new), configuration, file-operations
- Affected code: src/main.rs (major restructure), Cargo.toml (new dependencies), all test files
- **BREAKING**: Changes CLI interface from `local_shelf` to `local_shelf stow` for existing functionality
- Users will need to update scripts/aliases to use `local_shelf stow` instead of `local_shelf`
- New conversion functionality will be available via `local_shelf convert [folder_path]`