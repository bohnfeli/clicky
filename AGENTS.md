# AGENTS.md - Rust CLI Project Guidelines

## General Instructions to follow

- Ask questions using the question tool as soon as something is unclear
- Ask one question at a time. After the question is answered as the next question
- Allways to to use simple solutions, but follow clean code

## Archectectural Approach

- Use a `Clean Architecture` style architecture.
- Do TDD for new features.
- Use Test Driven Bugfixing: first create a test that reproduces the bug, then fix the bug.
- Allways update the documentation when possible

## Documentation guidelines

- Store all insights in the arc42 documentation under `./docs`
- Use PlantUML diagrams for documenting and planning business logic

## Quality Gates

Before creating a PR, ensure all quality gates pass locally:

```bash
# Install required tools
cargo install cargo-audit

# Run all quality gates
make check
```

Or run individual checks:

```bash
# Build with all features
cargo build --all-features

# Run all tests
cargo test --all-features

# Run clippy linter
cargo clippy --all-features -- -D warnings

# Check formatting
cargo fmt -- --check

# Security audit
cargo audit

# Build documentation
cargo doc --no-deps --all-features
```

All these checks are automatically run in CI and must pass before merging.
See [docs/developer/quality-gates.adoc](docs/developer/quality-gates.adoc) for detailed documentation.

## Build Commands

```bash
# Build the project
cargo build

# Build for release (optimized)
cargo build --release

# Check code without building
cargo check

# Clean build artifacts
cargo clean
```

## Test Commands

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name

# Run tests matching a pattern
cargo test test_prefix_

# Run tests in a specific module
cargo test module_name::

# Run tests with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

## Lint Commands

```bash
# Run clippy (linter)
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Run clippy and treat warnings as errors
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

## Code Style Guidelines

### Imports
- Group imports: std lib first, then external crates, then local modules
- Use `use crate::` for local module imports
- Avoid wildcard imports (`use module::*`)
- Sort imports alphabetically within groups

```rust
use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::utils::format_output;
```

### Formatting
- Use `cargo fmt` with default settings
- Max line width: 100 characters
- 4 spaces for indentation (tabs converted to spaces)
- Trailing commas in multi-line structures

### Types & Naming
- Use `PascalCase` for types, traits, enums, structs
- Use `snake_case` for functions, variables, modules, files
- Use `SCREAMING_SNAKE_CASE` for constants, statics
- Use `PascalCase` for enum variants
- Prefer explicit types over `impl Trait` in public APIs
- Use `Result<T, E>` for fallible operations

### Error Handling
- Use `thiserror` or `anyhow` for error handling
- Create custom error types for library code
- Use `?` operator to propagate errors
- Avoid `unwrap()` and `expect()` in production code
- Use `ok()` or `ok_or()` for Option to Result conversions

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid configuration: {0}")]
    Config(String),
}

fn read_file(path: &Path) -> Result<String, CliError> {
    std::fs::read_to_string(path)?
        .map_err(|e| CliError::Io(e))
}
```

### CLI Patterns
- Use `clap` for argument parsing with derive macros
- Support `--help` and `--version` flags
- Use meaningful exit codes (0 = success, non-zero = error)
- Support stdin input and stdout output
- Follow POSIX conventions for flags and arguments

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "mycli")]
#[command(about = "A brief description")]
#[command(version)]
struct Args {
    #[arg(short, long, help = "Input file path")]
    input: Option<PathBuf>,
    
    #[arg(short, long, default_value = "output.txt")]
    output: PathBuf,
    
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}
```

### Documentation
- Document all public items with `///`
- Include examples in doc comments
- Use `//!` for module-level documentation
- Run `cargo doc` to generate documentation

### Testing
- Unit tests in `src/` files with `#[cfg(test)]` module
- Integration tests in `tests/` directory
- Use `tempfile` crate for temporary files in tests
- Use `assert_cmd` and `predicates` for CLI testing
- **Follow the Arrange-Act-Assert pattern** for test structure:
  - **Arrange**: Set up test data and dependencies
  - **Act**: Execute the code under test
  - **Assert**: Verify the expected outcome
- **Use test-driven development (TDD)**: Write tests before implementing features

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_functional() {
        // Arrange
        let input1 = 2;
        let input2 = 3;

        // Act
        let output = add(input1, input2);

        // Assert
        assert_eq!(output, 5, "Die Addition von {} und {} sollte 5 ergeben", input1, input2);
    }
}
```

### Dependencies
- Keep dependencies minimal
- Use `cargo tree` to audit dependency tree
- Pin versions in `Cargo.toml` for reproducible builds
- Use `cargo audit` to check for security vulnerabilities

## Git Workflow

### 1. Update develop branch
```bash
git checkout develop
git pull origin develop
```

### 2. Create a feature branch from develop
Use descriptive branch names with prefixes:
- `feature/` - new features
- `bugfix/` - bug fixes
- `hotfix/` - urgent production fixes
- `refactor/` - code refactoring

```bash
git checkout -b feature/add-user-authentication
```

### 3. Make small, focused commits
Use [Conventional Commits](https://www.conventionalcommits.org/) format:
- `feat:` - new feature
- `fix:` - bug fix
- `docs:` - documentation only
- `style:` - formatting, missing semicolons, etc.
- `refactor:` - code change that neither fixes a bug nor adds a feature
- `test:` - adding or correcting tests
- `chore:` - changes to build process, dependencies, etc.

```bash
git add .
git commit -m "feat: add OAuth2 authentication for GitHub"
```

### 4. Create a Pull Request
```bash
git push -u origin feature/add-user-authentication
```

Then create a PR on GitHub with:
- Clear title describing the change
- Description explaining what and why
- Link to related issues
- Ensure CI checks pass before requesting review
