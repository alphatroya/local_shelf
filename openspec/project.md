# Project Context

## Purpose
The main purpose is to search all new .md article files in ~/Downloads/ folder and move them to {{Knowledge Base}}/pages folder.
After them add a new link with timestamp to the end of today {{Knowledge Base}}/journals folder.

The $KNOWLEDGE_BASE location parameter is stored in the project config folder (~/.config/local_shelf/config.yaml on Linux, ~/Library/Application Support/local_shelf/config.yaml on macOS).

Journals file name should be YYYY_MM_DD.md format. Create a new file if not exists, otherwise add entry at the end of the file.
A new journal entry should suit this format:
```
- **HH:mm** [[Name of the file]]
```

## Configuration
The application uses a YAML-based configuration system with the following hierarchy (highest precedence first):
1. Environment variables (KNOWLEDGE_BASE)
2. User configuration file (~/.config/local_shelf/config.yaml)
3. Default values

### Configuration Options
- `knowledge_base_path`: Path to the Knowledge Base directory (default: "~/Knowledge Base")

### Environment Variables
- `KNOWLEDGE_BASE`: Overrides the knowledge_base_path configuration

## Tech Stack
- Rust as primary language
- Jujutsu as VCS

## Project Conventions

### Code Style
- General Rust project guidelines
- Use `cargo fmt` after all performed tasks.
- Call `cargo clippy` for check any new linter issues.

### Testing Strategy
- Write comprehensive testing suite before each feature implementation.
- MUST USE TDD as a tests strategy

### Jujutsu Workflow
- Use `jj new` before implementing a new feature.
- Use `jj desc -m` for describing an upcoming change.

## Important Constraints
- Before moving markdown file ensure that new file destination is not exists. Ensure any rewrite are forbidden.
- If same file exists, append a new hash postfix to new one.
