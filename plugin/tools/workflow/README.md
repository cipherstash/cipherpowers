# Workflow Executor

Execute markdown-based workflows with shell commands, conditionals, and interactive prompts.

## Why?

Workflows documented in markdown stay readable AND executable. Single source of truth - no duplication between docs and scripts.

**Problem:** LLM agents don't consistently follow algorithmic workflows despite testing.
**Solution:** Execute workflows deterministically - no cognitive load, no rationalization.

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

```bash
cd plugin/tools/workflow
cargo install --path .
```

Or from anywhere in the cipherpowers repo:

```bash
cargo install --path plugin/tools/workflow
```

## Usage

```bash
# Run workflow in enforcement mode (sequential, STOP only)
workflow path/to/workflow.md

# Run workflow in guided mode (enables Continue/GoTo)
workflow --guided path/to/workflow.md

# Dry run (show steps without executing)
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
workflow plugin/practices/git-commit-algorithm.md
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

### Why Two Modes?

**Enforcement prevents rationalization:**
- Agents can't skip steps under pressure
- Algorithmic decisions execute deterministically
- 100% compliance vs 33% with imperative instructions

**Guided enables flexibility:**
- Agent still uses tool (prevents "I don't need the workflow" rationalization)
- Can adapt to context while following process
- Same workflow syntax works in both modes

## Workflow Syntax

Workflows use simple markdown conventions.

### Steps (Headers)

```markdown
# Step 1: Description of step
# Step 2: Another step
```

### Commands (Code Blocks)

One code block per step (enforced):

```markdown
# Step 1: Run tests

```bash
mise run test
```
```

Add `quiet` flag to suppress output on success:

```markdown
```bash quiet
git status --porcelain
```
```

### Conditionals (Pass/Fail Labels)

**New simplified syntax:**

```markdown
# Step 1: Run tests

Fail: STOP (fix tests before committing)

```bash
mise run test
```
```

**Convention:**
- Exit code 0 = Pass
- Exit code non-zero = Fail

**Defaults (implicit):**
- Pass → Continue
- Fail → STOP

**Override when needed:**

```markdown
# Override: allow failure
Fail: Continue

# Override: change success behavior
Pass: Go to Step 5
```

**Available actions:** `Continue`, `STOP`, `STOP (message)`, `Go to Step N`

**Minimal syntax (no conditionals = use defaults):**

```markdown
# Step 1: Run tests

```bash
mise run test
```

# Step 2: Check formatting

```bash
mise run fmt -- --check
```
```

Behavior: Any failure stops workflow automatically.

**Complex conditions:** Use wrapper scripts to control exit codes

```markdown
# Step 1: Check for changes

Fail: STOP (nothing to commit)

```bash
mise run check-has-changes  # Script returns 0 if changes, 1 if empty
```
```

### Prompts (Bold)

```markdown
**Prompt:** Do all functions have tests?
```

Prompts wait for y/n input. Answering 'n' or Enter stops workflow (exit 2).

### Complete Example

```markdown
# Step 1: Check for changes

Fail: STOP (nothing to commit)

```bash quiet
mise run check-has-changes
```

# Step 2: Verify test coverage

**Prompt:** Do ALL new/modified functions have tests?

# Step 3: Run tests

Fail: STOP (fix tests before committing)

```bash
mise run test
```

# Step 4: Check formatting

Fail: STOP (run mise fmt to format code)

```bash quiet
mise run fmt -- --check
```

# Step 5: Commit

```bash
git commit
```
```

## Exit Codes

- `0` - Workflow completed successfully
- `1` - Workflow stopped due to STOP action
- `2` - User answered 'no' to prompt
- `3` - No steps found in workflow
- `4` - Execution error (invalid step reference, infinite loop, no matching conditional)

## Integration with CipherPowers

The workflow tool executes algorithmic workflows defined in `plugin/practices/`:

```bash
workflow plugin/practices/your-algorithm.md
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
- ✅ Execute bash commands with output capture
- ✅ Quiet mode for commands (suppress output on success)
- ✅ Conditional logic (exit codes, output matching, otherwise)
- ✅ Actions (Continue, Stop, Go to Step)
- ✅ Interactive prompts (y/n confirmation)
- ✅ Step progress indicators (Step 1/5)
- ✅ Infinite loop protection (max iterations = steps × 10)
- ✅ Error handling with helpful messages
- ✅ CLI flags (--list, --dry-run)
- ✅ Distinct exit codes for different failure modes

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
