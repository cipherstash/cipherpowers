# Quality Hooks

Automated quality enforcement via Claude Code's hook system. Runs project test and check commands automatically when agents modify code or complete work.

## Quick Start

**1. Create project-level configuration:**

```bash
# Recommended: .claude/gates.json
mkdir -p .claude
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json .claude/gates.json

# Customize commands for your project
vim .claude/gates.json
```

**2. Update commands to match your tooling:**

```json
{
  "gates": {
    "check": {"command": "npm run lint"},  // ← Your command
    "test": {"command": "npm test"}        // ← Your command
  }
}
```

See **[SETUP.md](./SETUP.md)** for detailed configuration guide.

## Debugging Hooks

If hooks aren't working as expected or agents seem stuck, debug logging is enabled by default.

**View logs:**

```bash
# Logs are written to: $TMPDIR/cipherpowers-hooks-YYYYMMDD.log
# On macOS: /var/folders/.../T/cipherpowers-hooks-YYYYMMDD.log
# On Linux: /tmp/cipherpowers-hooks-YYYYMMDD.log

# Watch logs in real-time
tail -f $TMPDIR/cipherpowers-hooks-$(date +%Y%m%d).log

# Or on macOS specifically
tail -f /var/folders/*/T/cipherpowers-hooks-$(date +%Y%m%d).log
```

**What gets logged:**
- Hook event type (PostToolUse, SubagentStop, UserPromptSubmit)
- Which gates.json config file is being used
- Which gates are running
- Gate command execution and exit codes
- Action handling (CONTINUE, BLOCK, STOP, chaining)
- Full flow from dispatcher → gate → action

**Common issues revealed by logging:**
- Gates not running: Check if tool/agent is in `enabled_tools`/`enabled_agents` list
- Wrong config: Check which gates.json path is being loaded
- Gate command failures: Check command output and exit codes
- Blocking behavior: See which action is being taken (BLOCK vs CONTINUE vs STOP)

**Disable logging (optional):**

```bash
export CIPHERPOWERS_HOOK_DEBUG=false
```

## Overview

Quality hooks integrate with Claude Code's event system to automatically run quality gates (test, check, build commands) at strategic points:

- **PostToolUse**: Runs after code modifications (Edit, Write tools)
- **SubagentStop**: Runs when specialized agents complete their work
- **UserPromptSubmit**: Runs when user submits a prompt (e.g., inject project commands)

**Configuration is project-specific** - each project has its own `gates.json` with commands tailored to that project's tooling.

## Implementation

The hooks system is implemented in TypeScript for type safety and testability. See [TYPESCRIPT.md](TYPESCRIPT.md) for architecture details.

