# Convention-Based Context Injection Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use cipherpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add convention-based context injection to CipherPowers hook system, allowing zero-config content injection via file naming patterns without requiring `auto_inject_context` configuration.

**Architecture:** Extend dispatcher.sh to discover and auto-inject context files based on naming convention (`.claude/context/{command-or-skill}-{start|end}.md`). Context injection happens before explicit gates, similar to existing commands.sh pattern. Control is via file presence/absence - rename or move files to disable.

**Tech Stack:** Bash (hook scripts), jq (JSON parsing), existing hook infrastructure

---

## Task 1: Add Helper Function for Context File Discovery

**Files:**
- Modify: `plugin/hooks/shared-functions.sh:153` (end of file)

**Step 1: Write the failing test**

Create test file:
```bash
# plugin/hooks/tests/test-context-discovery.sh
#!/usr/bin/env bash
source "$(dirname "$0")/../shared-functions.sh"

# Test: discover_context_file finds flat structure
mkdir -p /tmp/test-context/.claude/context
echo "test content" > /tmp/test-context/.claude/context/code-review-start.md

result=$(discover_context_file "/tmp/test-context" "code-review" "start")
expected="/tmp/test-context/.claude/context/code-review-start.md"

if [ "$result" = "$expected" ]; then
  echo "PASS: Flat structure discovery"
else
  echo "FAIL: Expected $expected, got $result"
  exit 1
fi

rm -rf /tmp/test-context
```

**Step 2: Run test to verify it fails**

Run: `bash plugin/hooks/tests/test-context-discovery.sh`
Expected: FAIL with "discover_context_file: command not found"

**Step 3: Write minimal implementation**

Add to `plugin/hooks/shared-functions.sh`:
```bash
# Discover context file using convention-based naming
# Args: cwd, name (command/skill without prefix), stage (start/end)
# Returns: path to context file if exists, empty if not found
discover_context_file() {
  local cwd="$1"
  local name="$2"
  local stage="$3"

  # Try discovery paths in priority order
  local paths=(
    "${cwd}/.claude/context/${name}-${stage}.md"                    # Flat
    "${cwd}/.claude/context/slash-command/${name}-${stage}.md"      # Organized
    "${cwd}/.claude/context/slash-command/${name}/${stage}.md"      # Hierarchical
    "${cwd}/.claude/context/skill/${name}-${stage}.md"              # Skill organized
    "${cwd}/.claude/context/skill/${name}/${stage}.md"              # Skill hierarchical
  )

  for path in "${paths[@]}"; do
    if [ -f "$path" ]; then
      echo "$path"
      return 0
    fi
  done

  # Not found
  return 1
}
```

**Step 4: Run test to verify it passes**

Run: `bash plugin/hooks/tests/test-context-discovery.sh`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/shared-functions.sh plugin/hooks/tests/test-context-discovery.sh
git commit -m "feat(hooks): add context file discovery with multiple path support"
```

---

## Task 2: Add Helper Function for Context File Injection

**Files:**
- Modify: `plugin/hooks/shared-functions.sh:180` (after discover_context_file)

**Step 1: Write the failing test**

Add to test file:
```bash
# plugin/hooks/tests/test-context-injection.sh
#!/usr/bin/env bash
source "$(dirname "$0")/../shared-functions.sh"

# Test: inject_context_file outputs valid JSON
mkdir -p /tmp/test-inject/.claude/context
echo "Security checklist content" > /tmp/test-inject/.claude/context/test.md

result=$(inject_context_file "/tmp/test-inject/.claude/context/test.md")

# Verify JSON structure
if echo "$result" | jq -e '.additionalContext' > /dev/null; then
  echo "PASS: Valid JSON with additionalContext"
else
  echo "FAIL: Invalid JSON structure"
  exit 1
fi

# Verify content
content=$(echo "$result" | jq -r '.additionalContext')
if [ "$content" = "Security checklist content" ]; then
  echo "PASS: Content matches"
