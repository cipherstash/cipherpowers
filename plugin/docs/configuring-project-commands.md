# Configuring Project Commands

## Overview

CipherPowers uses a tool-agnostic approach that works with any build/test tooling ecosystem. Projects configure their specific commands in `.claude/gates.json` and optionally use `.claude/context/` files for additional content injection.

## How It Works

### 1. Keyword-Triggered Gates

The plugin defines gates with keywords that trigger conditionally based on user message content:

```json
{
  "gates": {
    "test": {
      "keywords": ["test", "testing", "spec", "verify"],
      "command": "npm test",
      "on_fail": "BLOCK"
    },
    "check": {
      "keywords": ["lint", "check", "format", "quality"],
      "command": "npm run lint",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["test", "check"]
    }
  }
}
```

**Behavior:**
- When user says "run the tests", the `test` gate runs
- When user says "check the code quality", the `check` gate runs
- General questions don't trigger any gates (efficient)

### 2. Context Injection (Optional)

For custom content injection, create files in `.claude/context/`:

```
.claude/context/prompt-submit.md     # Injected on every user message
.claude/context/code-review-start.md # Injected when /code-review starts
```

This replaces the old CLAUDE.md frontmatter approach with more flexible file-based content.

## Setting Up Your Project

### 1. Create Project Gates Configuration

```bash
mkdir -p .claude
cat > .claude/gates.json << 'EOF'
{
  "gates": {
    "check": {
      "description": "Run quality checks",
      "keywords": ["lint", "check", "format", "quality", "clippy", "typecheck"],
      "command": "npm run lint",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run test suite",
      "keywords": ["test", "testing", "spec", "verify"],
      "command": "npm test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "build": {
      "description": "Build project",
      "keywords": ["build", "compile", "package"],
      "command": "npm run build",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["check", "test", "build"]
    },
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write"],
      "gates": ["check"]
    }
  }
}
EOF
```

### 2. Document Commands in CLAUDE.md (Optional)

For human reference and agent fallback:

```markdown
## Development Commands

- **Tests**: `npm test` - Run the project's test suite
- **Checks**: `npm run lint && npm run typecheck` - Run quality checks
- **Build**: `npm run build` - Build/compile the project
- **Run**: `npm start` - Execute the application
```

## Examples for Different Ecosystems

### Node.js

```json
{
  "gates": {
    "check": {"command": "npm run lint && npm run typecheck"},
    "test": {"command": "npm test"},
    "build": {"command": "npm run build"}
  }
}
```

### Rust

```json
{
  "gates": {
    "check": {"command": "cargo clippy -- -D warnings && cargo fmt --check"},
    "test": {"command": "cargo test"},
    "build": {"command": "cargo build --release"}
  }
}
```

### Python

```json
{
  "gates": {
    "check": {"command": "ruff check . && mypy ."},
    "test": {"command": "pytest"},
    "build": {"command": "python -m build"}
  }
}
```

### Task Runners (mise, just, make)

```json
{
  "gates": {
    "check": {"command": "mise run check"},
    "test": {"command": "mise run test"},
    "build": {"command": "mise run build"}
  }
}
```

## Default Keywords

The plugin defaults use these keyword mappings:

| Gate | Keywords |
|------|----------|
| `check` | lint, check, format, quality, clippy, typecheck |
| `test` | test, testing, spec, verify |
| `build` | build, compile, package |

Gates without keywords always run (for PostToolUse, SubagentStop hooks).

## Benefits

- **Tool flexibility**: Works with any build/test tooling
- **Token efficiency**: Gates only run when keywords match
- **Clear separation**: Plugin provides workflows, projects provide commands
- **Flexible content**: Use `.claude/context/` for any custom injection
- **Polyglot support**: Different commands per project

## Migration from CLAUDE.md Frontmatter

If you previously used CLAUDE.md frontmatter:

**Before (deprecated):**
```yaml
---
commands:
  test: "npm test"
  check: "npm run lint"
---
```

**After:**
```json
// .claude/gates.json
{
  "gates": {
    "test": {
      "keywords": ["test", "testing"],
      "command": "npm test"
    },
    "check": {
      "keywords": ["lint", "check"],
      "command": "npm run lint"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["test", "check"]
    }
  }
}
```

The new approach:
- Gives projects full control over gate configuration
- Supports keyword triggering for efficiency
- Uses standard gates.json format (same as other hooks)
- Allows custom content via `.claude/context/` files
