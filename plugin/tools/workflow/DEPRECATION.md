# Deprecation Notice: Legacy Conditional Syntax

## Timeline

**Current Version:** 0.1.0 (October 2025)
**Deprecation Started:** 0.1.0 (October 2025)
**Removal Planned:** 2.0.0 (Target: Q2 2026)

Legacy arrow-based conditional syntax is deprecated in favor of the simplified Pass/Fail syntax. All legacy syntax will continue to work with deprecation warnings until version 2.0.0.

## What's Deprecated

The following conditional syntaxes are deprecated:

### 1. Exit Code Conditionals

**Legacy (deprecated):**
```markdown
→ Exit 0: Continue
→ Exit ≠ 0: STOP
→ Exit 42: STOP (custom error)
```

**New syntax:**
```markdown
- PASS: CONTINUE
- FAIL: STOP
```

**Rationale:** PASS/FAIL is more intuitive and aligns with common workflow patterns. Exit code checking is now the command's responsibility (e.g., use `test` command or check return codes in scripts).

### 2. Output-Based Conditionals

**Legacy (deprecated):**
```markdown
→ If output empty: STOP
→ If output contains "ERROR": STOP
```

**New syntax:**
Use wrapper scripts that return appropriate exit codes:

```markdown
## 1. Check for errors

```bash
# Wrapper script that checks output and returns exit code
if output contains "ERROR"; then
  exit 1
else
  exit 0
fi
```

- PASS: CONTINUE
- FAIL: STOP errors found
```

**Rationale:** Output inspection belongs in the command logic, not in workflow conditionals. This separates concerns and makes workflows more composable.

### 3. Otherwise Conditional

**Legacy (deprecated):**
```markdown
→ Exit 0: Continue
→ Otherwise: STOP (unexpected exit code)
```

**New syntax:**
```markdown
- PASS: CONTINUE
- FAIL: STOP unexpected exit code
```

**Rationale:** FAIL covers all non-zero exit codes, eliminating the need for Otherwise.

## Migration Guide

### Step 1: Identify Legacy Syntax

Search your workflow files for lines starting with `→` or `->`.

```bash
grep -n '^→\|^->' path/to/workflow.md
```

### Step 2: Replace Exit Code Conditionals

**Before:**
```markdown
# Step 1: Run tests

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
```

**After:**
```markdown
## 1. Run tests

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP fix tests
```

### Step 3: Replace Output-Based Conditionals

**Before:**
```markdown
# Step 1: Check for uncommitted changes

```bash
git status --porcelain
```

→ If output empty: Continue
→ Otherwise: STOP (uncommitted changes)
```

**After:**
```markdown
## 1. Check for uncommitted changes

```bash
# Exit 0 if no changes, 1 if changes exist
[[ -z $(git status --porcelain) ]]
```

- PASS: CONTINUE
- FAIL: STOP uncommitted changes
```

### Step 4: Test Your Workflows

Run your migrated workflows in a test environment:

```bash
workflow path/to/migrated-workflow.md
```

Verify that:
1. PASS conditions trigger on exit code 0
2. FAIL conditions trigger on non-zero exit codes
3. STOP actions halt workflow with correct messages
4. GOTO actions jump to correct steps

## Compatibility Period

During the compatibility period (v0.1.0 → v2.0.0):

✅ Both syntaxes work
✅ Legacy syntax shows deprecation warnings
✅ No runtime behavior changes
⚠️ New workflows should use PASS/FAIL only
⚠️ Existing workflows should be migrated

## After v2.0.0

After version 2.0.0:

❌ Legacy arrow syntax will not parse
❌ Workflows using legacy syntax will fail
✅ Only PASS/FAIL syntax supported

## Need Help?

If you encounter migration issues:

1. **Check the syntax guide:** See README.md for complete Pass/Fail syntax
2. **Review examples:** See `examples/` directory for migration patterns
3. **Report issues:** Open an issue if you find edge cases not covered

## Technical Details

### Deprecated Enum Variants

The following `Conditional` enum variants are deprecated:

- `ExitCode { code: i32, action: Action }`
- `ExitNotZero { action: Action }`
- `OutputEmpty { action: Action }`
- `OutputContains { text: String, action: Action }`
- `Otherwise { action: Action }`

### New Enum Variants

Use these instead:

- `Pass { action: Action }` - Matches exit code 0 (keyword: PASS)
- `Fail { action: Action }` - Matches non-zero exit codes (keyword: FAIL)

### Code Example

If you're programmatically generating workflows, update your code:

**Before:**
```rust
Conditional::ExitCode {
    code: 0,
    action: Action::Continue,
}
```

**After:**
```rust
Conditional::Pass {
    action: Action::Continue,  // Rendered as "PASS: CONTINUE" in markdown
}
```

## Questions?

For questions about this deprecation:

- See README.md for current syntax documentation
- Check examples/ for common migration patterns
- Open an issue for clarification or help with edge cases
