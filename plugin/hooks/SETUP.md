# Quality Hooks Setup

## Simple Setup (Context Files Only)

**For most projects, context files are all you need.**

Context injection is AUTOMATIC - no configuration files required. Just create `.claude/context/` directory and add markdown files following the naming pattern.

### Quick Setup

```bash
# 1. Create context directory
mkdir -p .claude/context

# 2. Add context files for your commands/skills
# For /code-review command
cat > .claude/context/code-review-start.md << 'EOF'
## Security Requirements
- Authentication on all endpoints
- Input validation
- No secrets in logs
EOF

# For test-driven-development skill
cat > .claude/context/test-driven-development-start.md << 'EOF'
## TDD Standards
- Write failing test first
- Implement minimal code to pass
- Refactor with tests passing
EOF

# For session start
cat > .claude/context/session-start.md << 'EOF'
## Project Context
- TypeScript project using Vitest
- Follow functional programming style
- Use strict type checking
EOF
```

### That's It!

Context files auto-inject when commands/skills run. **No gates.json needed.**

**Need quality gates or custom commands?** Continue to "Advanced Setup" below.

---

## Advanced Setup (gates.json Configuration)

**Only needed for quality enforcement (lint, test, build checks) or custom commands.**

Quality hooks support optional **project-level** `gates.json` configuration for running quality checks.

### gates.json Search Priority

The hooks search for `gates.json` in this order:

1. **`.claude/gates.json`** - Project-specific configuration (recommended)
2. **`gates.json`** - Project root configuration
3. **`${CLAUDE_PLUGIN_ROOT}hooks/gates.json`** - Plugin default (fallback)

### Quick gates.json Setup

### Option 1: Recommended (.claude/gates.json)

```bash
# Create .claude directory
mkdir -p .claude

# Copy example configuration
cp ${CLAUDE_PLUGIN_ROOT}hooks/examples/strict.json .claude/gates.json

# Customize for your project
vim .claude/gates.json
```

### Option 2: Project Root (gates.json)

```bash
# Copy example configuration
cp ${CLAUDE_PLUGIN_ROOT}hooks/examples/strict.json gates.json

# Customize for your project
vim gates.json
```

## Customizing Gates

Edit your project's `gates.json` to match your build tooling:

```json
{
  "gates": {
    "check": {
      "description": "Run quality checks",
      "command": "npm run lint",  // ← Change to your command
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run tests",
      "command": "npm test",  // ← Change to your command
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

### Common Command Patterns

**Node.js/TypeScript:**
```json
{
  "gates": {
    "check": {"command": "npm run lint"},
    "test": {"command": "npm test"},
    "build": {"command": "npm run build"}
  }
}
```

**Rust:**
```json
{
  "gates": {
    "check": {"command": "cargo clippy"},
    "test": {"command": "cargo test"},
    "build": {"command": "cargo build"}
  }
}
```

**Python:**
```json
{
  "gates": {
    "check": {"command": "ruff check ."},
    "test": {"command": "pytest"},
    "build": {"command": "python -m build"}
  }
}
```

**mise tasks:**
```json
{
  "gates": {
    "check": {"command": "mise run check"},
    "test": {"command": "mise run test"},
    "build": {"command": "mise run build"}
  }
}
```

**Make:**
```json
{
  "gates": {
    "check": {"command": "make lint"},
    "test": {"command": "make test"},
    "build": {"command": "make build"}
  }
}
```

## Example Configurations

The plugin provides three example configurations:

### Strict Mode (Block on Failures)
```bash
cp ${CLAUDE_PLUGIN_ROOT}hooks/examples/strict.json .claude/gates.json
```

Best for: Production code, established projects

### Permissive Mode (Warn Only)
```bash
cp ${CLAUDE_PLUGIN_ROOT}hooks/examples/permissive.json .claude/gates.json
```

Best for: Prototyping, learning, experimental work

### Pipeline Mode (Chained Gates)
```bash
cp ${CLAUDE_PLUGIN_ROOT}hooks/examples/pipeline.json .claude/gates.json
```

Best for: Complex workflows, auto-formatting before checks

## Enabling/Disabling Hooks

### Disable Quality Hooks Entirely

Remove or rename your project's `gates.json`:

```bash
mv .claude/gates.json .claude/gates.json.disabled
```

### Disable Specific Hooks

Edit `gates.json` to remove hooks:

```json
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": [],  // ← Empty = disabled
      "gates": []
    },
    "SubagentStop": {
      "enabled_agents": ["rust-agent"],  // ← Keep enabled
      "gates": ["check", "test"]
    }
  }
}
```

### Disable Specific Tools/Agents

Remove from enabled lists:

```json
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],  // ← Removed "Write"
      "gates": ["check"]
    }
  }
}
```

## Testing Your Configuration

```bash
# Test gate execution manually
source ${CLAUDE_PLUGIN_ROOT}hooks/shared-functions.sh
run_gate "check" ".claude/gates.json"

# Verify JSON is valid
jq . .claude/gates.json

# Test with mock hook input
export CLAUDE_PLUGIN_ROOT=/path/to/plugin
echo '{"tool_name": "Edit", "cwd": "'$(pwd)'"}' | ${CLAUDE_PLUGIN_ROOT}hooks/post-tool-use.sh
```

## Version Control

### Recommended: Commit gates.json

```bash
git add .claude/gates.json
git commit -m "chore: configure quality gates"
```

This ensures all team members use the same quality standards.

### Optional: Per-Developer Override

Developers can override with local configuration:

```bash
# Team config
.claude/gates.json  ← committed

# Personal override (gitignored)
gates.json  ← takes priority, not committed
```

Add to `.gitignore`:
```
/gates.json
```

## Troubleshooting

### Hooks Not Running

1. Check configuration exists:
   ```bash
   ls -la .claude/gates.json
   ```

2. Verify plugin root is set:
   ```bash
   echo $CLAUDE_PLUGIN_ROOT
   ```

3. Check tool/agent is enabled:
   ```bash
   jq '.hooks.PostToolUse.enabled_tools' .claude/gates.json
   ```

### Gate Fails for Verification-Only Agents

**Symptom:** SubagentStop gates fail for agents that only read files (technical-writer in verification mode, research-agent).

**Cause:** Missing `enabled_agents` filter - gates run for ALL agents.

**Solution:** Add `enabled_agents` to only include code-modifying agents:

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

**Why:** Verification agents don't modify code, so check/test gates are unnecessary and produce false positive failures.

### Commands Failing

1. Test command manually:
   ```bash
   npm run lint  # or whatever your check command is
   ```

2. Check command exists:
   ```bash
   which npm
   ```

3. Verify working directory:
   - Commands run from project root (where gates.json lives)
   - Use absolute paths if needed

### JSON Syntax Errors

```bash
# Validate JSON
jq . .claude/gates.json

# Common errors:
# - Missing commas between items
# - Trailing commas in arrays/objects
# - Unescaped quotes in strings
```

## Migration from Plugin Default

If you were using the plugin's default `gates.json`, migrate to project-level:

```bash
# Copy current config
cp ${CLAUDE_PLUGIN_ROOT}hooks/gates.json .claude/gates.json

# Customize for this project
vim .claude/gates.json
```

The plugin default now serves as a fallback template only.
