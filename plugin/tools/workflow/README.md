# Workflow Executor

Execute markdown-based workflows with shell commands, conditionals, and interactive prompts.

## Why?

**Single source of truth:** Workflows documented in markdown are both readable AND executable.

**Algorithmic enforcement:** LLMs rationalize under pressure (33% compliance). Deterministic execution prevents skipped steps (100% compliance).

**Simple semantics:** One code block → one exit code → one evaluation. Exit 0 = Pass → Continue. Exit non-zero = Fail → STOP. Override when needed.

**Convention over configuration:** Minimal syntax for common cases. Explicit only when needed.

## Security Warning

**⚠️ IMPORTANT:** Workflow files execute arbitrary shell commands with the same permissions as the user running the workflow tool.

**Only run workflow files you trust.** Malicious workflow files could:
- Delete files
- Exfiltrate data
- Execute arbitrary code
- Modify system state

Treat workflow files like shell scripts - review them before execution.

**Best practices:**
- Version control workflow files in your repository
- Require code review before merging workflow changes
- Do not accept workflows from untrusted sources
- Review workflow markdown before executing

## Installation

When using CipherPowers as a local plugin, the workflow tool will auto-compile
on first use. However, this introduces a ~30 second delay the first time an agent
invokes a workflow, which can disrupt agent execution flow and make it appear
stuck. Pre-building avoids this delay:

```bash
# From CipherPowers repository root
mise run build-workflow
```

This compiles the release binary to `target/release/workflow`.

