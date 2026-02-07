# Local Shelf

A tool for organizing and converting markdown files in your Knowledge Base.

## Overview

Local Shelf is a command-line utility designed to help you manage markdown files by:
- Moving markdown files into your Knowledge Base pages directory (`stow` command)
- Converting markdown files to EPUB format (`convert` command)
- Automatically creating journal entries when files are added to your Knowledge Base

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
