# Workflow Guided Mode Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Add `--guided` flag to workflow executor enabling flexible execution with Continue/GoTo support, while default mode enforces sequential execution.

**Architecture:** Extend existing workflow executor with execution mode enum. Default mode (enforcement) only allows STOP conditionals and automatic continuation. Guided mode enables all conditionals including Continue and GoTo. Same workflow syntax works in both modes.

**Tech Stack:** Rust, existing workflow codebase (pulldown-cmark, clap)

---

## Task 1: Add Execution Mode to CLI Args

**Files:**
- Modify: `plugin/tools/workflow/src/main.rs:1183-1205`

**Step 1: Write failing test for guided mode flag**

Add to `plugin/tools/workflow/src/main.rs` integration tests section:

```rust
#[test]
fn test_guided_mode_flag_parsing() {
    // This will compile once we add the flag
    let args = Args::parse_from(vec!["workflow", "--guided", "test.md"]);
    assert!(args.guided);
}
```

**Step 2: Run test to verify it fails**

```bash
cd plugin/tools/workflow
cargo test test_guided_mode_flag_parsing
```

Expected: Compilation error - `guided` field doesn't exist

**Step 3: Add guided flag to Args struct**

In `plugin/tools/workflow/src/main.rs`, update the Args struct (around line 1183):

```rust
#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows", long_about = None)]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,

    /// Enable guided mode (allows Continue/GoTo conditionals)
    #[arg(long)]
    guided: bool,

    /// Show steps without executing
    #[arg(long)]
    dry_run: bool,

    /// Start from specific step number
    #[arg(long)]
    start_step: Option<usize>,

    /// Verbose output
    #[arg(long, short)]
    verbose: bool,

    /// List all steps
    #[arg(long)]
    list: bool,
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_guided_mode_flag_parsing
```

Expected: PASS

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/main.rs
git commit -m "feat(workflow): add --guided CLI flag for flexible execution mode"
```

---

## Task 2: Create ExecutionMode Enum

**Files:**
- Create: `plugin/tools/workflow/src/execution_mode.rs`
- Modify: `plugin/tools/workflow/src/main.rs` (add mod declaration)

**Step 1: Write failing test for execution mode behavior**

Create `plugin/tools/workflow/src/execution_mode.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutionMode {
    /// Sequential execution, STOP only, no skipping
    Enforcement,
    /// Full control flow (Continue, GoTo, STOP)
    Guided,
}

impl ExecutionMode {
    pub fn allows_continue(&self) -> bool {
        unimplemented!()
    }

    pub fn allows_goto(&self) -> bool {
        unimplemented!()
    }

