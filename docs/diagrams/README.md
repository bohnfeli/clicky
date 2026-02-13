# Architecture Diagrams

This directory contains PlantUML diagrams for documenting the Clicky architecture and processes.

## Diagrams

### quality-gates.puml

Visualizes the quality gates process flow, showing how changes move through the CI/CD pipeline.

*What it shows*:
- Developer workflow (push/PR)
- Parallel execution of quality gates
- Feedback loop when checks fail
- Final approval for merging

**To render**: `plantuml quality-gates.puml`

### ci-architecture.puml

Shows the detailed architecture of the CI/CD system including caching strategy, matrix testing, and all quality gate components.

*What it shows*:
- GitHub Actions workflow structure
- Matrix strategy for multiple Rust versions
- Caching layers for performance
- Quality gate checks in parallel
- Integration with external services (GitHub Dependency Database)

**To render**: `plantuml ci-architecture.puml`

## Usage

### Using PlantUML CLI

```bash
# Install PlantUML
brew install plantuml  # macOS
# or download from https://plantuml.com/download

# Render a diagram
plantuml quality-gates.puml

# Render as PNG (default)
plantuml quality-gates.puml

# Render as SVG
plantuml -tsvg quality-gates.puml
```

### Using VS Code

Install the PlantUML extension and use the preview pane to view diagrams.

### Online Editor

Use the official PlantUML online editor: https://plantuml.com/online

## Adding New Diagrams

When adding new diagrams:

1. Create a `.puml` file with a descriptive name
2. Use `@startuml` and `@enduml` markers
3. Add a title with `title` directive
4. Include descriptive notes where helpful
5. Update this README with the new diagram description

## Linking in Documentation

In AsciiDoc files, reference diagrams using:

```asciidoc
image::diagrams/quality-gates.svg[Quality Gates Flow, role="diagram"]
```

Note: The `.svg` extension assumes you've rendered the PlantUML file to SVG format.