else
  echo "FAIL: Expected 'Security checklist content', got '$content'"
  exit 1
fi

rm -rf /tmp/test-inject
```

**Step 2: Run test to verify it fails**

Run: `bash plugin/hooks/tests/test-context-injection.sh`
Expected: FAIL with "inject_context_file: command not found"

**Step 3: Write minimal implementation**

Add to `plugin/hooks/shared-functions.sh`:
```bash
# Inject context file content into conversation
# Args: file_path
# Output: JSON with additionalContext field (to stdout)
inject_context_file() {
  local file="$1"

  if [ ! -f "$file" ]; then
    log_debug "inject_context_file: File not found: $file"
    return 1
  fi

  local content=$(cat "$file")

  log_debug "inject_context_file: Injecting content from $file (${#content} chars)"

  # Output as JSON additionalContext (per Claude Code hook spec)
  jq -n --arg content "$content" '{
    additionalContext: $content
  }'
}
```

**Step 4: Run test to verify it passes**

Run: `bash plugin/hooks/tests/test-context-injection.sh`
Expected: PASS (both assertions)

**Step 5: Commit**

```bash
git add plugin/hooks/shared-functions.sh plugin/hooks/tests/test-context-injection.sh
git commit -m "feat(hooks): add context file injection with JSON output"
```

---

## Task 3: Add SlashCommandStart Hook Support to Dispatcher

**Files:**
- Modify: `plugin/hooks/dispatcher.sh:34-59` (case statement)

**Step 1: Write the failing test**

Create test:
```bash
# plugin/hooks/tests/test-slash-command-hook.sh
#!/usr/bin/env bash

# Create test context file
mkdir -p /tmp/test-slash/.claude/context
echo "Code review requirements" > /tmp/test-slash/.claude/context/code-review-start.md

# Mock hook input
INPUT=$(jq -n '{
  hook_event_name: "SlashCommandStart",
  command: "/code-review",
  user_message: "review this code",
  cwd: "/tmp/test-slash"
}')

# Run dispatcher
result=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

# Verify context was injected
if echo "$result" | jq -e '.additionalContext' > /dev/null 2>&1; then
  content=$(echo "$result" | jq -r '.additionalContext')
  if [ "$content" = "Code review requirements" ]; then
    echo "PASS: Context injected for SlashCommandStart"
  else
    echo "FAIL: Wrong content: $content"
    exit 1
  fi
else
  echo "FAIL: No additionalContext in output"
  exit 1
fi

rm -rf /tmp/test-slash
```

**Step 2: Run test to verify it fails**

Run: `bash plugin/hooks/tests/test-slash-command-hook.sh`
Expected: FAIL with "Unknown hook event" or no output

**Step 3: Write minimal implementation**

Modify `plugin/hooks/dispatcher.sh` case statement (after line 48):
```bash
  UserPromptSubmit)
    CONTEXT_KEY="user_message"
    CONTEXT_VALUE=$(echo "$INPUT" | jq -r '.user_message // ""')
    ENABLED_LIST_KEY="enabled"
    log_debug "dispatcher: User message (truncated): ${CONTEXT_VALUE:0:100}..."
    ;;
  SlashCommandStart|SlashCommandEnd)
    COMMAND=$(echo "$INPUT" | jq -r '.command // ""')
    COMMAND_NAME="${COMMAND#/}"  # Remove leading /
    STAGE="${HOOK_EVENT#SlashCommand}"  # "Start" or "End"
    STAGE_LOWER=$(echo "$STAGE" | tr '[:upper:]' '[:lower:]')
    CONTEXT_FILE=$(discover_context_file "$CWD" "$COMMAND_NAME" "$STAGE_LOWER")
    CONTEXT_KEY="command"
    CONTEXT_VALUE="$COMMAND"
    ENABLED_LIST_KEY="enabled_commands"
    log_debug "dispatcher: Command: $COMMAND, Stage: $STAGE_LOWER"
    [ -n "$CONTEXT_FILE" ] && log_debug "dispatcher: Context file: $CONTEXT_FILE"
    ;;
  *)
    # Unknown hook event - exit cleanly
