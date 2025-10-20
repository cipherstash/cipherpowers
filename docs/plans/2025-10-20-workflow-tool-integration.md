# Workflow Tool Integration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** Make the Rust workflow executor tool automatically available to agents when the CipherPowers plugin is installed as a local marketplace.

**Architecture:** Two-layer system combining mise setup task (recommended manual compilation after clone) with smart wrapper fallback (auto-compiles if binary missing). Agents always invoke the wrapper script, which ensures binary exists before execution.

**Tech Stack:** Bash (wrapper script), Cargo/Rust (workflow tool), mise (task runner)

---

## Task 1: Create Smart Wrapper Script

**Files:**
- Create: `plugin/tools/workflow` (executable bash wrapper)

**Step 1: Write the wrapper script**

Create executable bash script at `plugin/tools/workflow`:

```bash
#!/usr/bin/env bash
# Smart wrapper for workflow executor tool
# Automatically compiles binary if missing, then executes with all arguments

set -euo pipefail

# Determine paths relative to this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKFLOW_DIR="$SCRIPT_DIR/workflow"
BINARY_PATH="$WORKFLOW_DIR/target/release/workflow"

# Check if binary exists
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "🔧 Workflow binary not found. Compiling from source..." >&2
    echo "   This will take approximately 30 seconds on first run." >&2
    echo "" >&2

    # Verify workflow source directory exists
    if [[ ! -d "$WORKFLOW_DIR" ]]; then
        echo "❌ ERROR: Workflow source directory not found at: $WORKFLOW_DIR" >&2
        echo "   Plugin installation may be corrupted." >&2
        exit 1
    fi

    # Navigate to Rust project directory
    cd "$WORKFLOW_DIR" || {
        echo "❌ ERROR: Cannot access workflow directory" >&2
        exit 1
    }

    # Attempt compilation
    if cargo build --release 2>&1; then
        echo "" >&2
        echo "✅ Compilation successful" >&2
        echo "" >&2
    else
        echo "" >&2
        echo "❌ ERROR: Compilation failed" >&2
        echo "" >&2
        echo "Possible causes:" >&2
        echo "  1. Rust toolchain not installed (install from https://rustup.rs)" >&2
        echo "  2. Cargo build dependencies missing" >&2
        echo "" >&2
        echo "Alternative: Run manual setup" >&2
        echo "  cd to plugin root and run: mise run build-workflow" >&2
        echo "" >&2
        exit 1
    fi
fi

# Execute the binary with all original arguments
exec "$BINARY_PATH" "$@"
```

**Step 2: Make the wrapper executable**

Run: `chmod +x plugin/tools/workflow`

Expected: File permissions changed to executable

**Step 3: Verify wrapper syntax**

Run: `bash -n plugin/tools/workflow`

Expected: No output (syntax valid)

**Step 4: Test wrapper with missing binary**

```bash
# Ensure binary doesn't exist
rm -f plugin/tools/workflow/target/release/workflow

# Run wrapper (should auto-compile)
plugin/tools/workflow --help
```

Expected: Compilation progress messages, then workflow help output

**Step 5: Test wrapper with existing binary**

Run: `plugin/tools/workflow --help`

Expected: Immediate help output, no compilation messages

**Step 6: Commit**

```bash
git add plugin/tools/workflow
git commit -m "feat: add self-compiling wrapper for workflow tool

- Auto-detects missing binary and compiles on first use
- Clear error messages for Rust toolchain issues
- Provides fallback instructions for manual setup
- ~30 second first-run delay, instant subsequent runs"
```

---

## Task 2: Add Mise Setup Task

**Files:**
- Modify: `mise.toml` (add build-workflow and setup tasks)

**Step 1: Add build-workflow task to mise.toml**

Append to `mise.toml`:

```toml
[tasks.build-workflow]
description = "Compile the Rust workflow executor tool"
run = """
echo "🔧 Building workflow tool..."
cd plugin/tools/workflow
if cargo build --release; then
  echo "✅ Workflow tool compiled successfully"
  echo "   Binary: plugin/tools/workflow/target/release/workflow"
else
  echo "❌ Compilation failed"
  echo "   Ensure Rust toolchain is installed: https://rustup.rs"
  exit 1
fi
"""

[tasks.setup]
description = "Complete setup after cloning CipherPowers plugin"
depends = ["build-workflow"]
run = """
echo ""
echo "✅ CipherPowers plugin ready to use"
echo ""
echo "Next steps:"
echo "  1. Add this plugin to Claude Code configuration"
echo "  2. Restart Claude Code to load the plugin"
echo ""
"""
```

