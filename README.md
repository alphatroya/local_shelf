# Local Shelf

A tool for organizing and converting markdown files in your Knowledge Base.

## Overview

Local Shelf is a command-line utility designed to help you manage markdown files by:
- Moving markdown files into your Knowledge Base pages directory (`stow` command)
- Converting markdown files to EPUB format (`convert` command)
- Automatically creating journal entries when files are added to your Knowledge Base

## Features

- 📝 **Organize Markdown Files**: Move markdown files from any directory to your Knowledge Base
- 📚 **Convert to EPUB**: Convert markdown files to EPUB format using Pandoc
- 📖 **Journal Integration**: Automatically create journal entries when files are added
- ⚙️ **Flexible Configuration**: Configure via config file or environment variables
- 🏠 **Path Expansion**: Supports tilde (`~`) expansion for home directory paths
- 🔄 **Cross-Platform**: Works on macOS, Linux, and other Unix-like systems

## Installation

### Prerequisites

- Rust toolchain (1.70 or later)
- For the `convert` command: [Pandoc](https://pandoc.org/installing.html)

### Build from Source

```bash
git clone https://github.com/alphatroya/local_shelf.git
cd local_shelf
cargo build --release
```

The binary will be available at `target/release/local_shelf`.

### Install

```bash
cargo install --path .
```

## Configuration

### Configuration File

Local Shelf uses a YAML configuration file located at:
- **macOS**: `~/Library/Application Support/local_shelf/config.yaml`
- **Linux**: `~/.config/local_shelf/config.yaml`

On first run, Local Shelf automatically creates a default configuration file with the following content:

```yaml
# Local Shelf Configuration
# 
# Knowledge Base path - where markdown files will be organized
# Can be overridden with KNOWLEDGE_BASE environment variable
knowledge_base_path: "~/Knowledge Base"
```

### Configuration Options

| Option | Description | Default Value |
|--------|-------------|---------------|
| `knowledge_base_path` | Path to your Knowledge Base directory | `~/Knowledge Base` |

### Environment Variable Override

You can override the configuration file settings using environment variables:

```bash
# Override the Knowledge Base path
KNOWLEDGE_BASE="/path/to/my/knowledge-base" local_shelf stow
```

Environment variables take precedence over configuration file settings.

### Path Expansion

Local Shelf supports tilde (`~`) expansion in paths. For example:
- `~/Knowledge Base` expands to `/home/username/Knowledge Base` on Linux
- `~/Documents/KB` expands to `/Users/username/Documents/KB` on macOS

### Migration from Legacy Configuration

If you were using an older version with the `local-shelf` directory name (with a hyphen), Local Shelf will automatically migrate your configuration to the new `local_shelf` directory (with an underscore) on first run.

## Usage

Local Shelf provides two main commands: `stow` and `convert`.

### Stow Command

Move markdown files from a directory to your Knowledge Base pages directory.

```bash
# Move markdown files from the current directory
local_shelf stow

# Move markdown files from a specific directory
local_shelf stow /path/to/directory
```

**What it does:**
1. Scans the specified directory (or current directory) for markdown files
2. Moves each markdown file to `{Knowledge Base}/pages/`
3. Creates journal entries for each moved file in `{Knowledge Base}/journal/{YYYY-MM-DD}.md`

**Example output:**
```
Local Shelf starting...
Knowledge Base path: /home/user/Knowledge Base
Scanning /tmp/notes for markdown files...
Found 2 markdown file(s) in /tmp/notes:
  - /tmp/notes/meeting-notes.md
  - /tmp/notes/project-ideas.md

Moving files to {Knowledge Base}/pages...
✓ Moved meeting-notes.md → /home/user/Knowledge Base/pages/meeting-notes.md
✓ Moved project-ideas.md → /home/user/Knowledge Base/pages/project-ideas.md

Successfully moved 2 file(s) to pages directory.
Creating journal entries...
✓ Added 2 journal entries to /home/user/Knowledge Base/journal/2026-02-06.md
```

### Convert Command

Convert markdown files to EPUB format using Pandoc.

```bash
# Convert markdown files in the current directory
local_shelf convert

# Convert markdown files in a specific directory
local_shelf convert /path/to/directory
```

**Prerequisites:** Pandoc must be installed and available in your PATH.

**What it does:**
1. Checks if Pandoc is installed
2. Scans the specified directory for markdown files
3. Converts each markdown file to EPUB format using Pandoc
4. Saves the EPUB files in the same directory as the source files

**Example output:**
```
Converting markdown files in: /tmp/notes
================================
Converting: /tmp/notes/article.md -> /tmp/notes/article.epub
✓ Successfully converted: article.epub

================================
Conversion complete!
Files converted: 1
Files skipped: 0
```

## Command Reference

```bash
# Display help
local_shelf --help

# Display version
local_shelf --version

# Stow command help
local_shelf stow --help

# Convert command help
local_shelf convert --help
```

## Examples

### Example 1: Organize Weekly Notes

```bash
# Move this week's notes to Knowledge Base
cd ~/Downloads/weekly-notes
local_shelf stow
```

### Example 2: Convert Book Chapters to EPUB

```bash
# Convert all markdown chapters to EPUB
cd ~/Documents/my-book
local_shelf convert
```

### Example 3: Custom Knowledge Base Path

```bash
# Use a different Knowledge Base location
KNOWLEDGE_BASE="~/Dropbox/Notes" local_shelf stow ~/Downloads
```

## Directory Structure

After using Local Shelf, your Knowledge Base will have the following structure:

```
~/Knowledge Base/
├── pages/
│   ├── meeting-notes.md
│   ├── project-ideas.md
│   └── ...
└── journal/
    ├── 2026-02-06.md
    ├── 2026-02-07.md
    └── ...
```

- **pages/**: Contains all your markdown files
- **journal/**: Contains daily journal entries with links to files added on that day

## Troubleshooting

### Configuration Issues

**Problem**: "Unable to determine config directory" error

**Solution**: Ensure your home directory is properly set. Check `$HOME` environment variable.

---

**Problem**: "Parent directory does not exist" error

**Solution**: Make sure the parent directory of your Knowledge Base path exists. For example, if your path is `~/Documents/Knowledge Base`, the `~/Documents` directory must exist.

### Pandoc Issues

**Problem**: "pandoc is not installed" error when using `convert` command

**Solution**: Install Pandoc from https://pandoc.org/installing.html

---

**Problem**: Pandoc conversion fails

**Solution**: Check that your markdown files are valid and Pandoc can read them. Try running `pandoc yourfile.md -o output.epub` manually to see detailed error messages.

## Development

See [AGENTS.md](AGENTS.md) for development guidelines, coding standards, and testing practices.

### Quick Start for Contributors

```bash
# Build the project
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Check linting
cargo clippy

# Run the application
cargo run -- stow /path/to/directory
```

## License

See the repository for license information.

## Contributing

Contributions are welcome! Please ensure:
1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No linting warnings (`cargo clippy`)

## Author

Created by the Local Shelf Contributors.
