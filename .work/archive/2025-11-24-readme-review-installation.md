# README Installation Section Review

## Section: Installation (lines 15-52)

### ✅ Validated Claims

**Line 17: "CipherPowers uses Claude Code's marketplace system for plugin installation."**
- VERIFIED: The repository contains `.claude-plugin/marketplace.json` which is the required file for marketplace installation
- VERIFIED: marketplace.json structure is correct with "cipherpowers-dev" as marketplace name and "cipherpowers" as plugin name

**Line 21-26: Installation from GitHub instructions**
- VERIFIED: Claude Code documentation confirms the format for adding GitHub marketplaces
- VERIFIED: The `/plugin install cipherpowers@cipherpowers` syntax is correct (plugin-name@marketplace-name)
- VERIFIED: marketplace.json has `"name": "cipherpowers"` for the plugin

**Line 33-42: Local Development Installation**
- VERIFIED: Instructions are accurate for local marketplace setup
- VERIFIED: The `~/cipherpowers` path is consistent with cloning to any location
- VERIFIED: The `@cipherpowers-dev` suffix matches the marketplace name in marketplace.json ("cipherpowers-dev")

**Line 44-52: Verify Installation**
- VERIFIED: `/brainstorm` and `/code-review` are actual commands that exist in the plugin
- VERIFIED: The troubleshooting note about restarting is reasonable

### ❌ Errors Found

**Line 23: INCORRECT GitHub URL format**
```markdown
Current (INCORRECT):
/plugin marketplace add https://github.com/cipherstash/cipherpowers.git

Should be (CORRECT):
/plugin marketplace add cipherstash/cipherpowers
```

**Evidence:** According to Claude Code documentation at https://code.claude.com/docs/en/plugin-marketplaces:
- GitHub repositories use the shorthand `owner/repo` format
- Full URLs are only required for non-GitHub git services (e.g., GitLab)
- The .git extension is NOT used for GitHub repos

**Line 242: SAME ERROR - Troubleshooting section also uses incorrect URL**
```markdown
Current (INCORRECT):
/plugin marketplace add https://github.com/cipherstash/cipherpowers.git

Should be (CORRECT):
/plugin marketplace add cipherstash/cipherpowers
```

### ⚠️ Unclear/Needs Verification

**Line 46-52: Command availability timing**
- README says "Commands should appear immediately after installation"
- Cannot verify the exact timing behavior without actual Claude Code installation testing
- The fallback of "restart your Claude Code session" seems reasonable

---

## Section: Setup (lines 54-103)

### ✅ Validated Claims

**Line 56-58: Quality Hooks Configuration is optional**
- VERIFIED: Quality hooks are indeed optional based on plugin/hooks/README.md
- VERIFIED: Configuration is project-specific via gates.json

**Line 60-67: ${CLAUDE_PLUGIN_ROOT} discovery**
- VERIFIED: This is a valid approach - CLAUDE.md documents this environment variable
- VERIFIED: The variable is set by Claude Code when plugin loads

**Line 74-76: Example configuration path**
```bash
cp <PLUGIN_PATH>/plugin/hooks/examples/strict.json .claude/gates.json
```
- VERIFIED: strict.json exists at `/Users/tobyhede/src/cipherpowers/plugin/hooks/examples/strict.json`
- VERIFIED: The path structure is correct (plugin/hooks/examples/)

**Line 82-101: Manual configuration example**
- VERIFIED: The JSON structure matches the actual gates.json schema used in strict.json
- VERIFIED: The gate properties (onSuccess, onFailure) match actual implementation
- NOTE: Actual files use "on_pass" and "on_fail" (with underscore), but example shows "onSuccess" and "onFailure"

**Line 103: Reference to SETUP.md**
- VERIFIED: File exists at `/Users/tobyhede/src/cipherpowers/plugin/hooks/SETUP.md`
- VERIFIED: SETUP.md contains detailed configuration guide as claimed

### ❌ Errors Found

**Lines 86-101: Inconsistent property naming**
```json
Current manual example uses:
"onSuccess": "CONTINUE",
"onFailure": "BLOCK"

But actual implementation uses:
"on_pass": "CONTINUE",
"on_fail": "BLOCK"
```

