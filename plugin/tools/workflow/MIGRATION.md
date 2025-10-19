# Migration to Simplified Syntax

## Summary

The workflow tool now uses simplified Pass/Fail syntax instead of verbose arrow-based conditionals.

## What Changed

**Before (verbose):**
```markdown
→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
→ If output empty: STOP (nothing to commit)
```

**After (clean):**
```markdown
Fail: STOP (fix tests)
```

## Key Changes

1. **Exit codes only** - No output-based conditionals
2. **One code block per step** - Enforced constraint
3. **Implicit defaults** - Pass → Continue, Fail → STOP
4. **Simple labels** - Pass:/Fail: instead of arrow syntax
5. **Wrapper scripts** - Complex logic in scripts, not workflow

## Migration Guide

### Replace arrow conditionals

- `→ Exit 0: Continue` → Remove (implicit default)
- `→ Exit ≠ 0: STOP (msg)` → `Fail: STOP (msg)`
- `→ Exit 0: Go to Step N` → `Pass: Go to Step N`

### Handle output checks

Before:
```markdown
→ If output empty: STOP (nothing to commit)
```

After (wrapper script):
```markdown
Fail: STOP (nothing to commit)

```bash
mise run check-has-changes  # Returns 0 if changes exist, 1 if empty
```
```

Or inline:
```markdown
Fail: STOP (nothing to commit)

```bash
git status --porcelain | grep -q .
```
```

### Multiple commands per step

Before (allowed):
```markdown
# Step 1: Multiple commands

```bash
command1
```

```bash
command2
```
```

After (split into steps):
```markdown
# Step 1: First command

```bash
command1
```

# Step 2: Second command

```bash
command2
```
```

## Benefits

- **Cleaner workflows** - Less syntax noise
- **Clear semantics** - One command, one exit code, one evaluation
- **Simpler mental model** - Exit codes only, no special cases
- **Better defaults** - Fail → STOP prevents errors from propagating

## Breaking Changes

- Arrow syntax removed (clean break)
- Output-based conditionals removed (use wrapper scripts)
- Multiple code blocks per step disallowed (enforced)