**Requirements:**
- Rust toolchain 1.70+ (install from https://rustup.rs)
- Cargo (included with Rust)

**For agents:** The workflow tool is accessed via the wrapper script at
`${CLAUDE_PLUGIN_ROOT}plugin/tools/workflow/run`, which handles automatic
compilation if needed.

## Usage

```bash
# Run workflow in enforcement mode (sequential, STOP only)
workflow path/to/workflow.md

# Run workflow in guided mode (enables Continue/GoTo)
workflow --guided path/to/workflow.md

# Validate workflow structure without executing
workflow --validate workflow.md

# Dry run (parse and show steps without executing commands)
workflow --dry-run workflow.md

# List all steps
workflow --list workflow.md
```

## Execution Modes

### Enforcement Mode (default)

Sequential execution with no skipping:
- Steps execute in order (1 → 2 → 3...)
- Only `STOP` conditionals are respected
- `Continue` and `Go to Step X` are ignored
- Automatic progression between steps

Use for algorithmic workflows requiring 100% compliance (e.g., git-commit-algorithm).

```bash
workflow plugin/workflows/git-commit.md
```

### Guided Mode (--guided)

Flexible execution with full control flow:
- All conditionals enabled (Continue, GoTo, STOP)
- Agent/user can adapt based on context
- Workflow serves as guide, not rigid script

Use for repeatable processes where judgment calls are needed (e.g., execute-plan).

```bash
workflow --guided docs/work/2025-10-19-feature/plan.md
```

### Validation Mode (--validate)

Parse and validate workflow structure without execution:

```bash
workflow --validate plugin/workflows/git-commit.md
```

**Validation checks:**
- Exactly one H1 (workflow title)
- All steps use H2 (`##`)
- Sequential numbering (1, 2, 3...)
- GOTO targets exist
- Conditional lists have exactly 2 items (PASS and FAIL)
- No duplicate PASS or FAIL branches
- Keywords are ALLCAPS
- No "Step" keyword in headers

Use after migrating workflows to verify new syntax.

### Dry-Run Mode (--dry-run)

Parse workflow and show execution flow without running commands:

```bash
workflow --dry-run plugin/workflows/git-commit.md
```

**Behavior:**
- Shows commands but doesn't execute them
- Displays prompts (with pause for Enter)
- Follows conditionals assuming success (exit code 0)
- Tests workflow structure before actual execution

### Why Two Modes?

**Enforcement prevents rationalization:**
- Agents can't skip steps under pressure
- Algorithmic decisions execute deterministically
- 100% compliance vs 33% with imperative instructions

**Guided enables flexibility:**
- Agent still uses tool (prevents "I don't need the workflow" rationalization)
- Can adapt to context while following process
- Same workflow syntax works in both modes

## Separation of Concerns

**Workflows define WHAT (the process):**
- Pure markdown describing steps, commands, conditionals
- No opinion on whether execution is enforced or flexible
- Same workflow file works in both modes

**Callers define HOW (the enforcement):**
- Agents/commands/skills decide execution mode
- Enforcement mode (no flag): Use for algorithms requiring 100% compliance
- Guided mode (--guided flag): Use for processes needing flexibility
- The workflow itself remains unchanged

**Example:**
- `test-check-build.md` describes test → check → build steps
- `/commit` calls it in enforcement mode (must pass)
- `/execute` calls it in enforcement mode before completion
- Other contexts could call it in guided mode if appropriate
- Workflow file doesn't change based on caller

## Workflow Syntax

Workflows use simple markdown conventions with clean, minimal syntax.

### Complete Example: Before & After

**Old syntax (verbose):**

```markdown
# Git Commit Readiness

# Step 1: Check for changes

Fail: STOP (nothing to commit)

```bash quiet
check-has-changes
```

# Step 2: Run tests

Pass: Continue
Fail: STOP (fix tests first)

```bash
cargo test
```

# Step 3: Review atomicity

**Prompt:** Are changes focused on single logical change?

Pass: Continue
Fail: Go to Step 5

# Step 4: Commit

```bash
git commit
```

# Step 5: Split changes

**Prompt:** Break into separate commits first.
```

**New syntax (clean):**

```markdown
# Git Commit Readiness

## 1. Check for changes

```bash quiet
check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit

## 2. Run tests

```bash
cargo test
```

- PASS: CONTINUE
- FAIL: STOP fix tests first

## 3. Review atomicity

Are changes focused on single logical change?

- PASS: CONTINUE
- FAIL: GOTO 5

## 4. Commit

```bash
git commit
```

## 5. Split changes

Break into separate commits first.
```

**Key differences:**
1. Headers: `# Step N:` → `## N.` (clean numbered headings)
2. Keywords: `Pass:`/`Fail:` → `PASS:`/`FAIL:` (ALLCAPS)
3. GOTO syntax: `Go to Step 6` → `GOTO 6` (concise)
4. STOP syntax: `STOP (message)` → `STOP message` (no parens)
5. List-based conditionals: `- PASS: ACTION` format
6. Implicit prompts: No `**Prompt:**` prefix needed
7. Implicit defaults: Steps without lists use defaults

### Structure

**Workflow title:**
```markdown
# My Workflow Title
```
Single H1 heading at top (required).

**Steps:**
```markdown
## 1. First step
## 2. Second step
## 3. Third step
```
- Use H2 (`##`) for all steps
- Sequential numbering (1, 2, 3...)
- Flexible separator: `. : - )` or space
- Examples: `## 1. Title` or `## 1: Title` or `## 1 Title`

### Commands (Code Blocks)

One code block per step (enforced):

```markdown
## 1. Run tests

```bash
mise run test
```
```

**Interactive commands work naturally:**

Commands that require user interaction (like `git add -p`, `git commit`, `vim`, etc.) work automatically because all commands inherit stdin/stdout/stderr:

```markdown
## 1. Stage changes interactively

```bash
git add -p
```
```

### Conditionals (PASS/FAIL Lists)

**Convention:**
- Exit code 0 = PASS
- Exit code non-zero = FAIL

**Defaults (implicit):**
- Commands: PASS → CONTINUE, FAIL → STOP
- Prompts: Always CONTINUE

**Override defaults with list:**

```markdown
## 1. Run tests

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP fix tests before committing
```

**Available actions:**
- `CONTINUE` - Proceed to next step
- `STOP message` - Halt with optional message
- `GOTO N` - Jump to step N

**Atomic conditionals principle:**
Either use defaults (no list) OR override both branches (2-item list).

```markdown
# ❌ Invalid (partial override)
- FAIL: STOP

# ✅ Valid (complete override)
- PASS: CONTINUE
- FAIL: STOP fix first

# ✅ Valid (use defaults)
(no list at all)
```

**Flexible separators:**
```markdown
- PASS: CONTINUE  ✅
- PASS CONTINUE   ✅
- PASS - CONTINUE ✅
```

**Complex conditions:** Use wrapper scripts to control exit codes

```markdown
## 1. Check for changes

```bash
mise run check-has-changes  # Returns 0 if changes, 1 if empty
```

- PASS: CONTINUE
- FAIL: STOP nothing to commit
```

### Prompts (Implicit)

No `**Prompt:**` prefix needed - steps without code blocks are prompts:

```markdown
## 2. Verify test coverage

Do ALL new/modified functions have tests?
```

Prompts wait for y/n input. Answering 'n' or Enter stops workflow (exit 2).

### Migration Guide

**Converting old workflows to new syntax:**

1. **Update headers:** `# Step N:` → `## N.`
2. **Update keywords:** `Pass:` → `PASS:`, `Fail:` → `FAIL:`, `Continue` → `CONTINUE`
3. **Update GOTO:** `Go to Step N` → `GOTO N`
4. **Update STOP:** `STOP (message)` → `STOP message`
5. **Update conditionals:** Paragraph format → List syntax (`- PASS: ACTION`)
6. **Remove prompt prefix:** `**Prompt:**` prefix no longer needed
7. **Apply atomic principle:** Either no list (defaults) or 2-item list (both branches)
8. **Validate:** `workflow --validate migrated-file.md`

## Exit Codes

- `0` - Workflow completed successfully
- `1` - Workflow stopped due to STOP action
- `2` - User answered 'no' to prompt
- `3` - No steps found in workflow
- `4` - Execution error (invalid step reference, infinite loop, no matching conditional)

## Integration with CipherPowers

The workflow tool executes algorithmic workflows defined in `plugin/workflows/`:

```bash
workflow plugin/workflows/your-algorithm.md
```

Agents can call this tool directly instead of trying to follow the algorithm themselves.

**Note:** Not all practice files are executable workflows. Practice files are documentation; only files that follow the workflow format (Step N: Description with commands/conditionals) can be executed.

## Design

See implementation plan: `docs/plans/2025-10-19-workflow-executor.md`

## Philosophy

**Automate what can be automated.** Reserve human/LLM judgment for what requires it.

If the logic is algorithmic (binary decisions, shell commands, conditionals), execute it deterministically. Don't ask LLMs to "follow" algorithms - they rationalize under pressure.

Evidence: Algorithmic enforcement achieves 100% compliance vs 0-33% with imperative instructions.

## Features

- ✅ Parse markdown workflows (H1 headers for steps)
- ✅ Execute bash commands with inherited stdio (supports interactive commands)
- ✅ Conditional logic (exit codes based PASS/FAIL evaluation)
- ✅ Actions (Continue, Stop, Go to Step)
- ✅ Interactive prompts (y/n confirmation)
- ✅ Interactive command support (`git add -p`, `git commit`, `vim`, etc.)
- ✅ Step progress indicators (Step 1/5)
- ✅ Infinite loop protection (max iterations = steps × 10)
- ✅ Error handling with helpful messages
- ✅ CLI flags (--list, --dry-run, --validate)
- ✅ Distinct exit codes for different failure modes
- ✅ Two execution modes (enforcement and guided)

## Example Workflows

See `examples/simple.md` for a basic example workflow.

## Limitations

- Prompts with inline markdown (code, emphasis, links) may truncate
- Steps must be numbered sequentially (Step 1, Step 2, ...)
- Only bash commands supported (no other shells)
- No support for environment variable substitution in workflows
- No parallel step execution

## Contributing

This tool is part of the CipherPowers plugin for Claude Code. See `CLAUDE.md` in the repository root for development guidelines.

## License

See repository LICENSE file.