    pub fn allows_stop(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforcement_mode_restrictions() {
        let mode = ExecutionMode::Enforcement;
        assert!(!mode.allows_continue());
        assert!(!mode.allows_goto());
        assert!(mode.allows_stop());
    }

    #[test]
    fn test_guided_mode_allows_all() {
        let mode = ExecutionMode::Guided;
        assert!(mode.allows_continue());
        assert!(mode.allows_goto());
        assert!(mode.allows_stop());
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_enforcement_mode_restrictions
```

Expected: FAIL - `unimplemented!()`

**Step 3: Implement ExecutionMode methods**

Update `plugin/tools/workflow/src/execution_mode.rs`:

```rust
impl ExecutionMode {
    pub fn allows_continue(&self) -> bool {
        match self {
            ExecutionMode::Enforcement => false,
            ExecutionMode::Guided => true,
        }
    }

    pub fn allows_goto(&self) -> bool {
        match self {
            ExecutionMode::Enforcement => false,
            ExecutionMode::Guided => true,
        }
    }

    pub fn allows_stop(&self) -> bool {
        true // Both modes allow STOP
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_enforcement_mode_restrictions
cargo test test_guided_mode_allows_all
```

Expected: Both PASS

**Step 5: Add mod declaration to main.rs**

Add to `plugin/tools/workflow/src/main.rs` (near top with other mod declarations):

```rust
mod execution_mode;
```

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/execution_mode.rs plugin/tools/workflow/src/main.rs
git commit -m "feat(workflow): add ExecutionMode enum to control conditional flow"
```

---

## Task 3: Pass ExecutionMode to WorkflowRunner

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:909-931`
- Modify: `plugin/tools/workflow/src/main.rs:1254` (where runner is created)

**Step 1: Write failing test for runner with execution mode**

Add to `plugin/tools/workflow/src/runner.rs` tests:

```rust
#[test]
fn test_runner_with_enforcement_mode() {
    use crate::execution_mode::ExecutionMode;

    let steps = vec![
        Step {
            number: 1,
            description: "Test".to_string(),
            commands: vec![
                Command {
                    code: "echo 'test'".to_string(),
                    quiet: false,
                }
            ],
            prompts: vec![],
            conditionals: vec![
                Conditional::ExitCode {
                    code: 0,
                    action: Action::Continue, // Should be ignored in enforcement mode
                }
            ],
        },
    ];

    let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let result = runner.run().unwrap();
    assert_eq!(result, ExecutionResult::Success);
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_runner_with_enforcement_mode
```

Expected: Compilation error - `new()` signature doesn't match

**Step 3: Update WorkflowRunner to accept ExecutionMode**

Update `plugin/tools/workflow/src/runner.rs`:

```rust
use crate::models::*;
use crate::executor::{execute_command, CommandOutput};
use crate::execution_mode::ExecutionMode;
use anyhow::Result;

pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
    mode: ExecutionMode,
}

impl WorkflowRunner {
    pub fn new(steps: Vec<Step>, mode: ExecutionMode) -> Self {
        Self {
            steps,
            current_step: 0,
            mode,
        }
    }

    // ... rest of implementation unchanged for now
}
```

**Step 4: Update all existing test calls to WorkflowRunner::new**

Find all tests in `runner.rs` that call `WorkflowRunner::new()` and add `ExecutionMode::Enforcement`:

```rust
// Example from existing test_run_simple_workflow
let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
```

**Step 5: Update main.rs to pass execution mode**

In `plugin/tools/workflow/src/main.rs`, update the runner creation (around line 1254):

```rust
use execution_mode::ExecutionMode;

// ... in main() function
let mode = if args.guided {
    ExecutionMode::Guided
} else {
    ExecutionMode::Enforcement
};

let mut runner = runner::WorkflowRunner::new(steps, mode);
let result = runner.run()?;
```

**Step 6: Run all tests to verify they pass**

```bash
cargo test
```

Expected: All tests PASS

**Step 7: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs plugin/tools/workflow/src/main.rs
git commit -m "feat(workflow): pass ExecutionMode to WorkflowRunner constructor"
```

---

## Task 4: Implement Conditional Filtering in Runner

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:1057-1094` (evaluate_conditionals method)

**Step 1: Write failing test for enforcement mode filtering**

Add to `plugin/tools/workflow/src/runner.rs` tests:

```rust
#[test]
fn test_enforcement_mode_ignores_continue_and_goto() {
    use crate::execution_mode::ExecutionMode;

    let steps = vec![
        Step {
            number: 1,
            description: "Step with Continue".to_string(),
            commands: vec![
                Command {
                    code: "echo 'test'".to_string(),
                    quiet: false,
                }
            ],
            prompts: vec![],
            conditionals: vec![
                Conditional::ExitCode {
                    code: 0,
                    action: Action::Continue, // Should be ignored
                },
                Conditional::ExitCode {
                    code: 1,
                    action: Action::Stop { message: Some("failed".to_string()) },
                }
            ],
        },
        Step {
            number: 2,
            description: "Second step".to_string(),
            commands: vec![],
            prompts: vec![],
            conditionals: vec![],
        },
    ];

    let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let result = runner.run().unwrap();

    // Should complete both steps (Continue was ignored, didn't stop early)
    assert_eq!(result, ExecutionResult::Success);
}

#[test]
fn test_guided_mode_respects_continue() {
    use crate::execution_mode::ExecutionMode;

    let steps = vec![
        Step {
            number: 1,
            description: "Step with Continue".to_string(),
            commands: vec![
                Command {
                    code: "echo 'test'".to_string(),
                    quiet: false,
                }
            ],
            prompts: vec![],
            conditionals: vec![
                Conditional::ExitCode {
                    code: 0,
                    action: Action::Continue,
                }
            ],
        },
    ];

    let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
    let result = runner.run().unwrap();
    assert_eq!(result, ExecutionResult::Success);
}
```

**Step 2: Run tests to verify they fail**

```bash
cargo test test_enforcement_mode_ignores_continue_and_goto
cargo test test_guided_mode_respects_continue
```

Expected: Tests may pass or fail depending on current implementation, but we need to verify correct behavior

**Step 3: Update evaluate_conditionals to respect ExecutionMode**

Update the `evaluate_conditionals` method in `plugin/tools/workflow/src/runner.rs`:

```rust
fn evaluate_conditionals(&self, conditionals: &[Conditional], output: &CommandOutput) -> Result<Option<Action>> {
    for conditional in conditionals {
        let matched_action = match conditional {
            Conditional::ExitCode { code, action } => {
                if output.exit_code == *code {
                    Some(action.clone())
                } else {
                    None
                }
            }
            Conditional::ExitNotZero { action } => {
                if output.exit_code != 0 {
                    Some(action.clone())
                } else {
                    None
                }
            }
            Conditional::OutputEmpty { action } => {
                if output.stdout.trim().is_empty() {
                    Some(action.clone())
                } else {
                    None
                }
            }
            Conditional::OutputContains { text, action } => {
                if output.stdout.contains(text) || output.stderr.contains(text) {
                    Some(action.clone())
                } else {
                    None
                }
            }
            Conditional::Otherwise { action } => Some(action.clone()),
        };

        // If we matched a conditional, check if it's allowed in current mode
        if let Some(action) = matched_action {
            let is_allowed = match &action {
                Action::Continue => self.mode.allows_continue(),
                Action::Stop { .. } => self.mode.allows_stop(),
                Action::GoToStep { .. } => self.mode.allows_goto(),
            };

            if is_allowed {
                return Ok(Some(action));
            } else {
                // In enforcement mode, Continue/GoTo are ignored - continue to next conditional
                println!("→ Conditional matched but ignored in enforcement mode: {:?}", action);
                continue;
            }
        }
    }
    Ok(None)
}
```

**Step 4: Run tests to verify they pass**

```bash
cargo test test_enforcement_mode_ignores_continue_and_goto
cargo test test_guided_mode_respects_continue
cargo test # Run all tests
```

Expected: All tests PASS

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "feat(workflow): filter conditionals based on ExecutionMode

In enforcement mode, Continue and GoTo actions are ignored, only STOP
is respected. In guided mode, all conditionals work as designed."
```

---

## Task 5: Update README Documentation

**Files:**
- Modify: `plugin/tools/workflow/README.md`

**Step 1: Add guided mode to usage section**

Update the Usage section in `plugin/tools/workflow/README.md`:

```markdown
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

# Start from specific step
workflow --start-step 3 workflow.md
```
```

**Step 2: Add execution modes section**

Add new section after Usage:

```markdown
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
```

**Step 3: Update conditionals section**

Update the Conditionals section to explain mode behavior:

```markdown
### Conditionals (Arrow Notation)

```markdown
→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
→ If output empty: STOP (nothing to commit)
→ If output contains "error": STOP
→ Otherwise: Continue
→ Exit 0: Go to Step 5
```

**Enforcement mode:**
- `STOP` conditionals work as written
- `Continue` and `Go to Step X` are ignored (automatic progression)
- Use for algorithmic enforcement

**Guided mode:**
- All conditionals work as written
- Full control flow enabled
- Use when flexibility needed
```

**Step 4: Manual test - verify README renders correctly**

```bash
# Preview with markdown viewer or GitHub
cat plugin/tools/workflow/README.md
```

Expected: Documentation is clear and accurate

**Step 5: Commit**

```bash
git add plugin/tools/workflow/README.md
git commit -m "docs(workflow): document enforcement vs guided execution modes

Explain when to use each mode, how conditionals behave differently,
and why two modes prevent both rationalization risks."
```

---

## Task 6: Integration Testing with Real Workflows

**Files:**
- None (testing only)

**Step 1: Test with git-commit-algorithm in enforcement mode**

```bash
cd plugin/tools/workflow
cargo build --release

# This should execute sequentially, ignoring Continue conditionals
./target/release/workflow --dry-run ../../../plugin/practices/git-commit-algorithm.md
```

Expected: Shows all steps, previews execution

**Step 2: Create test workflow with Continue/GoTo**

Create `/tmp/test-guided.md`:

```markdown
# Step 1: First step

```bash
echo "Step 1"
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP

# Step 2: Second step

```bash
echo "Step 2"
```

→ Exit 0: Go to Step 4

# Step 3: Should be skipped

```bash
echo "This should not run in guided mode"
```

# Step 4: Final step

```bash
echo "Step 4 - done"
```
```

**Step 3: Test in enforcement mode (should execute all steps)**

```bash
./target/release/workflow /tmp/test-guided.md
```

Expected:
- Executes Step 1
- Executes Step 2 (ignores GoTo)
- Executes Step 3
- Executes Step 4
- Success

**Step 4: Test in guided mode (should skip step 3)**

```bash
./target/release/workflow --guided /tmp/test-guided.md
```

Expected:
- Executes Step 1
- Executes Step 2 (GoTo Step 4 works)
- Skips Step 3
- Executes Step 4
- Success

**Step 5: Document any issues found**

If behavior doesn't match expectations, note the issue and add a bug fix task.

**Step 6: No commit needed** (testing only)

---

## Task 7: Add Help Text and Examples

**Files:**
- Modify: `plugin/tools/workflow/src/main.rs:1184`
- Create: `plugin/tools/workflow/examples/enforcement.md`
- Create: `plugin/tools/workflow/examples/guided.md`

**Step 1: Enhance CLI help text**

Update Args struct in `plugin/tools/workflow/src/main.rs`:

```rust
#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows with deterministic control flow")]
#[command(long_about = "Execute markdown workflows in enforcement mode (sequential, STOP only) or guided mode (full control flow). Enforcement prevents rationalization, guided enables flexibility.")]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,

    /// Enable guided mode (allows Continue/GoTo conditionals). Default is enforcement mode (sequential, STOP only).
    #[arg(long, help = "Enable guided mode for flexible execution")]
    guided: bool,

    // ... rest unchanged
}
```

**Step 2: Create enforcement mode example**

Create `plugin/tools/workflow/examples/enforcement.md`:

```markdown
# Enforcement Mode Example

This workflow demonstrates enforcement mode (default). All conditionals except STOP are ignored, ensuring sequential execution.

# Step 1: Check preconditions

```bash
echo "Checking preconditions..."
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (preconditions failed)

# Step 2: Run operation

```bash
echo "Running operation..."
```

**Prompt:** Did the operation complete successfully?

# Step 3: Verify results

```bash
echo "Verifying results..."
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (verification failed)

# Step 4: Complete

```bash
echo "Workflow complete!"
```

## Usage

```bash
# Enforcement mode (default) - executes all 4 steps sequentially
workflow examples/enforcement.md

# Continue conditionals are ignored, STOP conditionals work
# No way to skip steps - perfect for algorithmic enforcement
```
```

**Step 3: Create guided mode example**

Create `plugin/tools/workflow/examples/guided.md`:

```markdown
# Guided Mode Example

This workflow demonstrates guided mode. All conditionals work, enabling flexible control flow.

# Step 1: Check if work needed

```bash quiet
git status --porcelain
```

→ If output empty: STOP (nothing to do)
→ Otherwise: Continue

# Step 2: Quick check

```bash
echo "Running quick check..."
```

→ Exit 0: Go to Step 4
→ Exit ≠ 0: Continue

# Step 3: Detailed analysis

```bash
echo "Running detailed analysis (only if quick check failed)..."
```

# Step 4: Confirm action

**Prompt:** Ready to proceed?

# Step 5: Execute

```bash
echo "Executing action..."
```

## Usage

```bash
# Guided mode - enables all conditionals
workflow --guided examples/guided.md

# Continue, GoTo, and STOP all work
# Flexible control flow while still using the tool
```
```

**Step 4: Test examples**

```bash
cd plugin/tools/workflow
cargo run -- examples/enforcement.md
cargo run -- --guided examples/guided.md
```

Expected: Both examples execute correctly demonstrating their respective modes

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/main.rs plugin/tools/workflow/examples/
git commit -m "docs(workflow): add CLI help text and mode-specific examples

Created examples demonstrating enforcement (sequential) and guided
(flexible) execution modes with clear usage instructions."
```

---

## Task 8: Update Main CipherPowers Documentation

**Files:**
- Modify: `CLAUDE.md` (reference to workflow tool)

**Step 1: Add workflow execution modes to CLAUDE.md**

Find the Algorithmic Workflow Enforcement section in `CLAUDE.md` and update it:

```markdown
## Algorithmic Workflow Enforcement

CipherPowers uses **algorithmic decision trees** instead of imperative instructions for discipline-enforcing workflows. This achieves 0% → 100% compliance improvement under pressure.

**Workflow Executor Tool:** `plugin/tools/workflow`

Executes markdown-based workflows with two modes:

**Enforcement mode (default):**
```bash
workflow path/to/workflow.md
```
- Sequential execution, no skipping
- Only STOP conditionals respected
- 100% compliance for algorithmic tasks
- Use for: git-commit, TDD enforcement, code review triggers

**Guided mode (--guided):**
```bash
workflow --guided path/to/workflow.md
```
- All conditionals enabled (Continue, GoTo, STOP)
- Agent uses tool but retains flexibility
- Prevents "I don't need the workflow" rationalization
- Use for: execute-plan, repeatable processes with judgment calls

**Pattern:** `plugin/skills/workflow/`

**Implemented algorithms:**
1. **Git Commit Readiness** - `plugin/practices/git-commit-algorithm.md`
   - 10-step algorithm: tests pass → checks pass → docs updated → atomic
   - Prevents WIP commits, "will fix later", exhaustion-driven commits

2. **TDD Enforcement** - `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`
   - Prevents code before tests via binary "Does failing test exist?" check
   - Recovery mandates deleting untested code (no sunk cost exceptions)

3. **Code Review Trigger** - `plugin/skills/conducting-code-review/SKILL.md` (Section 1)
   - Requires review before merge via binary commit + review status checks
   - Invalidates "too small", "senior dev", "tests passing" rationalizations

**Why algorithmic?** LLMs treat algorithms as deterministic systems (execute them) but treat imperatives as suggestions (interpret them). Evidence: 33% imperative compliance vs 100% algorithmic compliance in pressure testing.

**Workflow syntax:** Markdown with conventional headers (steps), code blocks (commands), arrows (conditionals), bold (prompts). See `plugin/tools/workflow/README.md` for full syntax.
```

**Step 2: Manual verification**

```bash
# Check documentation renders correctly
cat CLAUDE.md | grep -A 20 "Algorithmic Workflow Enforcement"
```

Expected: Documentation is clear and complete

**Step 3: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with workflow execution modes

Document enforcement vs guided modes, when to use each, and how they
prevent different rationalization risks."
```

---

## Summary

**What we built:**
- `--guided` flag for flexible workflow execution
- ExecutionMode enum controlling conditional behavior
- Enforcement mode: Sequential, STOP only (100% compliance)
- Guided mode: Full control flow (prevents tool avoidance)
- Documentation and examples for both modes

**Testing:**
- Unit tests for ExecutionMode
- Integration tests for both modes
- Manual testing with real workflows
- Examples demonstrating each mode

**Files modified:**
- `plugin/tools/workflow/src/main.rs` - CLI args and mode selection
- `plugin/tools/workflow/src/execution_mode.rs` - New mode enum
- `plugin/tools/workflow/src/runner.rs` - Conditional filtering
- `plugin/tools/workflow/README.md` - Mode documentation
- `plugin/tools/workflow/examples/` - Mode-specific examples
- `CLAUDE.md` - Updated workflow enforcement section

**How to use:**
```bash
# Enforcement (algorithmic compliance)
workflow plugin/practices/git-commit-algorithm.md

# Guided (flexible but still using tool)
workflow --guided docs/work/feature/plan.md
```

**Next steps:**
- Create workflow execution and creation skills (separate plan)
- Convert existing algorithms to workflow format
- Test under pressure scenarios to verify compliance