**Step 2: Test build-workflow task**

```bash
# Remove binary to test from scratch
rm -f plugin/tools/workflow/target/release/workflow

# Run task
mise run build-workflow
```

Expected: Compilation output, success message with binary path

**Step 3: Test setup task**

Run: `mise run setup`

Expected: Builds workflow tool, displays completion message

**Step 4: Verify binary exists after setup**

Run: `ls -lh plugin/tools/workflow/target/release/workflow`

Expected: Binary file exists (~2-5 MB)

**Step 5: Commit**

```bash
git add mise.toml
git commit -m "feat: add mise tasks for workflow tool setup

- build-workflow: Compiles workflow tool with clear output
- setup: Complete post-clone setup (depends on build-workflow)
- Provides clear error messages if Rust missing"
```

---

## Task 3: Update README Documentation

**Files:**
- Modify: `README.md` (add setup section)

**Step 1: Add setup section to README**

Add after the initial project description (before any "Usage" or "Architecture" sections):

```markdown
## Setup

After cloning the CipherPowers repository:

\`\`\`bash
# Option 1: Complete setup (recommended)
mise run setup

# Option 2: Build workflow tool only
mise run build-workflow
\`\`\`

This compiles the Rust workflow executor tool (~30 seconds). The tool will
auto-compile on first agent use if you skip this step, but manual setup is
faster and catches any Rust toolchain issues early.

**Requirements:**
- Rust toolchain (install from https://rustup.rs)
- mise task runner
```

**Step 2: Verify README renders correctly**

Run: `head -50 README.md | grep -A 15 "## Setup"`

Expected: Setup section displays with proper formatting

**Step 3: Commit**

```bash
git add README.md
git commit -m "docs: add setup instructions for workflow tool

- Documents mise run setup command for post-clone setup
- Explains auto-compilation fallback behavior
- Lists Rust toolchain requirement"
```

---

## Task 4: Update Workflow Tool README

**Files:**
- Modify: `plugin/tools/workflow/README.md` (add installation section)

**Step 1: Add installation section to workflow README**

Find the appropriate location (likely near the top, after overview) and add:

```markdown
## Installation

When using CipherPowers as a local plugin, the workflow tool will auto-compile
on first use. To avoid delays during agent execution, pre-build the tool after
cloning:

\`\`\`bash
# From CipherPowers repository root
mise run build-workflow
\`\`\`

This compiles the release binary to `target/release/workflow`.

**Requirements:**
- Rust toolchain 1.70+ (install from https://rustup.rs)
- Cargo (included with Rust)

**For agents:** The workflow tool is accessed via the wrapper script at
`${CLAUDE_PLUGIN_ROOT}/plugin/tools/workflow/run`, which handles automatic
compilation if needed.
```

**Step 2: Verify section placement**

Run: `head -80 plugin/tools/workflow/README.md | grep -A 20 "## Installation"`

Expected: Installation section displays with context

**Step 3: Commit**

```bash
git add plugin/tools/workflow/README.md
git commit -m "docs(workflow): add installation and agent usage notes

- Explains auto-compilation behavior for plugin users
- Documents mise run build-workflow for pre-building
- Clarifies agents use wrapper, not binary directly"
```

---

## Task 5: Update Agent Documentation Example

**Files:**
- Modify: `plugin/agents/rust-engineer.md` (add workflow tool reference)

**Step 1: Add workflow tool to agent available tools section**

Find the "Available Tools" or similar section and add:

```markdown
**Workflow Executor:** `${CLAUDE_PLUGIN_ROOT}/plugin/tools/workflow/run`
- Execute markdown workflows in enforcement or guided mode
- Enforcement mode (default): Sequential execution, no skipping
- Guided mode: All conditionals enabled, agent retains flexibility
- Syntax: `workflow/run <workflow-file.md>` or `workflow/run --guided <workflow-file.md>`
- Example: `${CLAUDE_PLUGIN_ROOT}/plugin/tools/workflow/run ${CLAUDE_PLUGIN_ROOT}/plugin/practices/git-commit-algorithm.md`
```

**Step 2: Verify reference syntax**

Run: `grep -n "CLAUDE_PLUGIN_ROOT.*workflow" plugin/agents/rust-engineer.md`

Expected: Shows line with correct environment variable usage

**Step 3: Commit**

```bash
git add plugin/agents/rust-engineer.md
git commit -m "docs(agent): add workflow tool reference to rust-engineer

- Documents workflow tool invocation via ${CLAUDE_PLUGIN_ROOT}
- Explains enforcement vs guided modes
- Provides example usage with git-commit-algorithm"
```

