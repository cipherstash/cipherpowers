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