```

**Step 4: Add context injection logic before gates**

Add after config loading (after line 85):
```bash
log_debug "dispatcher: Hook '$HOOK_EVENT' is configured"

# Convention-based injection (if context file exists)
if [ -n "${CONTEXT_FILE:-}" ] && [ -f "$CONTEXT_FILE" ]; then
  log_debug "dispatcher: Auto-injecting context from $CONTEXT_FILE"
  inject_context_file "$CONTEXT_FILE"
fi

# For PostToolUse and SubagentStop: Check if context value is in enabled list
```

**Step 5: Run test to verify it passes**

Run: `bash plugin/hooks/tests/test-slash-command-hook.sh`
Expected: PASS

**Step 6: Commit**

```bash
git add plugin/hooks/dispatcher.sh plugin/hooks/tests/test-slash-command-hook.sh
git commit -m "feat(hooks): add SlashCommandStart/End hook support with auto-injection"
```

---

## Task 4: Add SkillStart/SkillEnd Hook Support

**Files:**
- Modify: `plugin/hooks/dispatcher.sh:59` (after SlashCommand case)

**Step 1: Write the failing test**

Create test:
```bash
# plugin/hooks/tests/test-skill-hook.sh
#!/usr/bin/env bash

# Create test context file
mkdir -p /tmp/test-skill/.claude/context
echo "TDD requirements" > /tmp/test-skill/.claude/context/test-driven-development-start.md

# Mock hook input
INPUT=$(jq -n '{
  hook_event_name: "SkillStart",
  skill: "test-driven-development",
  user_message: "implement feature with TDD",
  cwd: "/tmp/test-skill"
}')

# Run dispatcher
result=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

# Verify context was injected
if echo "$result" | jq -e '.additionalContext' > /dev/null 2>&1; then
  content=$(echo "$result" | jq -r '.additionalContext')
  if [ "$content" = "TDD requirements" ]; then
    echo "PASS: Context injected for SkillStart"
  else
    echo "FAIL: Wrong content: $content"
    exit 1
  fi
else
  echo "FAIL: No additionalContext in output"
  exit 1
fi

rm -rf /tmp/test-skill
```

**Step 2: Run test to verify it fails**

Run: `bash plugin/hooks/tests/test-skill-hook.sh`
Expected: FAIL (unknown hook event)

**Step 3: Write minimal implementation**

Add to dispatcher.sh case statement (after SlashCommand case):
```bash
  SlashCommandStart|SlashCommandEnd)
    COMMAND=$(echo "$INPUT" | jq -r '.command // ""')
    COMMAND_NAME="${COMMAND#/}"  # Remove leading /
    STAGE="${HOOK_EVENT#SlashCommand}"  # "Start" or "End"
    STAGE_LOWER=$(echo "$STAGE" | tr '[:upper:]' '[:lower:]')
    CONTEXT_FILE=$(discover_context_file "$CWD" "$COMMAND_NAME" "$STAGE_LOWER")
    CONTEXT_KEY="command"
    CONTEXT_VALUE="$COMMAND"
    ENABLED_LIST_KEY="enabled_commands"
    log_debug "dispatcher: Command: $COMMAND, Stage: $STAGE_LOWER"
    [ -n "$CONTEXT_FILE" ] && log_debug "dispatcher: Context file: $CONTEXT_FILE"
    ;;
  SkillStart|SkillEnd)
    SKILL=$(echo "$INPUT" | jq -r '.skill // ""')
    STAGE="${HOOK_EVENT#Skill}"  # "Start" or "End"
    STAGE_LOWER=$(echo "$STAGE" | tr '[:upper:]' '[:lower:]')
    CONTEXT_FILE=$(discover_context_file "$CWD" "$SKILL" "$STAGE_LOWER")
    CONTEXT_KEY="skill"
    CONTEXT_VALUE="$SKILL"
    ENABLED_LIST_KEY="enabled_skills"
    log_debug "dispatcher: Skill: $SKILL, Stage: $STAGE_LOWER"
    [ -n "$CONTEXT_FILE" ] && log_debug "dispatcher: Context file: $CONTEXT_FILE"
    ;;
  *)
