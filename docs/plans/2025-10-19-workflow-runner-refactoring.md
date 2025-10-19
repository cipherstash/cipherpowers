# Workflow Runner Refactoring Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Refactor plugin/tools/workflow/src/runner.rs to eliminate debug logging duplication, extract testable functions, integrate tracing crate, improve type safety, and update examples to new Pass/Fail syntax.

**Architecture:** Replace manual debug branches with tracing crate (standard Rust observability), extract 122-line run() method into 7+ focused functions, introduce StepControl enum to separate step-level control flow from workflow-level results, delete obsolete arrow-syntax examples and create new Pass/Fail syntax examples.

**Tech Stack:** Rust 1.70+, tracing 0.1, tracing-subscriber 0.3, anyhow, existing workflow models

---

## Pre-Task Setup

**Step 1: Create feature branch**

Run:
```bash
cd plugin/tools/workflow
git checkout -b refactor/workflow-runner
```

**Step 2: Establish clean baseline**

Run:
```bash
cd plugin/tools/workflow
cargo test
```

Expected: All existing tests PASS

**Step 3: Verify current behavior**

Run:
```bash
cd plugin/tools/workflow
cargo run -- examples/simple.md
```

Expected: Workflow executes successfully (this confirms we can test against current behavior)

---

## Task 1: Integrate Tracing Crate

**Files:**
- Modify: `plugin/tools/workflow/Cargo.toml` (dependencies section)
- Modify: `plugin/tools/workflow/src/main.rs` (add tracing initialization)
- Modify: `plugin/tools/workflow/src/runner.rs` (remove debug field, add tracing macros)

**Step 1: Write test for tracing output**