**Quick links:**
- [TypeScript Architecture](TYPESCRIPT.md)
- [Creating Built-in Gates](TYPESCRIPT.md#create-new-built-in-gate)
- [Development Guide](TYPESCRIPT.md#development)

## Convention-Based Context Injection

**Zero-config content injection** via file naming convention.

### How It Works

1. Place markdown files in `.claude/context/` following naming pattern
2. Files auto-inject when corresponding hook fires
3. No `gates.json` configuration needed
4. Control via file presence (rename/move to disable)

### Naming Convention

```
Pattern: {command-or-skill-name}-{stage}.md

Examples:
  /code-review starts  → .claude/context/code-review-start.md
  /code-review ends    → .claude/context/code-review-end.md
  /plan starts         → .claude/context/plan-start.md
  TDD skill starts     → .claude/context/test-driven-development-start.md
```

### Directory Structures

**Flat (small projects):**
```
.claude/
├── gates.json
└── context/
    ├── code-review-start.md
    ├── code-review-end.md
    └── plan-start.md
```

**Organized (larger projects):**
```
.claude/
└── context/
    ├── slash-command/
    │   ├── code-review-start.md
    │   └── plan-start.md
    └── skill/
        └── test-driven-development-start.md
```

**Hierarchical (large projects):**
```
.claude/
└── context/
    └── slash-command/
        ├── code-review/
        │   ├── start.md
        │   └── end.md
        └── plan/
            └── start.md
```

Dispatcher searches all structures automatically.

### Execution Order

1. **Convention-based injection** (if file exists)
2. **Explicit gates** (from gates.json)
3. Continue or block based on results

### Example: Code Review Requirements

```bash
# Create context file
cat > .claude/context/code-review-start.md << 'EOF'
## Security Requirements

All reviews must verify:
- Authentication on all endpoints
- Input validation using allowlist
- No secrets in logs
EOF
```

Now when `/code-review` runs, requirements auto-inject. No configuration needed!

### Disabling Auto-Injection

Simply rename or move the file:
```bash
# Disable by renaming
mv .claude/context/code-review-start.md \
   .claude/context/code-review-start.md.disabled

# Or move out of discovery paths
mv .claude/context/code-review-start.md \
   .claude/disabled/code-review-start.md
```

### Examples

See `plugin/hooks/examples/context/` for:
- `code-review-start.md` - Security/performance requirements
- `plan-start.md` - Planning template
- `test-driven-development-start.md` - TDD standards

## Components

### 1. Hook Registration (`hooks.json`)

Registers hook scripts with Claude Code. All hooks use the universal dispatcher:

```json
{
  "hooks": {
    "PostToolUse": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher.sh"}]
    }],
    "SubagentStop": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher.sh"}]
    }],
    "UserPromptSubmit": [{
      "matcher": ".*",
      "hooks": [{"type": "command", "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher.sh"}]
    }]
  }
}
```

The dispatcher routes hook events to configured gates based on `gates.json`.

### 2. Gate Configuration (`gates.json`)

**Project-level configuration** - hooks search for `gates.json` in this order:

1. `.claude/gates.json` (recommended - project-specific)
2. `gates.json` (project root)
3. `${CLAUDE_PLUGIN_ROOT}/hooks/gates.json` (plugin default fallback)

**Context file discovery** - hooks search for context files in this order:

1. `.claude/context/{name}-{stage}.md` (flat)
2. `.claude/context/slash-command/{name}-{stage}.md` (organized)
3. `.claude/context/slash-command/{name}/{stage}.md` (hierarchical)
4. `.claude/context/skill/{name}-{stage}.md` (skill organized)
5. `.claude/context/skill/{name}/{stage}.md` (skill hierarchical)

Self-contained configuration defining gates, commands, and actions:

```json
{
  "gates": {
    "check": {
      "description": "Run quality checks",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-agent"],
      "gates": ["check", "test"]
    }
  }
}
```

### 3. Hook Scripts

- **dispatcher.sh**: Universal hook dispatcher - routes all hook events to configured gates
- **session-start.sh**: Special hook for session initialization (injects skills context)
- **shared-functions.sh**: Common gate execution logic (`run_gate`, `handle_action`)

### 4. Gate Scripts (`gates/`)

Gates can be either shell commands or custom scripts in `plugin/hooks/gates/`:

- **gates/commands.sh**: Built-in gate that **always runs first** on UserPromptSubmit events to inject project commands from CLAUDE.md frontmatter
- **gates/plan-compliance.sh**: Built-in gate that **always runs first** on SubagentStop events to check agent STATUS and handle BLOCKED reports

**Special behavior for built-in gates:**
- `commands.sh` runs automatically on UserPromptSubmit before any configured gates
- `plan-compliance.sh` runs automatically on SubagentStop before any configured gates
- Projects don't need to configure built-in gates in `gates.json`
- Additional gates can optionally be configured to run after built-in gates

Custom gate scripts receive hook context via `HOOK_INPUT` environment variable and output JSON.

## Gate Actions

Gates support four action types:

- **CONTINUE**: Proceed (default on pass)
- **BLOCK**: Prevent agent from proceeding (default on fail)
- **STOP**: Stop Claude entirely
- **{gate_name}**: Chain to another gate (subroutine call)

## Creating Custom Gate Scripts

Gate scripts are executables in `plugin/hooks/gates/` that receive hook context and output JSON.

**Input:** `HOOK_INPUT` environment variable (JSON with hook event data)

**Output:** JSON with one of:
- `{"additionalContext": "..."}` - Add context to conversation
- `{"decision": "block", "reason": "..."}` - Block execution
- `{"continue": false, "message": "..."}` - Stop Claude

**Example:** `plugin/hooks/gates/commands.sh` injects project commands from CLAUDE.md:

```bash
#!/bin/bash
set -euo pipefail

# Parse hook input
USER_MESSAGE=$(echo "$HOOK_INPUT" | jq -r '.user_message // ""')

# Your logic here...

# Output JSON
jq -n --arg content "$result" '{
  additionalContext: $content
}'
```

**Register in gates.json (optional - for running after built-in commands.sh):**
```json
{
  "gates": {
    "my-custom-gate": {
      "description": "My custom gate",
      "command": "${CLAUDE_PLUGIN_ROOT}/hooks/gates/my-custom-gate.sh",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "enabled": true,
      "gates": ["my-custom-gate"]
    }
  }
}
```

**Note:** Built-in gates (`commands.sh` for UserPromptSubmit, `plan-compliance.sh` for SubagentStop) still run first automatically, then custom gates run.

## Gate Chaining

Actions can reference other gates, creating complex workflows:

```json
{
  "gates": {
    "format": {
      "command": "mise run format",
      "on_pass": "check"
    },
    "check": {
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

When format passes → calls check gate → if check passes → continues

## Configuration

### Enable/Disable for Tools

```json
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "mcp__serena__replace_symbol_body"],
      "gates": ["check"]
    }
  }
}
```

### Enable/Disable for Agents

```json
{
  "hooks": {
    "SubagentStop": {
      "enabled_agents": ["rust-agent", "code-review-agent"],
      "gates": ["check", "test"]
    }
  }
}
```

### Per-Gate Actions

```json
{
  "gates": {
    "check": {
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  }
}
```

## Example Configurations

### Strict Mode

Block on all failures:

```json
{
  "gates": {
    "check": {"command": "mise run check", "on_fail": "BLOCK"},
    "test": {"command": "mise run test", "on_fail": "BLOCK"}
  }
}
```

### Permissive Mode

Warn only:

```json
{
  "gates": {
    "check": {"command": "mise run check", "on_fail": "CONTINUE"},
    "test": {"command": "mise run test", "on_fail": "CONTINUE"}
  }
}
```

### Gate Pipeline

Chain gates sequentially:

```json
{
  "gates": {
    "format": {"command": "mise run format", "on_pass": "check"},
    "check": {"command": "mise run check", "on_pass": "test"},
    "test": {"command": "mise run test"}
  },
  "hooks": {
    "SubagentStop": {"gates": ["format"]}
  }
}
```

## Testing

Test gates manually:

```bash
# Source functions
source plugin/hooks/shared-functions.sh

# Test gate execution
run_gate "check" "plugin/hooks/gates.json"
echo "Exit code: $?"

# Test with test configuration
run_gate "always-pass" "plugin/hooks/gates.test.json"
```

Test hooks with mock input:

```bash
# PostToolUse
echo '{"hook_event": "PostToolUse", "tool_name": "Edit", "cwd": "/test"}' | plugin/hooks/dispatcher.sh

# SubagentStop
echo '{"hook_event": "SubagentStop", "agent_name": "rust-agent", "cwd": "/test"}' | plugin/hooks/dispatcher.sh

# UserPromptSubmit
echo '{"hook_event": "UserPromptSubmit", "user_message": "run tests", "cwd": "/test"}' | plugin/hooks/dispatcher.sh
```

## Error Handling

- **Missing gate**: STOP with error message
- **Missing command**: STOP with error message
- **Invalid action**: STOP with error message
- **Gate failure with BLOCK**: Agent cannot proceed
- **Gate failure with STOP**: Claude halts entirely
- **Gate failure with CONTINUE**: Warning message, execution continues

## Benefits

1. **Consistent Quality**: All agent work validated automatically
2. **Early Detection**: Issues caught at edit time or completion
3. **Flexible Enforcement**: Configure actions per gate
4. **Self-Contained**: All configuration in gates.json
5. **Tool-Agnostic**: Works with any CLI tool
6. **Composable**: Build complex workflows via chaining
