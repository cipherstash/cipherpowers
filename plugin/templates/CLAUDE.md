---
commands:
  test: "[your test command]"
  check: "[your check command]"
  build: "[your build command]"
  run: "[your run command]"
---

# Project Name

Brief description of what this project does.

## Development Commands

CipherPowers uses a tool-agnostic approach with context-aware command injection.

**Two ways to define commands:**

1. **Frontmatter (recommended)** - Parsed by hooks for automatic injection:
   ```yaml
   ---
   commands:
     test: "npm test"
     check: "npm run lint && npm run typecheck"
     build: "npm run build"
     run: "npm start"
   ---
   ```

2. **Development Commands section** - Human-readable documentation (optional if frontmatter exists)

### Core Commands

- **Tests**: `[your test command]` - Run the project's test suite (unit tests, integration tests, etc.)
- **Checks**: `[your check command]` - Run quality checks (linting, formatting, type checking, etc.)
- **Build**: `[your build command]` - Build/compile the project
- **Run**: `[your run command]` - Execute the application

### Examples for Common Ecosystems

**Node.js/npm:**
```yaml
---
commands:
  test: "npm test"
  check: "npm run lint && npm run typecheck"
  build: "npm run build"
  run: "npm start"
---
```

**Rust/Cargo:**
```yaml
---
commands:
  test: "cargo test"
  check: "cargo clippy -- -D warnings && cargo fmt --check"
  build: "cargo build --release"
  run: "cargo run"
---
```

**Python/Poetry:**
```yaml
---
commands:
  test: "poetry run pytest"
  check: "poetry run ruff check && poetry run mypy ."
  build: "poetry build"
  run: "poetry run python -m myapp"
---
```

**Task Runners (mise, just, make):**
```yaml
---
commands:
  test: "mise run test"
  check: "mise run check"
  build: "mise run build"
  run: "mise run start"
---
```

## Architecture

High-level overview of the project architecture, key design decisions, and important patterns.

## Development Workflow

Any project-specific workflow information, branching strategy, or development practices.

## Additional Context

Any other information Claude Code should know when working with this codebase.
