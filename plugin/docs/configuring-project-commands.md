# Configuring Project Commands

## Overview

CipherPowers uses a tool-agnostic approach that works with any build/test tooling ecosystem. Instead of hardcoding specific commands (like `mise run test` or `npm test`), the plugin uses natural language to describe command concepts and relies on projects to document their specific commands in CLAUDE.md.

## The Four Core Command Concepts

CipherPowers workflows reference four standardized command concepts:

1. **test** - Run the project's test suite (unit tests, integration tests, etc.)
2. **check** - Run quality checks (linting, formatting, type checking, etc.)
3. **build** - Build/compile the project (compile, bundle, package, etc.)
4. **run** - Execute the application (start server, run CLI, etc.)

## How It Works

### In Plugin Files

Plugin files (skills, agents, workflows) use natural language to describe commands:

- ✅ "run the project's tests" instead of `mise run test`
- ✅ "run quality checks" instead of `mise run check`
- ✅ "build the project" instead of `mise run build`
- ✅ "run the application" instead of `mise run X`

They also reference CLAUDE.md:
- "See CLAUDE.md for the specific command"
- "Consult the project's CLAUDE.md for the test command"

### In Your Project's CLAUDE.md

Each project documents its specific commands using the standardized vocabulary:

```markdown
## Development Commands

- **Tests**: `npm test` - Run the project's test suite
- **Checks**: `npm run lint && npm run typecheck` - Run quality checks
- **Build**: `npm run build` - Build/compile the project
- **Run**: `npm start` - Execute the application
```

### How Agents Use This

When an agent needs to run tests:

1. Agent reads natural language instruction: "run the project's tests"
2. Agent reads project's CLAUDE.md to find the **Tests** command
3. Agent executes the project-specific command (`npm test`, `cargo test`, etc.)

This approach leverages Claude's semantic understanding - agents naturally map "run tests" to the **Tests** entry in CLAUDE.md.

## Setting Up Your Project

### 1. Copy the Template

```bash
cp ${CLAUDE_PLUGIN_ROOT}plugin/templates/CLAUDE.md ./CLAUDE.md
```

### 2. Fill in Your Commands

Replace the placeholder commands with your project's actual commands:

```markdown
## Development Commands

### Core Commands

- **Tests**: `cargo test` - Run the project's test suite
- **Checks**: `cargo clippy -- -D warnings && cargo fmt --check` - Run quality checks
- **Build**: `cargo build --release` - Build/compile the project
- **Run**: `cargo run --bin myapp` - Execute the application
```

### 3. Optional: Add Composite Commands

If your project has composite workflows, document them:

```markdown
### Composite Workflows

- **test-check-build**: `mise run test && mise run check && mise run build` - Full quality gate
- **review:active**: `mise run review:active` - Find current work directory
```

## Examples for Different Ecosystems

### Node.js with npm

```markdown
## Development Commands

- **Tests**: `npm test` - Run the project's test suite
- **Checks**: `npm run lint && npm run typecheck` - Run quality checks
- **Build**: `npm run build` - Build/compile the project
- **Run**: `npm start` - Execute the application
```

### Rust with Cargo

```markdown
## Development Commands

- **Tests**: `cargo test` - Run the project's test suite
- **Checks**: `cargo clippy -- -D warnings && cargo fmt --check` - Run quality checks
- **Build**: `cargo build --release` - Build/compile the project
- **Run**: `cargo run` - Execute the application
```

### Python with Poetry

```markdown
## Development Commands

- **Tests**: `poetry run pytest` - Run the project's test suite
- **Checks**: `poetry run ruff check && poetry run mypy .` - Run quality checks
- **Build**: `poetry build` - Build/package the project
- **Run**: `poetry run python -m myapp` - Execute the application
```

### Task Runners (mise, just, make)

```markdown
## Development Commands

- **Tests**: `mise run test` - Run the project's test suite
- **Checks**: `mise run check` - Run quality checks
- **Build**: `mise run build` - Build/compile the project
- **Run**: `mise run start` - Execute the application
```

### Polyglot Projects

For projects using multiple languages/toolchains, document all relevant commands:

```markdown
## Development Commands

### Frontend (TypeScript/Vite)

- **Tests**: `npm test` - Run frontend test suite
- **Checks**: `npm run lint && npm run typecheck` - Run frontend quality checks
- **Build**: `npm run build` - Build frontend assets
- **Run**: `npm run dev` - Start development server

### Backend (Rust)

- **Tests**: `cargo test` - Run backend test suite
- **Checks**: `cargo clippy -- -D warnings` - Run backend quality checks
- **Build**: `cargo build --release` - Build backend binary
- **Run**: `cargo run` - Start backend server

### Full Stack

- **Tests**: `./scripts/test-all.sh` - Run all tests (frontend + backend)
- **Checks**: `./scripts/check-all.sh` - Run all quality checks
- **Build**: `./scripts/build-all.sh` - Build complete application
- **Run**: `docker-compose up` - Run full application stack
```

## Benefits

### For Users

- **Tool flexibility**: Plugin works with any build/test tooling
- **Clear separation**: Plugin provides workflows, projects provide implementation
- **Polyglot support**: Different tools per project, consistent workflows
- **More intuitive**: "run tests" is clearer than "mise run test"

### For Maintainers

- **No tool coupling**: No tool-specific assumptions to maintain
- **Easy evolution**: Change vocabulary once, not scattered references
- **More testable**: Skills describe universal patterns
- **Better upstream path**: Tool-agnostic skills can be contributed to superpowers

## Troubleshooting

### Agent doesn't find my commands

Ensure your CLAUDE.md uses the exact vocabulary:
- ✅ `**Tests**:` not `**Test**:` or `**Testing**:`
- ✅ `**Checks**:` not `**Check**:` or `**Linting**:`
- ✅ `**Build**:` not `**Compile**:`
- ✅ `**Run**:` not `**Start**:` or `**Execute**:`

### Complex command workflows

For multi-step workflows, you can:

1. Document composite commands in CLAUDE.md
2. Create shell scripts that agents can reference
3. Use task runners (mise, just, make) to orchestrate steps

Example:

```markdown
## Development Commands

- **Tests**: `./scripts/test.sh` - Run complete test suite
- **Checks**: `./scripts/check.sh` - Run all quality checks
```

Where `scripts/test.sh` contains:
```bash
#!/bin/bash
set -euo pipefail

pytest tests/unit
pytest tests/integration
playwright test
```

## Migration from mise-specific plugin

If you're migrating from a mise-specific setup:

1. Your existing `mise run X` commands still work
2. Document them in CLAUDE.md using the standardized vocabulary:
   ```markdown
   - **Tests**: `mise run test` - Run the project's test suite
   ```
3. No code changes needed - just documentation

This preserves your mise workflow while making the plugin tool-agnostic.
