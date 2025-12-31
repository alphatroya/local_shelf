# Project Context

## Purpose
The main purpose to is to search all new .md articles files in ~/Downloads/ folder and move them to {{Knowledge Base}}/pages folder.
After them add a new link with timestamp to the end of today {{Knowledge Base}}/journals folder.

$KNOWLEDGE_BASE location parameter should be stored in project config folder (~/.config/{{ project }}/config.yaml

Journals file name should be YYYY_MM_DD.md format. Create a new file if not exists, otherwise add entry at the end of the file.
A new journal entry should suite this format:
```
- **HH_mm** [[Name of the file]]
```

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
