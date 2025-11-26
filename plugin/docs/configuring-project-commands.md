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

### Convention: CLAUDE.md Frontmatter

Projects define commands in CLAUDE.md frontmatter using YAML:

```yaml
---
commands:
  test: "npm test"
  check: "npm run lint && npm run typecheck"
  build: "npm run build"
  run: "npm start"
---
```

This follows **convention over configuration** - no separate config files needed.

### Context-Aware Hook Injection

The `user-prompt-submit.sh` hook automatically injects relevant commands based on:

1. **Slash commands** - `/cipherpowers:commit`, `/cipherpowers:execute`, `/cipherpowers:code-review` trigger specific commands
2. **Message keywords** - "run tests", "check quality", "build" trigger relevant commands
3. **No injection** - General questions don't inject commands (saves tokens)

**Example flow:**
```bash
User: "Please commit my changes" + /commit
→ Hook detects /commit
→ Injects: test, check, build commands
→ Agent sees: <project_commands><test>npm test</test>...</project_commands>

User: "Run the tests"
→ Hook detects "tests" keyword
→ Injects: test command only
→ Agent sees: <project_commands><test>npm test</test></project_commands>

User: "How does authentication work?"
→ Hook detects no command keywords
→ No injection (saves tokens)
```

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

### How Agents Use Commands

**With hook injection (automatic):**
1. User types message or slash command
2. Hook detects needed commands and injects from frontmatter
3. Agent sees commands in context: `<test>npm test</test>`
4. Agent executes the command directly

**Without hook (fallback):**
1. Agent reads natural language instruction: "run the project's tests"
2. Agent reads CLAUDE.md frontmatter or Development Commands section
3. Agent executes the project-specific command

The hook makes this seamless - commands are available when needed without agents having to read CLAUDE.md.

## Setting Up Your Project

### 1. Copy the Template

```bash
cp ${CLAUDE_PLUGIN_ROOT}templates/CLAUDE.md ./CLAUDE.md
```

### 2. Define Commands in Frontmatter

Add YAML frontmatter at the top of CLAUDE.md:

```yaml
---
commands:
  test: "cargo test"
  check: "cargo clippy -- -D warnings && cargo fmt --check"
  build: "cargo build --release"
  run: "cargo run --bin myapp"
---
```

**The hook automatically parses this and injects commands when needed.**

### 3. Optional: Document Commands for Humans

If helpful for team documentation, add a Development Commands section:

```markdown
## Development Commands

- **Tests**: `cargo test` - Run all tests
- **Checks**: `cargo clippy -- -D warnings && cargo fmt --check` - Quality checks
- **Build**: `cargo build --release` - Production build
- **Run**: `cargo run --bin myapp` - Start application
```

**Note:** With frontmatter, this section is optional - the hook works from frontmatter alone.

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

## Hook Behavior Details

### Command Detection

The `user-prompt-submit.sh` hook detects needed commands through:

**1. Slash Command Mapping:**
- `/cipherpowers:commit` → test, check, build
- `/cipherpowers:execute` → test, check, build
- `/cipherpowers:code-review` → test, check
- `/cipherpowers:brainstorm`, `/cipherpowers:plan` → no injection

**2. Keyword Detection:**
- "test", "testing", "spec", "verify" → test
- "lint", "check", "format", "quality", "clippy", "typecheck" → check
- "build", "compile", "package" → build
- "run", "start", "execute the application" → run

**3. No Match:**
- General questions, refactoring requests, etc. → no injection

### Benefits

✅ **Token efficiency** - Only inject when needed (saves ~100-200 tokens per message)
✅ **Zero agent work** - Commands pre-injected, no CLAUDE.md reads required
✅ **Context-aware** - Different commands for different tasks
✅ **Automatic** - No agent configuration or prompt engineering needed
✅ **Reliable** - Frontmatter parsing is deterministic

### Example Token Savings

Without hook:
```
User: "commit changes"
Agent: reads CLAUDE.md (~500 tokens)
Agent: extracts test command
Agent: extracts check command
Agent: extracts build command
Total: ~600 tokens for command discovery
```

With hook:
```
User: "commit changes"
Hook: injects <test>...<check>...<build> (~100 tokens)
Agent: uses injected commands directly
Total: ~100 tokens, 5x more efficient
```

## Advanced Usage

### Custom Commands

Beyond the core four, you can define custom project commands:

```yaml
---
commands:
  test: "npm test"
  check: "npm run lint"
  build: "npm run build"
  run: "npm start"
  deploy: "npm run deploy"
  docs: "npm run docs:build"
---
```

These won't be auto-injected by the hook (only test/check/build/run are), but agents can still reference them from CLAUDE.md when needed.

### Scoped Commands (Optional)

For fine-grained control:

```yaml
---
commands:
  test: "npm test"
  test:unit: "npm run test:unit"
  test:integration: "npm run test:integration"
  check: "npm run check"
  check:lint: "npm run lint"
  check:types: "npm run typecheck"
---
```

Hook will use base command (`test`) unless agent specifically requests scoped version (`test:unit`).