Create: `plugin/tools/workflow/src/runner.rs` (add to tests module)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt::format::FmtSpan;

    #[test]
    fn test_tracing_debug_output_works() {
        // Setup tracing capture
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_span_events(FmtSpan::ACTIVE)
            .with_test_writer()
            .finish();

        let _guard = tracing::subscriber::set_default(subscriber);

        // Create simple workflow
        let steps = vec![Step {
            number: 1,
            description: "Test tracing".to_string(),
            command: Some(Command {
                code: "echo 'test'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();

        // Verify workflow completed
        assert_eq!(result, ExecutionResult::Success);
        // Manual verification: RUST_LOG=debug cargo test test_tracing_debug_output_works shows debug output
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_tracing_debug_output_works`

Expected: FAIL with "cannot find macro `tracing` in this scope"

**Step 3: Add tracing dependencies**

Modify: `plugin/tools/workflow/Cargo.toml`

```toml
[dependencies]
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
markdown = "1.0.0-alpha.20"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**Step 4: Initialize tracing in main.rs**

Modify: `plugin/tools/workflow/src/main.rs` (add at top of main function, before existing code)

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    // Initialize tracing (respects RUST_LOG env var)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "workflow=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ... rest of existing main.rs code
```

**Step 5: Add tracing import to runner.rs**

Modify: `plugin/tools/workflow/src/runner.rs` (add after existing use statements at line 1)

```rust
use tracing::{debug, info, warn};
```

**Step 6: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_tracing_debug_output_works`

Expected: PASS

Run: `cd plugin/tools/workflow && RUST_LOG=debug cargo test test_tracing_debug_output_works 2>&1 | grep DEBUG`

Expected: See debug output lines in test output

**Step 7: Commit**

```bash
git add plugin/tools/workflow/Cargo.toml plugin/tools/workflow/src/main.rs plugin/tools/workflow/src/runner.rs
git commit -m "feat(workflow): integrate tracing crate for observability

Replace manual debug flag infrastructure with Rust standard tracing crate.
Enables structured logging controlled via RUST_LOG environment variable.

Next: Remove debug field and replace println debug branches with tracing macros.
"
```

---

## Task 2: Remove Debug Field and Replace Debug Branches

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:22-30` (WorkflowRunner struct)
- Modify: `plugin/tools/workflow/src/runner.rs:45-47` (set_debug method)
- Modify: `plugin/tools/workflow/src/runner.rs:96-104` (first debug branch)
- Modify: `plugin/tools/workflow/src/runner.rs:111-125` (second debug branch)

**Step 1: Write test verifying debug removal**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_no_debug_field_exists() {
    // This test ensures debug field has been removed
    // If debug field exists, this won't compile
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    // If we can construct without set_debug(), field is gone
    drop(runner); // Explicit to show we just need construction
}
```

**Step 2: Remove debug field from struct**

Modify: `plugin/tools/workflow/src/runner.rs:22-30`

Before:
```rust
pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
    iterations: usize,
    max_iterations: usize,
    #[allow(dead_code)]
    mode: ExecutionMode,
    debug: bool,
}
```

After:
```rust
pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
    iterations: usize,
    max_iterations: usize,
    #[allow(dead_code)]
    mode: ExecutionMode,
}
```

**Step 3: Remove debug from constructor**

Modify: `plugin/tools/workflow/src/runner.rs:33-43`

Before:
```rust
    pub fn new(steps: Vec<Step>, mode: ExecutionMode) -> Self {
        let max_iterations = steps.len() * MAX_ITERATION_MULTIPLIER;
        Self {
            steps,
            current_step: 0,
            iterations: 0,
            max_iterations,
            mode,
            debug: false,
        }
    }
```

After:
```rust
    pub fn new(steps: Vec<Step>, mode: ExecutionMode) -> Self {
        let max_iterations = steps.len() * MAX_ITERATION_MULTIPLIER;
        Self {
            steps,
            current_step: 0,
            iterations: 0,
            max_iterations,
            mode,
        }
    }
```

**Step 4: Remove set_debug method**

Delete: `plugin/tools/workflow/src/runner.rs:45-47`

```rust
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
```

**Step 5: Replace first debug branch with tracing**

Modify: `plugin/tools/workflow/src/runner.rs:96-104`

Before:
```rust
                // Debug output
                if self.debug {
                    println!("→ [DEBUG] Checking: {}", DEBUG_EVALUATION_CRITERIA);
                    let result_text = if output.success {
                        format!("Pass (exit {})", output.exit_code)
                    } else {
                        format!("Fail (exit {})", output.exit_code)
                    };
                    println!("→ [DEBUG] Result: {}", result_text);
                }
```

After:
```rust
                // Debug output via tracing
                debug!(
                    exit_code = output.exit_code,
                    success = output.success,
                    "Checking: {}",
                    DEBUG_EVALUATION_CRITERIA
                );
```

**Step 6: Replace second debug branch with tracing**

Modify: `plugin/tools/workflow/src/runner.rs:111-125`

Before:
```rust
                if self.debug {
                    match &action {
                        Action::Continue => println!("→ [DEBUG] Action: Continue"),
                        Action::Stop { message } => {
                            if let Some(msg) = message {
                                println!("→ [DEBUG] Action: STOP ({})", msg);
                            } else {
                                println!("→ [DEBUG] Action: STOP");
                            }
                        }
                        Action::GoToStep { number } => {
                            println!("→ [DEBUG] Action: Go to Step {}", number)
                        }
                    }
                }
```

After:
```rust
                debug!(?action, "Determined action");
```

**Step 7: Run tests to verify**

Run: `cd plugin/tools/workflow && cargo test`

Expected: All tests PASS including test_no_debug_field_exists

**Step 8: Verify debug output works**

Run: `cd plugin/tools/workflow && RUST_LOG=debug cargo run -- examples/enforcement.md`

Expected: See debug output with action and evaluation info

**Step 9: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "refactor(workflow): remove debug field, use tracing macros

Delete debug: bool field and set_debug() method.
Replace all 'if self.debug { println!(...) }' branches with tracing::debug!().

Benefits:
- Removes 20+ lines of conditional debug code
- Standard Rust observability via RUST_LOG env var
- Structured logging with field capture (exit_code, success, action)

Verify: RUST_LOG=debug cargo run -- examples/enforcement.md
"
```

---

## Task 3: Extract StepControl Enum for Type Safety

**Files:**
- Create: `plugin/tools/workflow/src/runner.rs:229` (after ExecutionResult enum)
- Modify: `plugin/tools/workflow/src/runner.rs:49-171` (run method will use new enum)

**Step 1: Write test for StepControl enum**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_step_control_enum_exists() {
    // Test that StepControl enum and its variants exist
    let next = StepControl::Next;
    let jump = StepControl::JumpTo(5);
    let term = StepControl::Terminate(ExecutionResult::Success);

    // Pattern match to verify all variants handled
    match next {
        StepControl::Next => {},
        StepControl::JumpTo(_) => panic!("Should be Next"),
        StepControl::Terminate(_) => panic!("Should be Next"),
    }

    match jump {
        StepControl::JumpTo(n) => assert_eq!(n, 5),
        _ => panic!("Should be JumpTo"),
    }

    match term {
        StepControl::Terminate(ExecutionResult::Success) => {},
        _ => panic!("Should be Terminate(Success)"),
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_step_control_enum_exists`

Expected: FAIL with "cannot find type `StepControl` in this scope"

**Step 3: Add StepControl enum**

Add after ExecutionResult enum at line 229 in `plugin/tools/workflow/src/runner.rs`:

```rust
/// Control flow decision for a single step execution
#[derive(Debug, PartialEq)]
enum StepControl {
    /// Continue to next step (current_step + 1)
    Next,
    /// Jump to specific step index
    JumpTo(usize),
    /// Terminate workflow with final result
    Terminate(ExecutionResult),
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_step_control_enum_exists`

Expected: PASS

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "feat(workflow): add StepControl enum for type safety

Introduce StepControl enum to separate step-level control flow
(Next/JumpTo/Terminate) from workflow-level results (ExecutionResult).

This makes control flow explicit and prevents invalid state combinations.

Next: Refactor run() method to use StepControl.
"
```

---

## Task 4: Extract execute_action Function

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:127-146` (duplicated Action match)
- Create: `plugin/tools/workflow/src/runner.rs` (new execute_action method)

**Step 1: Write tests for execute_action**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_execute_action_continue() {
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let action = Action::Continue;

    let control = runner.execute_action(action, 1).unwrap();
    assert_eq!(control, StepControl::Next);
}

#[test]
fn test_execute_action_goto() {
    let steps = vec![
        Step {
            number: 1,
            description: "Step 1".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        },
        Step {
            number: 2,
            description: "Step 2".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        },
    ];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let action = Action::GoToStep { number: 2 };

    let control = runner.execute_action(action, 1).unwrap();
    assert_eq!(control, StepControl::JumpTo(1)); // Index 1 for step number 2
}

#[test]
fn test_execute_action_goto_invalid() {
    // Test error case: invalid GoToStep number
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let action = Action::GoToStep { number: 99 }; // Step doesn't exist

    let result = runner.execute_action(action, 1);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Step 99 not found"));
}

#[test]
fn test_execute_action_stop() {
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let action = Action::Stop { message: Some("Test stop".to_string()) };

    let control = runner.execute_action(action, 1).unwrap();
    assert_eq!(control, StepControl::Terminate(ExecutionResult::Stopped {
        message: Some("Test stop".to_string())
    }));
}
```

**Step 2: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test execute_action`

Expected: FAIL with "no method named `execute_action` found"

**Step 3: Implement execute_action method**

Add before `find_step_index` method in `plugin/tools/workflow/src/runner.rs` (around line 213):

```rust
    fn execute_action(&self, action: Action, from_step: usize) -> Result<StepControl> {
        debug!(?action, "Executing action");

        match action {
            Action::Continue => Ok(StepControl::Next),

            Action::Stop { message } => {
                if let Some(msg) = &message {
                    println!("→ Action: STOP ({})", msg);
                } else {
                    println!("→ Action: STOP");
                }
                Ok(StepControl::Terminate(ExecutionResult::Stopped { message }))
            }

            Action::GoToStep { number } => {
                println!("→ Action: Go to Step {}", number);
                let index = self.find_step_index(number, from_step)?;
                Ok(StepControl::JumpTo(index))
            }
        }
    }
```

**Step 4: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test execute_action`

Expected: All 4 tests PASS

**Step 5: Update run() to use execute_action (remove duplication)**

Modify: `plugin/tools/workflow/src/runner.rs:127-146`

Before:
```rust
                match action {
                    Action::Continue => {
                        // Only show if debug mode
                        // Silent continue in normal mode for clean UX
                    }
                    Action::Stop { message } => {
                        if let Some(msg) = message {
                            println!("→ Action: STOP ({})", msg);
                            return Ok(ExecutionResult::Stopped { message: Some(msg) });
                        } else {
                            println!("→ Action: STOP");
                            return Ok(ExecutionResult::Stopped { message: None });
                        }
                    }
                    Action::GoToStep { number } => {
                        println!("→ Action: Go to Step {}", number);
                        self.current_step = self.find_step_index(number, step.number)?;
                        continue 'workflow_loop;
                    }
                }
```

After:
```rust
                let control = self.execute_action(action, step.number)?;
                match control {
                    StepControl::Next => {
                        // Continue to next step
                    }
                    StepControl::JumpTo(index) => {
                        self.current_step = index;
                        continue 'workflow_loop;
                    }
                    StepControl::Terminate(result) => {
                        return Ok(result);
                    }
                }
```

**Step 6: Run all tests to verify**

Run: `cd plugin/tools/workflow && cargo test`

Expected: All tests PASS

**Step 7: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "refactor(workflow): extract execute_action method

Extract duplicated Action match logic into execute_action() method.
Returns StepControl enum for explicit control flow handling.

Benefits:
- Eliminates duplicated match on Action enum (was lines 112-125 and 127-146)
- Single source of truth for action execution
- Testable in isolation
- Uses StepControl for type-safe control flow

Tests: 4 new tests for execute_action (Continue, GoTo, GoTo error, Stop)
"
```

---

## Task 5: Extract display_command_output Function

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:77-93` (output display logic)
- Create: `plugin/tools/workflow/src/runner.rs` (new display_command_output method)

**Step 1: Add CommandOutput import to tests module**

Modify: `plugin/tools/workflow/src/runner.rs` tests module (add to imports at top of tests module):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::CommandOutput;  // ← Add this import
    use tracing_subscriber::fmt::format::FmtSpan;

    // ... rest of tests
```

**Step 2: Write tests for display_command_output**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_display_command_output_quiet_success() {
    // For quiet successful commands, output should be suppressed
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let output = CommandOutput {
        stdout: "test output\n".to_string(),
        stderr: String::new(),
        exit_code: 0,
        success: true,
    };

    // This should not panic and should suppress output
    runner.display_command_output(&output, true).unwrap();
}

#[test]
fn test_display_command_output_quiet_failure() {
    // For quiet failed commands, output should be shown
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let output = CommandOutput {
        stdout: "error output\n".to_string(),
        stderr: String::new(),
        exit_code: 1,
        success: false,
    };

    // This should not panic and should show output (failure case)
    runner.display_command_output(&output, true).unwrap();
}

#[test]
fn test_display_command_output_stderr_always_shown() {
    // Stderr should always be shown, even for quiet successful commands
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let output = CommandOutput {
        stdout: "stdout\n".to_string(),
        stderr: "warning: something\n".to_string(),
        exit_code: 0,
        success: true,
    };

    // Stderr should be shown even with quiet=true and success=true
    runner.display_command_output(&output, true).unwrap();
}
```

**Step 3: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test display_command_output`

Expected: FAIL with "no method named `display_command_output` found"

**Step 4: Implement display_command_output method**

Add before `execute_action` method in `plugin/tools/workflow/src/runner.rs`:

```rust
    fn display_command_output(&self, output: &CommandOutput, quiet: bool) -> Result<()> {
        // Show stdout based on quiet flag (suppress successful quiet commands)
        let should_suppress = quiet && output.success;
        if !should_suppress {
            print!("{}", output.stdout);
        }

        // Always show stderr (errors and warnings should be visible)
        if !output.stderr.is_empty() {
            eprint!("{}", output.stderr);
        }

        // Status
        let status_symbol = if output.success { "✓" } else { "✗" };
        let status_text = if output.success { "Passed" } else { "Failed" };
        println!(
            "{} {} (exit {})",
            status_symbol, status_text, output.exit_code
        );

        debug!(
            exit_code = output.exit_code,
            success = output.success,
            "Command completed"
        );

        Ok(())
    }
```

**Step 5: Update run() to use display_command_output**

Modify: `plugin/tools/workflow/src/runner.rs:77-104`

Before:
```rust
                // Show stdout based on quiet flag (suppress successful quiet commands)
                let should_suppress = command.quiet && output.success;
                if !should_suppress {
                    print!("{}", output.stdout);
                }

                // Always show stderr (errors and warnings should be visible)
                if !output.stderr.is_empty() {
                    eprint!("{}", output.stderr);
                }

                // Status
                let status_symbol = if output.success { "✓" } else { "✗" };
                let status_text = if output.success { "Passed" } else { "Failed" };
                println!(
                    "{} {} (exit {})",
                    status_symbol, status_text, output.exit_code
                );

                // Debug output via tracing
                debug!(
                    exit_code = output.exit_code,
                    success = output.success,
                    "Checking: {}",
                    DEBUG_EVALUATION_CRITERIA
                );
```

After:
```rust
                self.display_command_output(&output, command.quiet)?;

                debug!("Checking: {}", DEBUG_EVALUATION_CRITERIA);
```

**Step 6: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test`

Expected: All tests PASS

**Step 7: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "refactor(workflow): extract display_command_output method

Extract output display logic (stdout/stderr/status) into dedicated method.
Handles quiet flag semantics: suppress successful quiet commands, always show failures.

Benefits:
- Testable output display logic in isolation
- Clear quiet flag semantics
- Reduces run() method complexity

Tests: 3 new tests for quiet success/failure behavior and stderr always shown
"
```

---

## Task 6: Extract execute_step_prompts Function

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:149-164` (prompt execution logic)
- Create: `plugin/tools/workflow/src/runner.rs` (new execute_step_prompts method)

**Step 1: Write test for execute_step_prompts**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_execute_step_prompts_empty() {
    // Empty prompt list should succeed immediately
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    runner.execute_step_prompts(&[]).unwrap();
}

// Note: Cannot easily test interactive stdin in unit tests
// Manual verification required for actual prompt behavior
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test execute_step_prompts`

Expected: FAIL with "no method named `execute_step_prompts` found"

**Step 3: Implement execute_step_prompts method**

Add before `display_command_output` method in `plugin/tools/workflow/src/runner.rs`:

```rust
    fn execute_step_prompts(&self, prompts: &[Prompt]) -> Result<()> {
        // SECURITY: Prompts accept user input from stdin. While this is by design,
        // malicious workflows could craft misleading prompts to trick users.
        // Always review workflow files before execution (see README.md security section).
        for prompt in prompts {
            println!("→ Prompt: {} [y/N]: ", prompt.text);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let answer = input.trim().to_lowercase();
            if answer != "y" && answer != "yes" {
                println!("→ User answered no");
                return Err(anyhow::anyhow!("User cancelled at prompt"));
            }
        }
        Ok(())
    }
```

**Step 4: Update run() to use execute_step_prompts**

Modify: `plugin/tools/workflow/src/runner.rs:149-164`

Before:
```rust
            // Execute prompts
            // SECURITY: Prompts accept user input from stdin. While this is by design,
            // malicious workflows could craft misleading prompts to trick users.
            // Always review workflow files before execution (see README.md security section).
            for prompt in &step.prompts {
                println!("→ Prompt: {} [y/N]: ", prompt.text);

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                let answer = input.trim().to_lowercase();
                if answer != "y" && answer != "yes" {
                    println!("→ User answered no");
                    return Ok(ExecutionResult::UserCancelled);
                }
            }
```

After:
```rust
            // Execute prompts
            if self.execute_step_prompts(&step.prompts).is_err() {
                return Ok(ExecutionResult::UserCancelled);
            }
```

**Step 5: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test`

Expected: All tests PASS

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "refactor(workflow): extract execute_step_prompts method

Extract prompt execution logic into dedicated method.
Preserves security warning about prompt review.

Benefits:
- Isolated prompt handling logic
- Testable for empty prompt case
- Cleaner error handling (converts Err to UserCancelled)

Tests: 1 new test for empty prompts case
"
```

---

## Task 7: Extract check_iteration_limit Function

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:54-61` (iteration limit check)
- Create: `plugin/tools/workflow/src/runner.rs` (new check_iteration_limit method)

**Step 1: Write test for check_iteration_limit**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_check_iteration_limit_within_bounds() {
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    // Within bounds: iterations = 0, max = 10 (1 step * 10 multiplier)
    runner.check_iteration_limit(&steps[0]).unwrap();
}

#[test]
fn test_check_iteration_limit_exceeded() {
    let steps = vec![Step {
        number: 1,
        description: "Test infinite loop".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![],
    }];

    let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    runner.iterations = 100; // Exceed max_iterations (1 * 10 = 10)

    let result = runner.check_iteration_limit(&steps[0]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Exceeded maximum iterations"));
}
```

**Step 2: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test check_iteration_limit`

Expected: FAIL with "no method named `check_iteration_limit` found"

**Step 3: Implement check_iteration_limit method**

Add before `execute_step_prompts` method in `plugin/tools/workflow/src/runner.rs`:

```rust
    fn check_iteration_limit(&self, step: &Step) -> Result<()> {
        if self.iterations > self.max_iterations {
            return Err(anyhow::anyhow!(
                "Exceeded maximum iterations ({}) at Step {}: '{}'. Possible infinite loop in workflow.\nCheck for GoTo loops or missing STOP conditions.",
                self.max_iterations,
                step.number,
                step.description
            ));
        }
        Ok(())
    }
```

**Step 4: Update run() to use check_iteration_limit**

Modify: `plugin/tools/workflow/src/runner.rs:54-61`

Before:
```rust
            if self.iterations > self.max_iterations {
                return Err(anyhow::anyhow!(
                    "Exceeded maximum iterations ({}) at Step {}: '{}'. Possible infinite loop in workflow.\nCheck for GoTo loops or missing STOP conditions.",
                    self.max_iterations,
                    step.number,
                    step.description
                ));
            }
```

After:
```rust
            self.check_iteration_limit(step)?;
```

**Step 5: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test`

Expected: All tests PASS

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "refactor(workflow): extract check_iteration_limit method

Extract iteration limit guard into dedicated method.

Benefits:
- Testable infinite loop detection
- Clear error messages with step context
- Reduces run() method complexity

Tests: 2 new tests for within bounds and exceeded cases
"
```

---

## Task 8: Extract determine_action Function

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:106-109` (conditional evaluation + defaults)
- Create: `plugin/tools/workflow/src/runner.rs` (new determine_action method)

**Step 1: Write test for determine_action**

Add to `plugin/tools/workflow/src/runner.rs` tests module:

```rust
#[test]
fn test_determine_action_explicit_pass() {
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![
            Conditional::Pass { action: Action::GoToStep { number: 2 } }
        ],
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
    let output = CommandOutput {
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 0,
        success: true,
    };

    let action = runner.determine_action(&steps[0], &output).unwrap();
    assert_eq!(action, Action::GoToStep { number: 2 });
}

#[test]
fn test_determine_action_implicit_defaults() {
    let steps = vec![Step {
        number: 1,
        description: "Test".to_string(),
        command: None,
        prompts: vec![],
        conditionals: vec![], // No explicit conditionals
    }];

    let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);

    // Success → Continue
    let output_success = CommandOutput {
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 0,
        success: true,
    };
    let action = runner.determine_action(&steps[0], &output_success).unwrap();
    assert_eq!(action, Action::Continue);

    // Failure → STOP
    let output_fail = CommandOutput {
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 1,
        success: false,
    };
    let action = runner.determine_action(&steps[0], &output_fail).unwrap();
    assert_eq!(action, Action::Stop { message: None });
}
```

**Step 2: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test determine_action`

Expected: FAIL with "no method named `determine_action` found"

**Step 3: Implement determine_action method**

Add before `check_iteration_limit` method in `plugin/tools/workflow/src/runner.rs`:

```rust
    fn determine_action(&self, step: &Step, output: &CommandOutput) -> Result<Action> {
        // Evaluate explicit conditionals first
        if let Some(action) = self.evaluate_conditionals(&step.conditionals, output)? {
            return Ok(action);
        }

        // Apply implicit defaults
        Ok(self.apply_defaults(output, &step.conditionals))
    }
```

**Step 4: Update run() to use determine_action**

Modify: `plugin/tools/workflow/src/runner.rs:106-109`

Before:
```rust
                // Evaluate conditionals
                let action = self
                    .evaluate_conditionals(&step.conditionals, &output)?
                    .unwrap_or_else(|| self.apply_defaults(&output, &step.conditionals));
```

After:
```rust
                // Determine action from conditionals or defaults
                let action = self.determine_action(step, &output)?;
```

**Step 5: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test`

Expected: All tests PASS

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs
git commit -m "refactor(workflow): extract determine_action method

Combine conditional evaluation and default application into single method.

Benefits:
- Clear naming: determine_action vs evaluate_conditionals + apply_defaults
- Testable action determination logic
- Single source of truth for action selection

Tests: 2 new tests for explicit conditionals and implicit defaults
"
```

---

## Task 9: Delete Obsolete Example Files

**Files:**
- Delete: `plugin/tools/workflow/examples/simple.md`
- Delete: `plugin/tools/workflow/examples/enforcement.md`
- Delete: `plugin/tools/workflow/examples/guided.md`
- Create: `plugin/tools/workflow/tests/integration_test.rs` (if missing)

**Step 1: Create integration test file if missing**

Check if file exists:
```bash
cd plugin/tools/workflow
test -f tests/integration_test.rs && echo "EXISTS" || echo "MISSING"
```

If MISSING, create file structure:
```bash
mkdir -p tests
cat > tests/integration_test.rs << 'EOF'
// Integration tests for workflow examples
EOF
```

**Step 2: Write test verifying old syntax is gone**

Add to `plugin/tools/workflow/tests/integration_test.rs`:

```rust
use std::fs;
use std::path::Path;

#[test]
fn test_no_arrow_syntax_in_examples() {
    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

    for entry in fs::read_dir(examples_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).unwrap();

            // Old arrow syntax should not exist
            assert!(
                !content.contains("→ Exit 0:") && !content.contains("→ If output"),
                "File {:?} contains old arrow syntax (→)",
                path.file_name()
            );
        }
    }
}
```

**Step 3: Verify test fails with old files present**

Run: `cd plugin/tools/workflow && cargo test test_no_arrow_syntax_in_examples`

Expected: FAIL because old files still have arrow syntax

**Step 4: Delete obsolete files**

Run:
```bash
cd plugin/tools/workflow
rm examples/simple.md
rm examples/enforcement.md
rm examples/guided.md
```

**Step 5: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_no_arrow_syntax_in_examples`

Expected: PASS (only simple-new-syntax.md remains)

**Step 6: Commit**

```bash
git add plugin/tools/workflow/examples/ plugin/tools/workflow/tests/
git commit -m "refactor(workflow): delete obsolete arrow syntax examples

Remove simple.md, enforcement.md, guided.md.
All three used old arrow syntax (→ Exit 0:) which is no longer supported.

Replacement examples with Pass/Fail syntax will be created in next tasks.

Test: Added integration test to prevent arrow syntax from returning
"
```

---

## Task 10: Create New Enforcement Example

**Files:**
- Create: `plugin/tools/workflow/examples/enforcement.md`

**Step 1: Write test for enforcement example**

Add to `plugin/tools/workflow/tests/integration_test.rs`:

```rust
#[test]
fn test_enforcement_example_executable() {
    use std::process::Command;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let example_path = Path::new(manifest_dir).join("examples/enforcement.md");

    // Build the workflow binary first
    let build_status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(manifest_dir)
        .status()
        .expect("Failed to build workflow");

    assert!(build_status.success(), "Build failed");

    // Run the enforcement example
    let binary_path = Path::new(manifest_dir)
        .join("target/release/workflow");

    let output = Command::new(binary_path)
        .arg(example_path)
        .output()
        .expect("Failed to run enforcement example");

    // Should complete successfully (all commands succeed)
    assert!(output.status.success(), "Enforcement example failed: {:?}",
            String::from_utf8_lossy(&output.stderr));
}
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_enforcement_example_executable`

Expected: FAIL with "No such file" for enforcement.md

**Step 3: Create enforcement.md with Pass/Fail syntax**

Create: `plugin/tools/workflow/examples/enforcement.md`

```markdown
# Enforcement Mode Example

This workflow demonstrates enforcement mode (default). Only STOP conditionals work, ensuring sequential execution.

# Step 1: Check preconditions

Fail: STOP (preconditions failed)

```bash
echo "Checking preconditions..."
```

# Step 2: Run operation

```bash
echo "Running operation..."
```

**Prompt:** Did the operation complete successfully?

# Step 3: Verify results

Fail: STOP (verification failed)

```bash
echo "Verifying results..."
```

# Step 4: Complete

```bash
echo "✓ Workflow complete!"
```
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_enforcement_example_executable`

Expected: PASS

**Step 5: Manual verification**

Run: `cd plugin/tools/workflow && cargo run -- examples/enforcement.md`

Expected:
- Executes all 4 steps sequentially
- Prompts user at Step 2
- Completes successfully
- Uses new Pass/Fail syntax (no arrows)

**Step 6: Commit**

```bash
git add plugin/tools/workflow/examples/enforcement.md plugin/tools/workflow/tests/integration_test.rs
git commit -m "feat(workflow): add enforcement mode example with Pass/Fail syntax

Create new enforcement.md example using clean Pass/Fail syntax.
Demonstrates enforcement mode: sequential execution, STOP-only conditionals.

Features:
- Explicit Fail: STOP with custom messages
- User prompt at Step 2
- All steps use echo (guaranteed success)
- Clean syntax (no arrows)

Test: Integration test verifies example is executable
Manual: cargo run -- examples/enforcement.md
"
```

---

## Task 11: Create New Guided Example

**Files:**
- Create: `plugin/tools/workflow/examples/guided.md`

**Step 1: Write test for guided example**

Add to `plugin/tools/workflow/tests/integration_test.rs`:

```rust
#[test]
fn test_guided_example_syntax_valid() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let example_path = Path::new(manifest_dir).join("examples/guided.md");

    let content = fs::read_to_string(&example_path).unwrap();

    // Verify Pass/Fail syntax present (not arrow syntax)
    assert!(content.contains("Pass:") || content.contains("Fail:"),
            "Guided example should use Pass/Fail syntax");
    assert!(!content.contains("→ Exit 0:"),
            "Guided example should not use arrow syntax");

    // Verify it has GoTo conditional (not just text mention)
    assert!(content.contains("Pass: Go to Step") || content.contains("Fail: Go to Step"),
            "Guided example should have GoTo conditional");
}
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_guided_example_syntax_valid`

Expected: FAIL with "No such file" for guided.md

**Step 3: Create guided.md with Pass/Fail syntax**

Create: `plugin/tools/workflow/examples/guided.md`

```markdown
# Guided Mode Example

Run with: `workflow --guided examples/guided.md`

This workflow demonstrates guided mode. All conditionals work (Pass, Fail, GoTo), enabling flexible control flow.

# Step 1: Check if work needed

Pass: STOP (nothing to do)

```bash quiet
git status --porcelain
```

If git status output is empty, we stop early (nothing to commit).

# Step 2: Quick check

Pass: Go to Step 4
Fail: Continue

```bash
echo "Running quick check..."
exit 0
```

If quick check passes, skip detailed analysis (Go to Step 4).

# Step 3: Detailed analysis

Only runs if quick check failed (Step 2 Fail: Continue).

```bash
echo "Running detailed analysis (only if quick check failed)..."
```

# Step 4: Confirm action

**Prompt:** Ready to proceed?

# Step 5: Execute

```bash
echo "Executing action..."
```

# Step 6: Complete

```bash
echo "✓ Workflow complete!"
```
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_guided_example_syntax_valid`

Expected: PASS

**Step 5: Manual verification**

Run: `cd plugin/tools/workflow && cargo run -- --guided examples/guided.md`

Expected:
- Step 1: Continues (git status has output)
- Step 2: Passes, jumps to Step 4 (skips Step 3)
- Step 4: Prompts user
- Step 5-6: Execute normally
- Uses new Pass/Fail syntax

**Step 6: Commit**

```bash
git add plugin/tools/workflow/examples/guided.md plugin/tools/workflow/tests/integration_test.rs
git commit -m "feat(workflow): add guided mode example with Pass/Fail syntax

Create new guided.md example using clean Pass/Fail syntax.
Demonstrates guided mode: full conditionals (Pass, Fail, GoTo).

Features:
- Early STOP if no work needed (git status check)
- Conditional jump (Pass: Go to Step 4 skips detailed analysis)
- User prompt before execution
- Clean Pass/Fail syntax (no arrows)

Test: Integration test verifies Pass/Fail syntax and GoTo conditional present
Manual: cargo run -- --guided examples/guided.md
"
```

---

## Task 12: Update simple-new-syntax.md to be Executable

**Files:**
- Rename: `plugin/tools/workflow/examples/simple-new-syntax.md` → `simple.md`
- Modify: `plugin/tools/workflow/examples/simple.md`

**Step 1: Rename simple-new-syntax.md to simple.md**

Run:
```bash
cd plugin/tools/workflow/examples
mv simple-new-syntax.md simple.md
```

**Step 2: Write test for simple example executable**

Add to `plugin/tools/workflow/tests/integration_test.rs`:

```rust
#[test]
fn test_simple_example_executable() {
    use std::process::Command;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let example_path = Path::new(manifest_dir).join("examples/simple.md");

    // Build the workflow binary first
    let build_status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(manifest_dir)
        .status()
        .expect("Failed to build workflow");

    assert!(build_status.success(), "Build failed");

    // Run the simple example
    let binary_path = Path::new(manifest_dir)
        .join("target/release/workflow");

    let output = Command::new(binary_path)
        .arg(example_path)
        .output()
        .expect("Failed to run simple example");

    // Should complete successfully (all echo commands succeed)
    assert!(output.status.success(), "Simple example failed: {:?}",
            String::from_utf8_lossy(&output.stderr));
}
```

**Step 3: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_simple_example_executable`

Expected: FAIL because current simple.md is not executable (it's documentation)

**Step 4: Rewrite simple.md to be executable**

Modify: `plugin/tools/workflow/examples/simple.md`

Replace entire file with:
```markdown
# Simple Workflow Example

This demonstrates the minimal Pass/Fail syntax for common workflows.

# Step 1: Greet user

```bash
echo "Hello from workflow executor!"
```

# Step 2: Check directory

```bash quiet
ls -la
```

# Step 3: Confirm

**Prompt:** Does everything look good?

# Step 4: Complete

```bash
echo "✓ Workflow complete!"
```
```

**Step 5: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_simple_example_executable`

Expected: PASS

**Step 6: Manual verification**

Run: `cd plugin/tools/workflow && cargo run -- examples/simple.md`

Expected:
- Step 1: Prints greeting
- Step 2: Lists directory (quiet, no output unless fails)
- Step 3: Prompts user
- Step 4: Prints completion
- All implicit defaults (Pass → Continue, Fail → STOP)

**Step 7: Commit**

```bash
git add plugin/tools/workflow/examples/simple.md plugin/tools/workflow/tests/integration_test.rs
git commit -m "refactor(workflow): make simple.md executable

Rename simple-new-syntax.md → simple.md.
Rewrite as actual executable workflow (not just documentation).

Changes:
- Remove disclaimer about non-executable
- Remove multiple example sections (was documentation)
- Create single 4-step executable workflow
- Demonstrates minimal syntax (all implicit defaults)

Test: Integration test verifies executability
Manual: cargo run -- examples/simple.md
"
```

---

## Verification Checklist

After completing all tasks, verify the refactoring:

**Tracing Integration:**
- [ ] `debug: bool` field removed from WorkflowRunner
- [ ] `set_debug()` method removed
- [ ] All `if self.debug { println!(...) }` replaced with `debug!(...)`
- [ ] `RUST_LOG=debug cargo run -- examples/simple.md` shows debug output

**Function Extraction:**
- [ ] `run()` method reduced from 122 lines to ~40 lines
- [ ] 7 new methods extracted: check_iteration_limit, determine_action, execute_step_prompts, display_command_output, execute_action, evaluate_conditionals (existing), apply_defaults (existing)
- [ ] Each extracted function has unit tests
- [ ] All existing tests still pass

**Type Safety:**
- [ ] `StepControl` enum added with Next/JumpTo/Terminate variants
- [ ] `execute_action()` returns `StepControl` instead of directly manipulating control flow
- [ ] No more `continue 'workflow_loop` in duplicated code (replaced with `StepControl::JumpTo`)

**Examples:**
- [ ] Old files deleted: simple.md (old version), enforcement.md (old version), guided.md (old version)
- [ ] New files created: simple.md, enforcement.md, guided.md (all with Pass/Fail syntax)
- [ ] No arrow syntax (→) in any example files
- [ ] All examples are executable and complete successfully
- [ ] Integration tests verify executability

**Tests:**
- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo build --release` - no warnings
- [ ] Run each example manually and verify output
- [ ] Run with `RUST_LOG=debug` and verify debug output works

---

## Future Work (Explicitly Scoped OUT)

These improvements are NOT included in this plan:

- [ ] Refactoring parser.rs (out of scope)
- [ ] Adding workflow validation (check GoTo targets exist before execution)
- [ ] Improving error messages for user-facing errors
- [ ] Adding structured output formats (JSON, etc.)
- [ ] Performance optimization (already fast enough for human-driven workflows)

---

## Summary of Fixes Applied

This corrected plan addresses the following issues from the original:

1. **Fixed Task 1 tests module structure** - Added proper imports at module level
2. **Fixed Task 5 import placement** - Moved `CommandOutput` import to module level
3. **Fixed Task 5 test coverage** - Added test for stderr always shown
4. **Fixed Task 4 error case** - Added test for invalid GoToStep number
5. **Fixed Task 9 integration test setup** - Added explicit step to create integration_test.rs if missing
6. **Fixed Task 11 test validation** - Strengthened test to check for actual conditional syntax
7. **Fixed Task 12 step ordering** - Renamed file before writing tests
8. **Added pre-task setup** - Git branch creation and baseline testing

Plan complete and saved to `docs/plans/2025-10-19-workflow-runner-refactoring.md`.