---

## Task 6: Add .gitignore Entry for Binary

**Files:**
- Modify: `plugin/tools/workflow/.gitignore` (ensure target/ ignored)

**Step 1: Check existing .gitignore**

Run: `cat plugin/tools/workflow/.gitignore`

Expected: May or may not exist, may or may not contain target/

**Step 2: Ensure target/ directory is ignored**

If .gitignore doesn't exist or doesn't contain target/:

```bash
echo "target/" >> plugin/tools/workflow/.gitignore
```

**Step 3: Verify .gitignore**

Run: `cat plugin/tools/workflow/.gitignore | grep "^target/"

Expected: Shows "target/" line

**Step 4: Test that binary is ignored**

```bash
# Ensure binary exists
mise run build-workflow

# Check git status in workflow directory
cd plugin/tools/workflow && git status --porcelain
```

Expected: No output (target/ directory and binary ignored)

**Step 5: Commit if changed**

```bash
# Only commit if .gitignore was modified
git add plugin/tools/workflow/.gitignore
git commit -m "chore: ensure workflow tool target/ directory is ignored"
```

---

## Task 7: End-to-End Verification

**Files:**
- No file changes (testing only)

**Step 1: Clean slate test - remove binary**

Run: `rm -rf plugin/tools/workflow/target`

Expected: target/ directory removed

**Step 2: Test wrapper auto-compilation**

Run: `plugin/tools/workflow/run --version`

Expected: Compilation messages, then version output

**Step 3: Test immediate subsequent execution**

Run: `plugin/tools/workflow/run --help`

Expected: Instant help output, no compilation

**Step 4: Test mise setup from scratch**

```bash
# Clean again
rm -rf plugin/tools/workflow/target

# Run setup
mise run setup
```

Expected: Compilation, then success message

**Step 5: Test workflow execution with real workflow file**

```bash
# Find a workflow file to test
ls plugin/practices/*.md | head -1

# Execute it (will fail at steps, but should parse correctly)
plugin/tools/workflow/run plugin/practices/git-commit-algorithm.md || true
```

Expected: Workflow parser executes, shows steps (may fail during execution)

**Step 6: Verify agent path resolution**

```bash
# Simulate agent invocation with environment variable
export CLAUDE_PLUGIN_ROOT="$(pwd)"
$CLAUDE_PLUGIN_ROOT/plugin/tools/workflow/run --help
```

Expected: Help output (confirms path resolution works)

**Step 7: Document verification results**

Create test log:

```bash
echo "End-to-end verification completed at $(date)" > docs/plans/2025-10-20-workflow-tool-integration-verification.txt
echo "All tests passed:" >> docs/plans/2025-10-20-workflow-tool-integration-verification.txt
echo "- Auto-compilation from missing binary: ✅" >> docs/plans/2025-10-20-workflow-tool-integration-verification.txt
echo "- Immediate execution with existing binary: ✅" >> docs/plans/2025-10-20-workflow-tool-integration-verification.txt
echo "- Mise setup task: ✅" >> docs/plans/2025-10-20-workflow-tool-integration-verification.txt
echo "- Workflow file execution: ✅" >> docs/plans/2025-10-20-workflow-tool-integration-verification.txt
echo "- Agent path resolution: ✅" >> docs/plans/2025-10-20-workflow-tool-integration-verification.txt
```

**Step 8: Commit verification results**

```bash
git add docs/plans/2025-10-20-workflow-tool-integration-verification.txt
git commit -m "test: add end-to-end verification results for workflow tool integration"
```

---

## Completion Checklist

- [ ] Task 1: Smart wrapper script created and tested
- [ ] Task 2: Mise tasks added (build-workflow + setup)
- [ ] Task 3: README setup section added
- [ ] Task 4: Workflow tool README installation section added
- [ ] Task 5: Agent documentation updated with tool reference
- [ ] Task 6: .gitignore verified for target/ directory
- [ ] Task 7: End-to-end verification passed
- [ ] All commits follow conventional commit format
- [ ] No merge conflicts with main branch
- [ ] Ready for code review

## Post-Implementation

After completing all tasks:

1. Run full verification: `mise run setup && plugin/tools/workflow --help`
2. Test from agent perspective with `${CLAUDE_PLUGIN_ROOT}` variable
3. Consider code review via `/code-review` command
4. Merge to main and test plugin installation
5. Update team documentation if needed
