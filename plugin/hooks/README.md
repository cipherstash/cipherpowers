# Quality Hooks

Automated quality enforcement and context injection via Claude Code's hook system. A **self-referential TypeScript application** that uses its own configuration format.

## Quick Start

**Zero configuration required!** The plugin provides sensible defaults:

- **SessionStart**: Injects agent selection guide
- **UserPromptSubmit**: Runs keyword-triggered gates (check, test, build) when relevant keywords detected

**Optional: Add project-specific configuration:**

```bash
# Create project gates.json (overrides plugin defaults)
mkdir -p .claude
cat > .claude/gates.json << 'EOF'
{
  "gates": {
    "check": {"command": "npm run lint", "on_fail": "BLOCK"},
    "test": {"command": "npm test", "on_fail": "BLOCK"}
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write"],
      "gates": ["check"]
    }
  }
}
EOF
```

See **[SETUP.md](./SETUP.md)** for detailed configuration.

## How It Works

```
Hook Event → Context Injection → gates.json Gates → Action
                 ↓                      ↓              ↓
          .claude/context/       TypeScript or     CONTINUE
          plugin/context/        Shell command     BLOCK/STOP
```

1. **Context Injection** (always runs first): Discovers `.claude/context/{name}-{stage}.md` files
2. **Gate Execution**: Runs configured gates from merged `gates.json`
3. **Action Handling**: CONTINUE, BLOCK, STOP, or chain to another gate

See **[ARCHITECTURE.md](./ARCHITECTURE.md)** for detailed system design.

## Supported Hook Events

All 12 Claude Code hook types are supported:

| Event | Context Pattern | Default Behavior |
|-------|----------------|------------------|
| `SessionStart` | `session-start.md` | Plugin injects agent selection guide |
| `SessionEnd` | `session-end.md` | - |
| `UserPromptSubmit` | `prompt-submit.md` | Keyword-triggered gates (check, test, build) |
| `SlashCommandStart` | `{command}-start.md` | - |
| `SlashCommandEnd` | `{command}-end.md` | - |
| `SkillStart` | `{skill}-start.md` | - |
| `SkillEnd` | `{skill}-end.md` | - |
| `SubagentStop` | `{agent}-end.md` | - |
| `PreToolUse` | `{tool}-pre.md` | - |
| `PostToolUse` | `{tool}-post.md` | - |
| `Stop` | `agent-stop.md` | - |
| `Notification` | `notification-receive.md` | - |

## Context Injection

**Zero-config content injection** via file naming convention.

### Naming Convention

```
Pattern: .claude/context/{name}-{stage}.md

Examples:
  /code-review starts  → .claude/context/code-review-start.md
  /plan starts         → .claude/context/plan-start.md
  TDD skill loads      → .claude/context/test-driven-development-start.md
  SessionStart fires   → .claude/context/session-start.md
```

### Priority Order

1. **Project context** (`.claude/context/`) - highest priority
2. **Plugin context** (`${CLAUDE_PLUGIN_ROOT}/context/`) - fallback defaults

Projects can override any plugin-provided context by creating their own file.

### Example: Code Review Requirements

```bash
cat > .claude/context/code-review-start.md << 'EOF'
## Security Requirements

All reviews must verify:
- Authentication on all endpoints
- Input validation
- No secrets in logs
EOF
```

Now when `/cipherpowers:code-review` runs, requirements auto-inject!

See **[CONVENTIONS.md](./CONVENTIONS.md)** for full documentation.

## Gate Configuration

Gates are defined in `gates.json` and can be:

### Shell Command Gates

```json
{
  "gates": {
    "check": {
      "command": "npm run lint",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

### TypeScript Gates

Gates without `command` field are TypeScript modules in `src/gates/`:

```json
{
  "gates": {
    "plan-compliance": {
      "description": "Verify work follows the active plan",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

See **[TYPESCRIPT.md](./TYPESCRIPT.md)** for creating TypeScript gates.

### Keyword-Triggered Gates

Gates can define `keywords` to only run when the user message contains matching terms:

```json
{
  "gates": {
    "test": {
      "description": "Run project test suite",
      "keywords": ["test", "testing", "spec", "verify"],
      "command": "npm test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["test"]
    }
  }
}
```

**Behavior:**
- Gates with `keywords` only run if any keyword is found in the user message
- Gates without `keywords` always run (backwards compatible)
- Keyword matching is case-insensitive

## Configuration Merging

The system merges plugin and project configurations:

```
plugin/hooks/gates.json     (defaults)
        ↓ merged with
.claude/gates.json          (project overrides)
        ↓
Merged Configuration        (project takes precedence)
```

**Plugin provides defaults. Projects override what they need.**

## Debugging

Logs are written to `$TMPDIR/cipherpowers/hooks-YYYY-MM-DD.log`:

```bash
# View logs in real-time
tail -f $(node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js log-path)

# Or find the log file
ls $TMPDIR/cipherpowers/hooks-*.log
```

**What gets logged:**
- Hook event received
- Config files loaded
- Context files discovered
- Gates executed
- Actions taken

## Documentation

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - System design and data flow
- **[CONVENTIONS.md](./CONVENTIONS.md)** - Context file naming conventions
- **[SETUP.md](./SETUP.md)** - Detailed configuration guide
- **[TYPESCRIPT.md](./TYPESCRIPT.md)** - Creating TypeScript gates
- **[INTEGRATION_TESTS.md](./INTEGRATION_TESTS.md)** - Testing procedures

## Examples

See `examples/` for ready-to-use configurations:

- `strict.json` - Block on all failures
- `permissive.json` - Warn only
- `pipeline.json` - Gate chaining
- `context/` - Example context files
