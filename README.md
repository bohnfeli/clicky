# Clicky

A CLI kanban board for human-agent collaboration.

## Overview

Clicky is a command-line kanban board designed to facilitate collaboration between human users and coding agents (AI assistants, scripts, automation tools). It provides a simple, text-based interface for managing tasks that works seamlessly in both interactive and automated contexts.

## Features

- ✨ **CLI-First Design**: All operations accessible via intuitive commands
- 🤖 **Human-Agent Collaboration**: Perfect for manual use and automation
- 💾 **JSON Storage**: Human-readable, version-control friendly
- 🏗️ **Clean Architecture**: Maintainable and extensible codebase
- 🎯 **Simple & Fast**: Minimal overhead, maximum productivity

## Installation

### From crates.io (coming soon)

```bash
cargo install clicky
```

### From Source

```bash
git clone https://github.com/yourusername/clicky.git
cd clicky
cargo build --release
# Binary will be at ./target/release/clicky
```

## Quick Start

```bash
# Initialize a board in your project directory
clicky init

# Create a new card
clicky create "Implement feature" --assignee "Alice" --description "Add OAuth2 support"

# List all cards
clicky list

# Move card to in-progress
clicky move PRJ-001 in_progress

# Show card details
clicky show PRJ-001

# Mark as complete
clicky move PRJ-001 done
```

## Documentation

- 📖 [User Guide](docs/user-guide/quickstart.adoc) - Installation and usage
- 🏗️ [Architecture Docs](docs/arc42/index.adoc) - System architecture
- 💻 [Developer Guide](docs/developer/contributing.adoc) - Contributing guidelines

## Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize a new kanban board |
| `create` | Create a new card |
| `move` | Move a card to another column |
| `show` | Display card details |
| `list` | List all cards |
| `update` | Update card details |
| `delete` | Delete a card |
| `info` | Show board information |

## Architecture

Clicky follows **Clean Architecture** principles with clear separation of concerns:

```
src/
├── cli/           # CLI interface (clap)
├── application/   # Use cases and services
├── domain/        # Business entities and logic
└── infrastructure/# Storage and I/O
```

## Data Storage

Board data is stored in `.clicky/board.json` in your project directory:

```json
{
  "id": "myproject",
  "name": "My Project",
  "card_id_prefix": "MYP",
  "columns": [...],
  "cards": [...]
}
```

This makes it easy to:
- Inspect data manually
- Version control your board
- Backup and migrate
- Integrate with other tools

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy -- -D warnings
cargo fmt
```

## License

MIT OR Apache-2.0

## Contributing

We welcome contributions! See our [Contributing Guide](docs/developer/contributing.adoc) for details.