# Change: Add File Search Logic in Downloads Folder

## Why
The application needs to discover markdown files in the ~/Downloads directory as the first step of the Knowledge Base organization workflow. Currently, the application only loads configuration but has no capability to find and identify markdown files that need processing.

## What Changes
- Add file system scanning capability for ~/Downloads directory
- Implement markdown file filtering (.md extension detection)
- Add path expansion for tilde (~) notation
- Provide file discovery interface for the main workflow

## Impact
- Affected specs: file-discovery (new capability)
- Affected code: src/main.rs (add file discovery), new file discovery module
- Dependencies: Standard library filesystem operations