```

**Step 4: Run test to verify it passes**

Run: `bash plugin/hooks/tests/test-skill-hook.sh`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/dispatcher.sh plugin/hooks/tests/test-skill-hook.sh
git commit -m "feat(hooks): add SkillStart/End hook support with auto-injection"
```

---

## Task 5: Create Example Context Files

**Files:**
- Create: `plugin/hooks/examples/context/code-review-start.md`
- Create: `plugin/hooks/examples/context/plan-start.md`
- Create: `plugin/hooks/examples/context/test-driven-development-start.md`

**Step 1: Create code-review-start.md example**

```bash
mkdir -p plugin/hooks/examples/context
cat > plugin/hooks/examples/context/code-review-start.md << 'EOF'
## Project-Specific Code Review Requirements

This file demonstrates convention-based context injection.

**Location:** `.claude/context/code-review-start.md`

**Triggered by:** Running `/code-review` command (SlashCommandStart hook)

**Purpose:** Inject project-specific review requirements automatically.

---

### Additional Security Checks

For this project, code reviews MUST verify:

1. **Authentication:** All API endpoints require valid JWT
2. **Input Validation:** All user inputs use allowlist validation
3. **Rate Limiting:** Public endpoints have rate limits configured
4. **Logging:** No PII in application logs

### Performance Requirements

- Database queries: No N+1 patterns
- API response time: < 200ms for p95
- Memory usage: No leaks detected in tests

### Documentation

- Public APIs have JSDoc/TSDoc comments
- Complex algorithms have inline explanations
- Breaking changes noted in CHANGELOG.md

---

**To use:** Copy to `.claude/context/code-review-start.md` in your project.
EOF
```

**Step 2: Create plan-start.md example**

```bash
cat > plugin/hooks/examples/context/plan-start.md << 'EOF'
## Project Planning Template

**Location:** `.claude/context/plan-start.md`

**Triggered by:** Running `/plan` command (SlashCommandStart hook)

Your implementation plan must include:

### Architecture Impact
- Which services/modules are affected?
- Any new dependencies introduced?
- Database schema changes required?

### API Surface
- New endpoints or breaking changes?
- Version bump needed?
- Backward compatibility strategy?

### Testing Strategy
- Unit test coverage target (80%+)
- Integration tests for new flows
- E2E tests for user-facing features

### Deployment Considerations
- Feature flags required?
- Migration scripts needed?
- Rollback strategy?

### Success Criteria
- What does "done" look like?
- How to verify it works?
- What metrics to monitor?
EOF
```

**Step 3: Create TDD skill example**