**Evidence:** From `/Users/tobyhede/src/cipherpowers/plugin/hooks/examples/strict.json`:
```json
{
  "gates": {
    "check": {
      "description": "Quality checks must pass",
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

The manual example in README.md uses the wrong property names. Should use `on_pass` and `on_fail`, not `onSuccess` and `onFailure`.

### ⚠️ Unclear/Needs Verification

**Line 74-76: Path reference uses placeholder**
```bash
cp <PLUGIN_PATH>/plugin/hooks/examples/strict.json .claude/gates.json
```
- Uses `<PLUGIN_PATH>` as placeholder
- Could be more explicit about using `${CLAUDE_PLUGIN_ROOT}` which is mentioned in line 67
- Inconsistency: Line 67 suggests asking Claude for the path, but line 76 uses a placeholder

**Recommendation:** Should consistently use either:
1. `${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json` (assumes bash context)
2. Or clearly state "replace <PLUGIN_PATH> with the path from step 1"

---

## Section: Troubleshooting (lines 214-258)

### ✅ Validated Claims

**Lines 216-230: Commands not appearing - verification steps**
- VERIFIED: `/plugin list` is a standard Claude Code command for listing plugins
- VERIFIED: The plugin name "cipherpowers" matches marketplace.json
- VERIFIED: Restart suggestion is reasonable

**Lines 232-244: Plugin installed but commands fail**
- VERIFIED: The diagnosis about "old direct-clone method" is accurate (git history shows migration from direct clone to marketplace in commit 78e1a95)
- VERIFIED: The old paths (`~/.config/claude/plugins/cipherpowers` or `~/.claude/plugins/cipherpowers`) are accurate based on git history

**Lines 246-250: Skills not available**
- VERIFIED: Skills ARE auto-discovered (mentioned in CLAUDE.md and verified in git history)
- VERIFIED: ${CLAUDE_PLUGIN_ROOT} check is valid troubleshooting step

**Lines 252-258: Config directory location**
- VERIFIED: Both `~/.claude/` and `~/.config/claude/` are documented as possible config directories
- VERIFIED: Statement that marketplace handles this automatically is reasonable

### ❌ Errors Found

**Line 227: WRONG plugin name in reinstall command**
```bash
Current (INCORRECT):
/plugin install cipherpowers@cipherpowers-dev

Should be (CORRECT):
/plugin install cipherpowers@cipherpowers
```

**Evidence:**
- Line 26 (Installation section) correctly uses `cipherpowers@cipherpowers`
- The marketplace name is "cipherpowers-dev" (from marketplace.json line 2)
- The plugin name is "cipherpowers" (from marketplace.json line 13)
- For GitHub installation, you use: `plugin-name@marketplace-name`
- Since the GitHub repo is "cipherstash/cipherpowers", when added as marketplace it takes the marketplace name from marketplace.json
- But wait... let me reconsider this...

**CORRECTION TO MY ANALYSIS:**
Actually, this needs deeper analysis:
- When you add a marketplace from GitHub using `cipherstash/cipherpowers`, what name does it get?
- The marketplace.json has `"name": "cipherpowers-dev"`
- So when added, the marketplace is known as "cipherpowers-dev"
- Therefore `cipherpowers@cipherpowers-dev` might be correct for local dev install
- But `cipherpowers@cipherpowers` on line 26 would be WRONG then

**ACTUAL ERROR: Lines 26 vs 227 are inconsistent**
- Line 26 says: `/plugin install cipherpowers@cipherpowers` (for GitHub install)
- Line 227 says: `/plugin install cipherpowers@cipherpowers-dev` (for reinstall)
- These should be the SAME for GitHub marketplace installation
- Line 41 correctly shows: `/plugin install cipherpowers@cipherpowers-dev` (for local dev)

**The confusion:**
- Option 1 (GitHub install) should use: `cipherpowers@cipherpowers` BUT marketplace.json says name is "cipherpowers-dev"
- This is a FUNDAMENTAL PROBLEM: The marketplace.json "name" field doesn't match what should be used for GitHub installation

**Line 242: SAME ERROR as line 23 - incorrect GitHub URL format**
```markdown
Current (INCORRECT):
/plugin marketplace add https://github.com/cipherstash/cipherpowers.git

Should be (CORRECT):
/plugin marketplace add cipherstash/cipherpowers
```

### ⚠️ Unclear/Needs Verification

**Lines 219-222: Plugin name in list**
- README says "CipherPowers should appear in the installed plugins list"
- Unclear if it appears as "CipherPowers", "cipherpowers", "cipherpowers-dev", or something else
- Cannot verify without actual installation testing

**Lines 238-239: Old installation paths**
- Mentions both `~/.config/claude/plugins/cipherpowers` and `~/.claude/plugins/cipherpowers`
- Cannot verify which is correct without checking Claude Code's actual behavior
- Both are plausible based on commit history

---

## Critical Issues Summary

### 1. GitHub Marketplace URL Format (HIGH PRIORITY)

**Lines affected:** 23, 242

**Current:**
```bash
/plugin marketplace add https://github.com/cipherstash/cipherpowers.git
```

**Should be:**
```bash
/plugin marketplace add cipherstash/cipherpowers
```

**Source:** Claude Code documentation at https://code.claude.com/docs/en/plugin-marketplaces states:
- GitHub repos use shorthand `owner/repo` format
- Full URLs only for non-GitHub git services
- No `.git` extension

### 2. Plugin Installation Name Inconsistency (HIGH PRIORITY)

**Lines affected:** 26, 41, 227

**Current state:**
- Line 26 (GitHub install): `cipherpowers@cipherpowers`
- Line 41 (Local dev install): `cipherpowers@cipherpowers-dev`
- Line 227 (Troubleshooting reinstall): `cipherpowers@cipherpowers-dev`

**Problem:**
- marketplace.json has `"name": "cipherpowers-dev"`
- This means when added from GitHub, the marketplace will be known as "cipherpowers-dev"
- Therefore Line 26 should use `cipherpowers@cipherpowers-dev` NOT `cipherpowers@cipherpowers`
- OR marketplace.json should have `"name": "cipherpowers"` for GitHub installation

**Recommendation:**
Either:
1. Change marketplace.json name from "cipherpowers-dev" to "cipherpowers" (simpler for users)
2. Change Line 26 from `cipherpowers@cipherpowers` to `cipherpowers@cipherpowers-dev`

Option 1 is better for user experience - having different names is confusing.

### 3. Gate Property Names (MEDIUM PRIORITY)

**Lines affected:** 86-101

**Current manual example:**
```json
"onSuccess": "CONTINUE",
"onFailure": "BLOCK"
```

**Should be:**
```json
"on_pass": "CONTINUE",
"on_fail": "BLOCK"
```

**Evidence:** All example files in plugin/hooks/examples/*.json use `on_pass` and `on_fail`.

### 4. Placeholder Inconsistency (LOW PRIORITY)

**Lines affected:** 67, 76

Line 67 suggests asking Claude Code for `${CLAUDE_PLUGIN_ROOT}`, but line 76 uses placeholder `<PLUGIN_PATH>`.

**Recommendation:** Use consistent approach, preferably:
```bash
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json .claude/gates.json
```

---

## Additional Files Validated

### Example Gate Configurations
All mentioned example configurations exist and are valid:
- ✅ `plugin/hooks/examples/strict.json` (referenced line 76)
- ✅ `plugin/hooks/examples/permissive.json` (referenced in documentation section)
- ✅ `plugin/hooks/examples/pipeline.json` (referenced in documentation section)

Additional examples found but not mentioned in README:
- `plugin/hooks/examples/convention-based.json`
- `plugin/hooks/examples/plan-execution.json`
- `plugin/hooks/examples/typescript-gates.json`

### Documentation Files
All mentioned documentation files exist:
- ✅ `plugin/hooks/README.md` (line 265)
- ✅ `plugin/hooks/SETUP.md` (line 103, 266)
- ✅ `plugin/hooks/CONVENTIONS.md` (line 267)
- ✅ `plugin/hooks/INTEGRATION_TESTS.md` (line 268)

### Context Examples
All mentioned context examples exist:
- ✅ `plugin/hooks/examples/context/code-review-start.md`
- ✅ `plugin/hooks/examples/context/plan-start.md`
- ✅ `plugin/hooks/examples/context/test-driven-development-start.md`

---

## Recommendations

### Immediate Fixes Required

1. **Fix GitHub marketplace URL format** (lines 23, 242)
   - Change from full URL to `cipherstash/cipherpowers`

2. **Resolve plugin name inconsistency** (lines 26, 41, 227)
   - Either change marketplace.json name to "cipherpowers"
   - Or change line 26 to use @cipherpowers-dev
   - Recommend option 1 for clarity

3. **Fix gate property names** (lines 86-101)
   - Change `onSuccess`/`onFailure` to `on_pass`/`on_fail`

### Nice-to-Have Improvements

1. **Consistent placeholder usage** (line 76)
   - Use `${CLAUDE_PLUGIN_ROOT}` consistently

2. **Add context examples mention**
   - The examples/context/ directory has useful files not prominently mentioned

3. **Clarify plugin list output**
   - Specify exactly what name appears in `/plugin list` output

---

## Testing Verification Needed

The following claims could not be fully verified without actual Claude Code installation:

1. Commands appear "immediately" after installation (line 46)
2. Exact name shown in `/plugin list` output (line 222)
3. Exact config directory location on different systems (lines 254-257)

These appear reasonable but cannot be definitively validated from repository files alone.
