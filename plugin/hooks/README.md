# Quality Hooks

Automated quality enforcement and context injection via Claude Code's hook system. A **self-referential TypeScript application** that uses its own configuration format.

> **💡 CONTEXT INJECTION IS AUTOMATIC**
>
> Just create `.claude/context/{name}-{stage}.md` files - they auto-inject at the right time.
> **No configuration files needed.** No gates.json. No setup.
>
> The `gates.json` file is ONLY for optional quality enforcement (lint, test, build checks).

## Quick Start

### Zero-Config Context Injection (Recommended)

**Just create context files - they auto-inject automatically:**

```bash
# Create context directory
mkdir -p .claude/context

# Add context for /code-review command
cat > .claude/context/code-review-start.md << 'EOF'
## Security Requirements
- Authentication on all endpoints
- Input validation for user data
- No secrets in logs
- HTTPS only
EOF

# That's it! When /code-review runs, requirements auto-inject!
```

**Works with ANY command, skill, or agent.** Follow the naming pattern: `.claude/context/{name}-{stage}.md`

### Advanced: Quality Gates (Optional)

**Need to enforce quality checks?** Add `gates.json` configuration:

```bash
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

See **[SETUP.md](./SETUP.md)** for detailed gate configuration.

## How It Works

```
Hook Event → Context Injection (AUTOMATIC) → [OPTIONAL: gates.json Gates] → Action
                 ↓                                        ↓
          .claude/context/                         Quality checks
          plugin/context/                          Custom commands
          (zero config!)                           (requires gates.json)
```

1. **Context Injection** (AUTOMATIC): Always runs first, discovers `.claude/context/{name}-{stage}.md` files
2. **Gate Execution** (OPTIONAL): If `gates.json` configured, runs quality checks/custom commands
3. **Action Handling**: CONTINUE, BLOCK, STOP, or chain to another gate

**Context injection works standalone - gates.json is only for optional quality enforcement.**

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

### Complete Zero-Config Example

**Step 1: Create context file**

```bash
mkdir -p .claude/context
cat > .claude/context/code-review-start.md << 'EOF'
## Security Checklist

- [ ] Authentication on all endpoints
- [ ] Input validation for user data
- [ ] No secrets in logs
- [ ] HTTPS only
- [ ] Rate limiting configured
EOF
```

**Step 2: Run the command**

```bash
/code-review src/api/users.ts
```

**Step 3: Context auto-injects**

The security checklist appears in the conversation automatically. **No configuration files needed!**

**This works with ANY slash command, skill, or agent.** Just follow the naming pattern: `.claude/context/{name}-{stage}.md`

### Hook-to-File Mapping

| Hook Type | File Pattern | Example |
|-----------|--------------|---------|
| `SessionStart` | `session-start.md` | Session begins |
| `UserPromptSubmit` | `prompt-submit.md` | User sends message |
| `SlashCommandStart` | `{command}-start.md` | `/code-review-start.md` |
| `SkillStart` | `{skill}-start.md` | `test-driven-development-start.md` |
| `SubagentStop` | `{agent}-end.md` | `rust-agent-end.md` |
| `PreToolUse` | `{tool}-pre.md` | `Edit-pre.md` |

See **[CONVENTIONS.md](./CONVENTIONS.md)** for full documentation.

## Gate Configuration (Optional)

**Most users only need context files.** Gates are for optional quality enforcement and custom commands.

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

### Agent Filtering for SubagentStop

**Important:** Without `enabled_agents`, SubagentStop triggers for ALL agents - including verification-only agents that don't modify code.

```json
{
  "hooks": {
    "SubagentStop": {
      "enabled_agents": ["rust-agent", "code-agent", "commit-agent"],
      "gates": ["check", "test"]
    }
  }
}
```

**Why this matters:**
- Verification agents (technical-writer in VERIFICATION mode, research-agent) only read files
- Running `check` and `test` gates after read-only verification is unnecessary
- Gate failures for verification agents confuse the workflow (false positives)

**Recommended pattern:** Only include agents that modify code:
- `rust-agent`, `code-agent` - write/edit code
- `commit-agent` - makes git commits
- Exclude: `technical-writer` (verification mode), `research-agent`, `plan-review-agent`

**Note:** `enabled_tools` works the same way for PostToolUse hooks.

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