```bash
cat > plugin/hooks/examples/context/test-driven-development-start.md << 'EOF'
## Project TDD Standards

**Location:** `.claude/context/test-driven-development-start.md`

**Triggered by:** When `test-driven-development` skill loads (SkillStart hook)

This project uses:

- **Test framework:** Vitest
- **Test location:** `src/**/__tests__/*.test.ts`
- **Coverage requirement:** 80% line coverage minimum
- **Property testing:** Use fast-check for algorithms

### File Structure
```
src/
  components/
    Button/
      Button.tsx
      __tests__/
        Button.test.tsx
```

### Naming Convention
- Use `describe/it` blocks (not `test()`)
- Test names: "should [behavior] when [condition]"
- File naming: `{Component}.test.ts`

### Mocking Strategy
- Mock external services (APIs, databases)
- Do NOT mock internal modules (test real behavior)
- Use MSW for HTTP mocking

### RED-GREEN-REFACTOR
1. Write failing test first
2. Run test (verify it fails for right reason)
3. Write minimal code to pass
4. Run test (verify it passes)
5. Refactor (if needed)
6. Commit
EOF
```

**Step 4: Verify files created**

Run: `ls -la plugin/hooks/examples/context/`
Expected: Three .md files listed

**Step 5: Commit**

```bash
git add plugin/hooks/examples/context/
git commit -m "docs(hooks): add example context files for common patterns"
```

---

## Task 6: Update Hook Documentation

**Files:**
- Modify: `plugin/hooks/README.md:77-120` (after Overview section)

**Step 1: Add Convention-Based Injection section**

Insert after line 77 (after Overview section):
```markdown
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
```

**Step 2: Update Components section to mention context discovery**

Find Components section (around line 80), update **2. Gate Configuration** subsection:
```markdown
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
```

**Step 3: Verify documentation reads correctly**

Run: `cat plugin/hooks/README.md | grep -A 5 "Convention-Based"`
Expected: Section header and first few lines visible

**Step 4: Commit**

```bash
git add plugin/hooks/README.md
git commit -m "docs(hooks): document convention-based context injection"
```

---

## Task 7: Create CONVENTIONS.md Documentation

**Files:**
- Create: `plugin/hooks/CONVENTIONS.md`

**Step 1: Create comprehensive conventions guide**

```bash
cat > plugin/hooks/CONVENTIONS.md << 'EOF'
# Hook System Conventions

Convention-based patterns for zero-config hook customization.

## Overview

Conventions allow project-specific hook behavior without editing `gates.json`. Place files following naming patterns and they auto-execute at the right time.

## Convention Types

### 1. Context Injection

**Purpose:** Auto-inject content into conversation at hook events.

**Pattern:** `.claude/context/{name}-{stage}.md`

**Supported hooks:**
- `SlashCommandStart` - Before command executes
- `SlashCommandEnd` - After command completes
- `SkillStart` - When skill loads
- `SkillEnd` - When skill completes

**Examples:**

```bash
# Inject security checklist when code review starts
.claude/context/code-review-start.md

# Inject planning template when /plan runs
.claude/context/plan-start.md

# Inject TDD standards when TDD skill loads
.claude/context/test-driven-development-start.md

# Verify review complete when code review ends
.claude/context/code-review-end.md
```

### 2. Directory Organization

**Small projects (<5 files):**
```
.claude/context/{name}-{stage}.md
```

**Medium projects (5-20 files):**
```
.claude/context/slash-command/{name}-{stage}.md
.claude/context/skill/{name}-{stage}.md
```

**Large projects (>20 files):**
```
.claude/context/slash-command/{name}/{stage}.md
.claude/context/skill/{name}/{stage}.md
```

All structures supported - use what fits your project size.

## Discovery Order

Dispatcher searches paths in priority order:

1. `.claude/context/{name}-{stage}.md`
2. `.claude/context/slash-command/{name}-{stage}.md`
3. `.claude/context/slash-command/{name}/{stage}.md`
4. `.claude/context/skill/{name}-{stage}.md`
5. `.claude/context/skill/{name}/{stage}.md`

First match wins.

## Naming Rules

### Command Names
- Remove leading slash: `/code-review` → `code-review`
- Use exact command name: `/plan` → `plan`
- Lowercase only

### Skill Names
- Use exact skill name (may include hyphens)
- Example: `test-driven-development`
- Example: `conducting-code-review`

### Stage Names
- `start` - Before execution
- `end` - After completion
- Lowercase only

## Content Format

Context files are markdown with any structure:

```markdown
## Project Requirements

List your requirements here.

### Security
- Requirement 1
- Requirement 2

### Performance
- Benchmark targets
- Optimization goals
```

Content appears as `additionalContext` in conversation.

## Execution Model

### Injection Timing

**Before explicit gates:**
```
1. Convention file exists? → Auto-inject
2. Run explicit gates (from gates.json)
3. Continue or block based on results
```

**Example flow for /code-review:**
```
1. SlashCommandStart fires
2. Check for .claude/context/code-review-start.md
3. If exists → inject content
4. Run configured gates (e.g., verify-structure)
5. Continue if all pass
```

### Combining Conventions and Gates

**Zero-config approach:**
```bash
# Just create file - auto-injects!
echo "## Requirements..." > .claude/context/code-review-start.md
```

**Mixed approach:**
```bash
# Convention file for injection
.claude/context/code-review-start.md

# Plus explicit gates for verification
{
  "hooks": {
    "SlashCommandEnd": {
      "enabled_commands": ["/code-review"],
      "gates": ["verify-structure", "test"]
    }
  }
}
```

Execution: Inject context → Run verify-structure → Run test

## Control and Disabling

### Disable Convention

**Method 1: Rename file**
```bash
mv .claude/context/code-review-start.md \
   .claude/context/code-review-start.md.disabled
```

**Method 2: Move to non-discovery path**
```bash
mkdir -p .claude/disabled
mv .claude/context/code-review-start.md .claude/disabled/
```

**Method 3: Delete file**
```bash
rm .claude/context/code-review-start.md
```

No config changes needed - control via file presence.

### Enable Convention

Move/rename file back to discovery path:
```bash
mv .claude/context/code-review-start.md.disabled \
   .claude/context/code-review-start.md
```

## Common Patterns

### Pattern: Review Requirements

**File:** `.claude/context/code-review-start.md`

**Triggered by:** `/code-review` command

**Content example:**
```markdown
## Security Requirements
- Authentication required
- Input validation
- No secrets in logs

## Performance Requirements
- No N+1 queries
- Response time < 200ms
```

### Pattern: Planning Template

**File:** `.claude/context/plan-start.md`

**Triggered by:** `/plan` command

**Content example:**
```markdown
## Plan Structure

Must include:
1. Architecture impact
2. Testing strategy
3. Deployment plan
4. Success criteria
```

### Pattern: Skill Standards

**File:** `.claude/context/test-driven-development-start.md`

**Triggered by:** TDD skill loading

**Content example:**
```markdown
## Project TDD Standards

Framework: Vitest
Location: src/**/__tests__/*.test.ts
Coverage: 80% minimum
```

## Migration from Custom Scripts

**Before (custom script):**
```bash
# .claude/gates/inject-requirements.sh
#!/bin/bash
cat .claude/requirements.md | jq -Rs '{additionalContext: .}'
```

**After (convention):**
```bash
# Just rename/move the file!
mv .claude/requirements.md .claude/context/code-review-start.md
```

Zero scripting needed.

## Best Practices

1. **File Organization:** Start flat, grow hierarchically as needed
2. **Naming:** Use exact command/skill names (case-sensitive stage names)
3. **Content:** Keep focused - one concern per file
4. **Discovery:** Let multiple paths support project evolution
5. **Control:** Rename/move files rather than editing gates.json

## Debugging

**Check if file discovered:**
```bash
export CIPHERPOWERS_HOOK_DEBUG=true
tail -f $TMPDIR/cipherpowers-hooks-$(date +%Y%m%d).log
```

Look for: `"dispatcher: Context file: /path/to/file.md"`

**Common issues:**
- Wrong file name (check exact command/skill name)
- Wrong stage name (must be `start` or `end`, lowercase)
- File not in discovery path (check supported structures)
- Permissions (file must be readable)

## Examples Directory

See `plugin/hooks/examples/context/` for working examples:
- Code review requirements
- Planning templates
- TDD standards

Copy and customize for your project.
EOF
```

**Step 2: Verify file created**

Run: `cat plugin/hooks/CONVENTIONS.md | head -20`
Expected: Title and overview section visible

**Step 3: Commit**

```bash
git add plugin/hooks/CONVENTIONS.md
git commit -m "docs(hooks): add comprehensive conventions guide"
```

---

## Task 8: Update Example gates.json Files

**Files:**
- Modify: `plugin/hooks/examples/strict.json`
- Modify: `plugin/hooks/examples/permissive.json`

**Step 1: Add comment about conventions to strict.json**

Update `plugin/hooks/examples/strict.json`:
```json
{
  "description": "Strict enforcement - block on all failures. Supports convention-based context injection.",
  "comment": "Context files in .claude/context/ auto-inject without configuration. See CONVENTIONS.md",

  "gates": {
    "check": {
      "description": "Run project quality checks (formatting, linting, types)",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run project test suite",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },

  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "mcp__serena__replace_symbol_body"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-engineer", "code-reviewer", "ultrathink-debugger"],
      "gates": ["check", "test"]
    },
    "SlashCommandEnd": {
      "enabled_commands": ["/code-review"],
      "gates": ["test"]
    }
  }
}
```

**Step 2: Add comment to permissive.json**

Update `plugin/hooks/examples/permissive.json`:
```json
{
  "description": "Permissive mode - warn only, never block. Supports convention-based context injection.",
  "comment": "Context files in .claude/context/ auto-inject without configuration. See CONVENTIONS.md",

  "gates": {
    "check": {
      "description": "Run project quality checks",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    },
    "test": {
      "description": "Run project test suite",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  },

  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["code-reviewer"],
      "gates": ["check", "test"]
    }
  }
}
```

**Step 3: Create new example showing convention + gates**

```bash
cat > plugin/hooks/examples/convention-based.json << 'EOF'
{
  "description": "Demonstrates convention-based context injection with explicit gates",
  "comment": "Combines zero-config conventions with explicit verification gates",

  "gates": {
    "test": {
      "description": "Run project test suite",
      "command": "npm test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },

  "hooks": {
    "SlashCommandEnd": {
      "comment": "Convention file .claude/context/code-review-start.md auto-injects if exists",
      "enabled_commands": ["/code-review"],
      "gates": ["test"]
    },
    "SkillStart": {
      "comment": "Convention file .claude/context/test-driven-development-start.md auto-injects",
      "enabled_skills": ["test-driven-development"]
    }
  }
}
EOF
```

**Step 4: Verify files updated**

Run: `cat plugin/hooks/examples/strict.json | grep -i convention`
Expected: Comment line about conventions visible

Run: `cat plugin/hooks/examples/convention-based.json | head -5`
Expected: Description visible

**Step 5: Commit**

```bash
git add plugin/hooks/examples/
git commit -m "docs(hooks): update examples to document convention support"
```

---

## Task 9: Integration Test for Full Workflow

**Files:**
- Create: `plugin/hooks/tests/integration-test-conventions.sh`

**Step 1: Create comprehensive integration test**

```bash
cat > plugin/hooks/tests/integration-test-conventions.sh << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "=== Integration Test: Convention-Based Context Injection ==="

# Setup test environment
TEST_DIR="/tmp/test-convention-integration-$$"
mkdir -p "$TEST_DIR/.claude/context"
trap "rm -rf $TEST_DIR" EXIT

# Test 1: SlashCommandStart with flat structure
echo "Test 1: SlashCommandStart with flat structure"
echo "Security requirements" > "$TEST_DIR/.claude/context/code-review-start.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandStart",
  command: "/code-review",
  user_message: "review code",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "Security requirements"; then
  echo "✓ PASS: Flat structure context injection"
else
  echo "✗ FAIL: Context not injected for flat structure"
  exit 1
fi

# Test 2: SlashCommandEnd with organized structure
echo "Test 2: SlashCommandEnd with organized structure"
mkdir -p "$TEST_DIR/.claude/context/slash-command"
echo "Review complete checklist" > "$TEST_DIR/.claude/context/slash-command/code-review-end.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandEnd",
  command: "/code-review",
  user_message: "done",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "Review complete"; then
  echo "✓ PASS: Organized structure context injection"
else
  echo "✗ FAIL: Context not injected for organized structure"
  exit 1
fi

# Test 3: SkillStart with hierarchical structure
echo "Test 3: SkillStart with hierarchical structure"
mkdir -p "$TEST_DIR/.claude/context/skill/test-driven-development"
echo "TDD standards" > "$TEST_DIR/.claude/context/skill/test-driven-development/start.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SkillStart",
  skill: "test-driven-development",
  user_message: "implement with TDD",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "TDD standards"; then
  echo "✓ PASS: Hierarchical structure context injection"
else
  echo "✗ FAIL: Context not injected for hierarchical structure"
  exit 1
fi

# Test 4: No context file (should not error)
echo "Test 4: No context file exists"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandStart",
  command: "/plan",
  user_message: "create plan",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

# Should exit cleanly without error (may have no output if no gates configured)
if [ $? -eq 0 ]; then
  echo "✓ PASS: No error when context file doesn't exist"
else
  echo "✗ FAIL: Error when context file doesn't exist"
  exit 1
fi

# Test 5: Priority order (flat beats organized)
echo "Test 5: Discovery priority order"
echo "Flat version" > "$TEST_DIR/.claude/context/plan-start.md"
mkdir -p "$TEST_DIR/.claude/context/slash-command"
echo "Organized version" > "$TEST_DIR/.claude/context/slash-command/plan-start.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandStart",
  command: "/plan",
  user_message: "plan",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "Flat version"; then
  echo "✓ PASS: Flat structure has priority"
else
  echo "✗ FAIL: Wrong priority order"
  exit 1
fi

echo ""
echo "=== All Integration Tests Passed ==="
EOF

chmod +x plugin/hooks/tests/integration-test-conventions.sh
```

**Step 2: Run integration test to verify it fails**

Run: `bash plugin/hooks/tests/integration-test-conventions.sh`
Expected: Some tests fail (not all code implemented yet)

**Step 3: After all previous tasks complete, run test again**

Run: `bash plugin/hooks/tests/integration-test-conventions.sh`
Expected: All tests PASS

**Step 4: Commit**

```bash
git add plugin/hooks/tests/integration-test-conventions.sh
git commit -m "test(hooks): add integration tests for convention system"
```

---

## Verification Steps

**After all tasks complete:**

1. **Run all unit tests:**
   ```bash
   bash plugin/hooks/tests/test-context-discovery.sh
   bash plugin/hooks/tests/test-context-injection.sh
   bash plugin/hooks/tests/test-slash-command-hook.sh
   bash plugin/hooks/tests/test-skill-hook.sh
   ```
   Expected: All PASS

2. **Run integration test:**
   ```bash
   bash plugin/hooks/tests/integration-test-conventions.sh
   ```
   Expected: All 5 tests PASS

3. **Manual test with real project:**
   ```bash
   mkdir -p /tmp/test-real/.claude/context
   echo "## Test requirements" > /tmp/test-real/.claude/context/code-review-start.md
   cd /tmp/test-real
   # Trigger hook (requires Claude Code running)
   ```

4. **Check documentation:**
   ```bash
   cat plugin/hooks/README.md | grep "Convention-Based"
   cat plugin/hooks/CONVENTIONS.md | head -20
   ls plugin/hooks/examples/context/
   ```

## Success Criteria

- ✅ Context files auto-discovered in multiple directory structures
- ✅ Content auto-injected before explicit gates
- ✅ SlashCommandStart/End hooks supported
- ✅ SkillStart/End hooks supported
- ✅ No configuration needed (zero-config)
- ✅ Control via file presence/absence (rename to disable)
- ✅ All tests passing
- ✅ Documentation complete with examples

## Notes

- **No `auto_inject_context` config needed** - control via file presence
- **Backward compatible** - existing gates.json files work unchanged
- **Progressive enhancement** - conventions layer on top of existing system
- **Clear precedence** - conventions run before explicit gates (like commands.sh pattern)
- **Flexible organization** - supports flat → hierarchical project growth

## References

- Existing pattern: `plugin/hooks/gates/commands.sh` (context injection model)
- Hook spec: Claude Code hook system documentation
- Testing approach: @`${CLAUDE_PLUGIN_ROOT}/skills/test-driven-development/SKILL.md`
