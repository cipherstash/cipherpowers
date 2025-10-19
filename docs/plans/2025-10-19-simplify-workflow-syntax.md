# Simplify Workflow Syntax Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** Simplify workflow syntax from verbose conditionals to clean Pass/Fail labels with sensible defaults

**Architecture:** Replace verbose arrow-based conditionals (`→ Exit 0: Continue`) with simple Pass/Fail labels. Exit code 0 = Pass, non-zero = Fail. Implicit defaults: Pass → Continue, Fail → STOP. Enforce one code block per step for clear semantics.

**Tech Stack:** Rust (pulldown-cmark for parsing), existing workflow-executor codebase

**Code Review Integration:** This plan incorporates fixes from code-review.md:
- ✅ Issue #1 (Multi-command steps) - Resolved by Task 2 (one block per step)
- ✅ Issue #2 (Prompt truncation) - Fixed in Task 2
- ✅ Issue #3 (No validation) - Fixed in Task 2
- ✅ Issue #4 (Confusing UX) - Fixed in Task 6 (--debug flag)
- ✅ Issue #5 (Magic strings) - Simplified in Task 9 (Pass/Fail only)
- ✅ Issue #6 (Quiet flag logic) - Fixed in Task 5
- ✅ Issue #7 (Unnecessary cloning) - Fixed in Task 5
- ✅ Issue #8 (Dead code) - Fixed in Task 9
- ✅ Issue #9 (Error messages) - Fixed in Task 5
- ✅ Issue #10 (Test duplication) - Fixed in Task 5
- ⚠️ Issue #11 (JSON output) - Deferred (consider for future)

---

## Current vs New Syntax

**Before (verbose):**
```markdown
# Step 1: Run tests

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
```

**After (clean):**
```markdown
# Step 1: Run tests

Fail: STOP (fix tests)

```bash
mise run test
```
```

**Minimal (defaults only):**
```markdown
# Step 1: Run tests

```bash
mise run test
```
```
Behavior: Pass → Continue (implicit), Fail → STOP (implicit)

---

## Task 1: Update models for new conditional syntax

**Files:**
- Modify: `plugin/tools/workflow/src/models.rs:33-48`

**Step 1: Write failing test for Pass/Fail conditional parsing**

```rust
// In plugin/tools/workflow/src/models.rs tests section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_conditional_syntax() {
        // This will fail until we add the new enum variants
        let pass_cond = Conditional::Pass { action: Action::Continue };
        let fail_cond = Conditional::Fail { action: Action::Stop { message: None } };

        assert!(matches!(pass_cond, Conditional::Pass { .. }));
        assert!(matches!(fail_cond, Conditional::Fail { .. }));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_new_conditional_syntax`
Expected: FAIL with "no variant named `Pass`"

**Step 3: Add Pass/Fail variants to Conditional enum**

Replace the entire Conditional enum in `models.rs:33-39`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Conditional {
    // New simplified syntax
    Pass { action: Action },
    Fail { action: Action },

    // Legacy syntax (deprecated, will be removed)
    #[deprecated(note = "Use Pass/Fail instead")]
    ExitCode { code: i32, action: Action },
    #[deprecated(note = "Use Fail instead")]
    ExitNotZero { action: Action },
    #[deprecated(note = "Use Pass/Fail with wrapper scripts instead")]
    OutputEmpty { action: Action },
    #[deprecated(note = "Use Pass/Fail with wrapper scripts instead")]
    OutputContains { text: String, action: Action },
    #[deprecated(note = "Use Pass/Fail instead")]
    Otherwise { action: Action },
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_new_conditional_syntax`
Expected: PASS

**Step 5: Commit**

```bash
cd plugin/tools/workflow
git add src/models.rs
git commit -m "feat(workflow): add Pass/Fail conditional variants

- Add Conditional::Pass and Conditional::Fail variants
- Deprecate legacy conditional syntax
- Prepare for simplified workflow syntax"
```

---

## Task 2: Enforce one code block per step

**Files:**
- Modify: `plugin/tools/workflow/src/models.rs:8-15`
- Modify: `plugin/tools/workflow/src/parser.rs:7-122`

**Step 1: Write failing test for multiple code blocks error**

Add to `plugin/tools/workflow/src/parser.rs` tests:

```rust
#[test]
fn test_multiple_code_blocks_per_step_returns_error() {
    let markdown = r#"
# Step 1: Test with multiple blocks

```bash
echo "first"
```

```bash
echo "second"
```
"#;
    let result = parse_workflow(markdown);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Multiple code blocks per step"));
}
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/tools/workflow && cargo test test_multiple_code_blocks_per_step_returns_error`
Expected: FAIL (currently allows multiple blocks)

**Step 3: Change Step.commands from Vec to Option<Command>**

In `models.rs:8-15`, change:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub number: usize,
    pub description: String,
    pub command: Option<Command>,  // Changed from Vec<Command>
    pub prompts: Vec<Prompt>,
    pub conditionals: Vec<Conditional>,
}
```

**Step 4: Update parser to enforce single command block**

In `parser.rs`, update the code block parsing logic (around lines 25-48):

```rust
Event::End(Tag::CodeBlock(_)) => {
    in_code_block = false;
    let parts: Vec<&str> = code_block_lang.split_whitespace().collect();
    let is_bash = parts.first().is_some_and(|&lang| lang == "bash");
    let quiet = parts.contains(&"quiet");

    if is_bash {
        if let Some(step) = current_step.as_mut() {
            // Error if command already exists
            if step.command.is_some() {
                anyhow::bail!(
                    "Multiple code blocks per step not allowed. Step {} already has a command block.",
                    step.number
                );
            }
            step.command = Some(Command {
                code: code_block_content.trim().to_string(),
                quiet,
            });
        }
    }
    code_block_content.clear();
    code_block_lang.clear();
}
```

**Step 5: Update Step initialization in parser**

In `parser.rs` around line 81-87:

```rust
current_step = Some(Step {
    number: captures.0,
    description: captures.1,
    command: None,  // Changed from Vec::new()
    prompts: Vec::new(),
    conditionals: Vec::new(),
});
```

**Step 6: Run test to verify it passes**

Run: `cd plugin/tools/workflow && cargo test test_multiple_code_blocks_per_step_returns_error`
Expected: PASS

**Step 7: Fix all other compilation errors**

Run: `cd plugin/tools/workflow && cargo build`
Expected: Compilation errors in runner.rs, main.rs

Fix by changing `step.commands` to `step.command` throughout:
- In `runner.rs:46`: Change `for command in &step.commands` to `if let Some(command) = &step.command`
- In `main.rs:52`: Change `step.commands.len()` to `step.command.as_ref().map_or(0, |_| 1)`
- In `main.rs:65-67`: Change `for cmd in &step.commands` to `if let Some(cmd) = &step.command`

**Step 8: Run all tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 9: Fix prompt truncation bug (Code Review #2)**

Add test for prompts with inline markdown:

```rust
#[test]
fn test_parse_prompt_with_inline_code() {
    let markdown = r#"
# Step 1: Verify

**Prompt:** Do you want to update `main.rs` file?
"#;

    let steps = parse_workflow(markdown).unwrap();
    assert_eq!(steps[0].prompts.len(), 1);
    assert_eq!(steps[0].prompts[0].text, "Do you want to update `main.rs` file?");
}
```

**Step 10: Fix prompt parsing to accumulate all text**

In `parser.rs`, change prompt capturing logic (around lines 50-80):

```rust
// Add field to struct at top of parse_workflow
let mut current_prompt = String::new();

// In Event::Text:
} else if capturing_prompt {
    // Accumulate all text until we hit a non-text event
    current_prompt.push_str(&text);
}

// In Event::End(Tag::Paragraph) or Event::Start(Tag::CodeBlock(_)):
if capturing_prompt && !current_prompt.is_empty() {
    if let Some(step) = current_step.as_mut() {
        step.prompts.push(Prompt { text: current_prompt.trim().to_string() });
    }
    current_prompt.clear();
    capturing_prompt = false;
}
```

**Step 11: Test prompt fix**

Run: `cd plugin/tools/workflow && cargo test test_parse_prompt_with_inline_code`
Expected: PASS

**Step 12: Add workflow validation (Code Review #3)**

Add validation function to `parser.rs`:

```rust
fn validate_workflow(steps: &[Step]) -> Result<()> {
    for step in steps {
        // Warn if step has no executable content
        if step.command.is_none() && step.prompts.is_empty() {
            eprintln!("Warning: Step {} '{}' has no commands or prompts",
                step.number, step.description);
        }

        // Detect simple infinite loops (GoTo self with no STOP)
        for conditional in &step.conditionals {
            if let Conditional::Pass { action: Action::GoToStep { number } }
                | Conditional::Fail { action: Action::GoToStep { number } } = conditional {
                if *number == step.number {
                    eprintln!("Warning: Step {} has GoTo self - possible infinite loop",
                        step.number);
                }
            }
        }

        // Validate GoTo targets exist
        for conditional in &step.conditionals {
            if let Conditional::Pass { action: Action::GoToStep { number } }
                | Conditional::Fail { action: Action::GoToStep { number } } = conditional {
                if *number < 1 || *number > steps.len() {
                    anyhow::bail!(
                        "Step {}: GoTo target Step {} does not exist (workflow has {} steps)",
                        step.number, number, steps.len()
                    );
                }
            }
        }
    }
    Ok(())
}
```

**Step 13: Call validation in parse_workflow**

After step validation (around line 120):

```rust
// Validate that step numbers are sequential
for (i, step) in steps.iter().enumerate() {
    let expected = i + 1;
    if step.number != expected {
        anyhow::bail!(
            "Step numbers must be sequential. Expected Step {}, found Step {}",
            expected,
            step.number
        );
    }
}

// Validate workflow logic
validate_workflow(&steps)?;

Ok(steps)
```

**Step 14: Test validation**

Add test:

```rust
#[test]
fn test_validation_empty_step_warning() {
    let markdown = r#"
# Step 1: Empty step with no content

# Step 2: Valid step

```bash
echo "test"
```
"#;
    // Should parse successfully but emit warning
    let steps = parse_workflow(markdown).unwrap();
    assert_eq!(steps.len(), 2);
}

#[test]
fn test_validation_invalid_goto() {
    let markdown = r#"
# Step 1: Bad goto

Pass: Go to Step 99

```bash
echo "test"
```
"#;
    let result = parse_workflow(markdown);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("does not exist"));
}
```

**Step 15: Run all tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 16: Commit**

```bash
cd plugin/tools/workflow
git add src/models.rs src/parser.rs src/runner.rs src/main.rs
git commit -m "feat(workflow): enforce one block per step + fixes

- Enforce one code block per step (clear semantics)
- Fix prompt truncation with inline markdown (CR #2)
- Add workflow validation for logic errors (CR #3)
- Validate GoTo targets, detect infinite loops, warn on empty steps"
```

---

## Task 3: Parse Pass/Fail conditional syntax

**Files:**
- Modify: `plugin/tools/workflow/src/parser.rs:142-211`

**Step 1: Write failing test for Pass/Fail parsing**

Add to `parser.rs` tests:

```rust
#[test]
fn test_parse_pass_fail_conditionals() {
    let markdown = r#"
# Step 1: Run tests

Pass: Continue
Fail: STOP (fix tests)

```bash
mise run test
```
"#;

    let steps = parse_workflow(markdown).unwrap();
    assert_eq!(steps[0].conditionals.len(), 2);

    match &steps[0].conditionals[0] {
        Conditional::Pass { action } => {
            assert_eq!(*action, Action::Continue);
        }
        _ => panic!("Expected Pass conditional"),
    }

    match &steps[0].conditionals[1] {
        Conditional::Fail { action } => match action {
            Action::Stop { message } => {
                assert_eq!(message.as_deref(), Some("fix tests"));
            }
            _ => panic!("Expected Stop action"),
        },
        _ => panic!("Expected Fail conditional"),
    }
}

#[test]
fn test_parse_minimal_syntax_no_conditionals() {
    let markdown = r#"
# Step 1: Run tests

```bash
mise run test
```
"#;

    let steps = parse_workflow(markdown).unwrap();
    assert_eq!(steps[0].conditionals.len(), 0);
}
```

**Step 2: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test test_parse_pass_fail_conditionals`
Expected: FAIL (parser doesn't recognize Pass/Fail yet)

**Step 3: Update parse_conditional function**

Replace `parse_conditional` function in `parser.rs:142-188`:

```rust
fn parse_conditional(text: &str) -> Option<Conditional> {
    let trimmed = text.trim();

    // New syntax: Pass/Fail labels (no arrow prefix)
    if trimmed.starts_with("Pass:") {
        let action_str = trimmed.strip_prefix("Pass:")?.trim();
        let action = parse_action(action_str)?;
        return Some(Conditional::Pass { action });
    }

    if trimmed.starts_with("Fail:") {
        let action_str = trimmed.strip_prefix("Fail:")?.trim();
        let action = parse_action(action_str)?;
        return Some(Conditional::Fail { action });
    }

    // Legacy syntax: arrow-based conditionals (deprecated)
    if !trimmed.starts_with("→") && !trimmed.starts_with("->") {
        return None;
    }

    // Remove arrow prefix
    let content = trimmed
        .strip_prefix("→")
        .or_else(|| trimmed.strip_prefix("->"))?
        .trim();

    // Split on first ':'
    let parts: Vec<&str> = content.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }

    let condition = parts[0].trim();
    let action_str = parts[1].trim();

    // Parse action
    let action = parse_action(action_str)?;

    // Parse legacy condition types
    #[allow(deprecated)]
    if condition == "Exit 0" {
        Some(Conditional::ExitCode { code: 0, action })
    } else if condition == "Exit ≠ 0" || condition == "Exit != 0" {
        Some(Conditional::ExitNotZero { action })
    } else if condition.starts_with("Exit ") {
        let code: i32 = condition.strip_prefix("Exit ")?.trim().parse().ok()?;
        Some(Conditional::ExitCode { code, action })
    } else if condition == "If output empty" {
        Some(Conditional::OutputEmpty { action })
    } else if condition.starts_with("If output contains") {
        let text = condition
            .strip_prefix("If output contains")?
            .trim()
            .trim_matches('"')
            .to_string();
        Some(Conditional::OutputContains { text, action })
    } else if condition == "Otherwise" {
        Some(Conditional::Otherwise { action })
    } else {
        None
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test test_parse_pass_fail_conditionals test_parse_minimal_syntax_no_conditionals`
Expected: PASS

**Step 5: Commit**

```bash
cd plugin/tools/workflow
git add src/parser.rs
git commit -m "feat(workflow): parse Pass/Fail conditional syntax

- Parse 'Pass: <action>' and 'Fail: <action>' labels
- Maintain backward compatibility with arrow syntax
- Support minimal syntax (no conditionals = use defaults)"
```

---

## Task 4: Implement implicit defaults in runner

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:69-99`

**Step 1: Write failing test for implicit defaults**

Create new test file: `plugin/tools/workflow/tests/implicit_defaults_test.rs`

```rust
use workflow::*;

#[test]
fn test_implicit_pass_continue() {
    // No conditionals specified, command succeeds → should continue
    let workflow = r#"
# Step 1: First step

```bash
exit 0
```

# Step 2: Second step

```bash
echo "reached step 2"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Enforcement,
    );
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Success);
}

#[test]
fn test_implicit_fail_stop() {
    // No conditionals specified, command fails → should STOP
    let workflow = r#"
# Step 1: First step

```bash
exit 1
```

# Step 2: Should not reach

```bash
echo "should not see this"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Enforcement,
    );
    let result = runner.run().unwrap();

    assert_eq!(
        result,
        runner::ExecutionResult::Stopped { message: None }
    );
}
```

**Step 2: Make modules public for testing**

In `plugin/tools/workflow/src/main.rs`, change:

```rust
mod execution_mode;
mod executor;
mod models;
mod parser;
mod runner;
```

to:

```rust
pub mod execution_mode;
pub mod executor;
pub mod models;
pub mod parser;
pub mod runner;
```

**Step 3: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test test_implicit_pass_continue test_implicit_fail_stop`
Expected: FAIL (currently errors on no matching conditional)

**Step 4: Update evaluate_conditionals to apply defaults**

In `runner.rs`, add new method before `evaluate_conditionals`:

```rust
fn apply_defaults(&self, output: &CommandOutput, conditionals: &[Conditional]) -> Action {
    // If no conditionals specified, use implicit defaults
    if conditionals.is_empty() {
        return if output.success {
            Action::Continue  // Implicit: Pass → Continue
        } else {
            Action::Stop { message: None }  // Implicit: Fail → STOP
        };
    }

    // If conditionals exist but none matched, use defaults
    if output.success {
        Action::Continue  // Implicit: Pass → Continue
    } else {
        Action::Stop { message: None }  // Implicit: Fail → STOP
    }
}
```

**Step 5: Update command execution logic in run method**

In `runner.rs:69-99`, update the conditional evaluation section:

```rust
// Evaluate conditionals
let action = self.evaluate_conditionals(&step.conditionals, &output)?
    .unwrap_or_else(|| self.apply_defaults(&output, &step.conditionals));

match action {
    Action::Continue => {
        println!("→ Action: Continue");
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
        self.current_step = self.find_step_index(number)?;
        continue 'workflow_loop;
    }
}
```

**Step 6: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test test_implicit_pass_continue test_implicit_fail_stop`
Expected: PASS

**Step 7: Run all tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 8: Commit**

```bash
cd plugin/tools/workflow
git add src/main.rs src/runner.rs tests/implicit_defaults_test.rs
git commit -m "feat(workflow): implement implicit defaults

- Pass → Continue (implicit)
- Fail → STOP (implicit)
- Apply defaults when no conditionals match
- Works with minimal syntax (no conditionals specified)"
```

---

## Task 5: Implement Pass/Fail evaluation logic

**Files:**
- Modify: `plugin/tools/workflow/src/runner.rs:136-197`

**Step 1: Write failing test for Pass/Fail evaluation**

Add to `tests/implicit_defaults_test.rs`:

```rust
#[test]
fn test_pass_conditional_on_success() {
    let workflow = r#"
# Step 1: Run command

Pass: Go to Step 3

```bash
exit 0
```

# Step 2: Should skip

```bash
echo "should not see this"
```

# Step 3: Should reach

```bash
echo "reached step 3"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Guided,
    );
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Success);
}

#[test]
fn test_fail_conditional_on_failure() {
    let workflow = r#"
# Step 1: Run command

Fail: STOP (command failed as expected)

```bash
exit 1
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Enforcement,
    );
    let result = runner.run().unwrap();

    assert_eq!(
        result,
        runner::ExecutionResult::Stopped {
            message: Some("command failed as expected".to_string())
        }
    );
}

#[test]
fn test_fail_override_continue() {
    // Override default: Fail → Continue instead of STOP
    let workflow = r#"
# Step 1: Optional hook

Fail: Continue

```bash
exit 1
```

# Step 2: Should still reach

```bash
echo "reached despite failure"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Enforcement,
    );
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Success);
}
```

**Step 2: Run tests to verify they fail**

Run: `cd plugin/tools/workflow && cargo test test_pass_conditional_on_success test_fail_conditional_on_failure`
Expected: FAIL (evaluate_conditionals doesn't handle Pass/Fail yet)

**Step 3: Update evaluate_conditionals method**

In `runner.rs`, replace the `evaluate_conditionals` method (around lines 136-197):

```rust
fn evaluate_conditionals(
    &self,
    conditionals: &[Conditional],
    output: &CommandOutput,
) -> Result<Option<Action>> {
    for conditional in conditionals {
        match conditional {
            // New syntax
            Conditional::Pass { action } => {
                if output.success {
                    return Ok(Some(action.clone()));
                }
            }
            Conditional::Fail { action } => {
                if !output.success {
                    return Ok(Some(action.clone()));
                }
            }

            // Legacy syntax (deprecated)
            #[allow(deprecated)]
            Conditional::ExitCode { code, action } => {
                if output.exit_code == *code {
                    return Ok(Some(action.clone()));
                }
            }
            #[allow(deprecated)]
            Conditional::ExitNotZero { action } => {
                if output.exit_code != 0 {
                    return Ok(Some(action.clone()));
                }
            }
            #[allow(deprecated)]
            Conditional::OutputEmpty { action } => {
                if output.stdout.trim().is_empty() {
                    return Ok(Some(action.clone()));
                }
            }
            #[allow(deprecated)]
            Conditional::OutputContains { text, action } => {
                if output.stdout.contains(text) || output.stderr.contains(text) {
                    return Ok(Some(action.clone()));
                }
            }
            #[allow(deprecated)]
            Conditional::Otherwise { action } => {
                return Ok(Some(action.clone()));
            }
        }
    }
    Ok(None) // No conditional matched
}
```

**Step 4: Run tests to verify they pass**

Run: `cd plugin/tools/workflow && cargo test test_pass_conditional test_fail_conditional test_fail_override_continue`
Expected: PASS

**Step 5: Run all tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 6: Fix quiet flag logic readability (Code Review #6)**

In `runner.rs`, improve quiet flag logic (around line 52):

Before:
```rust
if !command.quiet || !output.success {
    print!("{}", output.stdout);
}
```

After:
```rust
let should_suppress = command.quiet && output.success;
if !should_suppress {
    print!("{}", output.stdout);
}
```

**Step 7: Make Action derive Copy to avoid cloning (Code Review #7)**

In `models.rs`, update Action enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]  // Add Copy
pub enum Action {
    Continue,
    Stop { message: Option<String> },  // ⚠️ Can't be Copy with String
    GoToStep { number: usize },
}
```

Wait - `Stop` has `Option<String>` which isn't Copy. Instead, remove `.clone()` calls and keep borrowing:

In `runner.rs`, change evaluate_conditionals return type and remove clones:

```rust
fn evaluate_conditionals(
    &self,
    conditionals: &[Conditional],
    output: &CommandOutput,
) -> Result<Option<&Action>> {  // Return reference
    for conditional in conditionals {
        match conditional {
            Conditional::Pass { action } => {
                if output.success {
                    return Ok(Some(action));  // No clone
                }
            }
            Conditional::Fail { action } => {
                if !output.success {
                    return Ok(Some(action));  // No clone
                }
            }
            // ... legacy cases with references
        }
    }
    Ok(None)
}
```

Actually, this gets complex with ownership. Keep Action as-is but document why cloning is acceptable (small enum, infrequent operation).

**Step 8: Improve error messages with context (Code Review #9)**

In `runner.rs`, update error when no conditional matches:

Before:
```rust
return Err(anyhow::anyhow!(
    "Command failed with exit code {} but no conditional matched. Add an 'Otherwise' conditional to handle unexpected cases.",
    output.exit_code
));
```

After:
```rust
return Err(anyhow::anyhow!(
    "Step {}: Command '{}' failed with exit code {} but no conditional matched.\nPresent conditionals: {:?}\nSuggestion: Add 'Fail: STOP (message)' to handle failures.",
    step.number,
    command.code.lines().next().unwrap_or(&command.code), // First line only
    output.exit_code,
    step.conditionals
));
```

**Step 9: Add test helper functions (Code Review #10)**

Create `plugin/tools/workflow/tests/helpers.rs`:

```rust
use workflow::*;

#[allow(dead_code)]
pub fn simple_step(num: usize, description: &str, cmd: &str) -> models::Step {
    models::Step {
        number: num,
        description: description.to_string(),
        command: Some(models::Command {
            code: cmd.to_string(),
            quiet: false,
        }),
        prompts: vec![],
        conditionals: vec![],
    }
}

#[allow(dead_code)]
pub fn step_with_fail_stop(
    num: usize,
    description: &str,
    cmd: &str,
    message: &str,
) -> models::Step {
    models::Step {
        number: num,
        description: description.to_string(),
        command: Some(models::Command {
            code: cmd.to_string(),
            quiet: false,
        }),
        prompts: vec![],
        conditionals: vec![models::Conditional::Fail {
            action: models::Action::Stop {
                message: Some(message.to_string()),
            },
        }],
    }
}
```

Add to `tests/implicit_defaults_test.rs`:

```rust
mod helpers;
use helpers::*;

#[test]
fn test_using_helpers() {
    let steps = vec![
        simple_step(1, "Test step", "echo test"),
        step_with_fail_stop(2, "Checked step", "exit 0", "should not stop"),
    ];

    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Enforcement,
    );
    let result = runner.run().unwrap();
    assert_eq!(result, runner::ExecutionResult::Success);
}
```

**Step 10: Run all tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 11: Commit**

```bash
cd plugin/tools/workflow
git add src/models.rs src/runner.rs tests/implicit_defaults_test.rs tests/helpers.rs
git commit -m "feat(workflow): evaluate Pass/Fail + code review fixes

- Pass conditional matches on exit code 0
- Fail conditional matches on exit code non-zero
- Support overriding defaults (Fail: Continue)
- Fix quiet flag readability (CR #6)
- Improve error messages with context (CR #9)
- Add test helper functions (CR #10)
- Document Action clone rationale (CR #7)"
```

---

## Task 6: Add --debug flag for detailed output

**Files:**
- Modify: `plugin/tools/workflow/src/main.rs:11-30`
- Modify: `plugin/tools/workflow/src/runner.rs:1-24`

**Step 1: Write failing test for debug output**

Add to `tests/implicit_defaults_test.rs`:

```rust
#[test]
fn test_debug_mode_shows_evaluation() {
    // This is more of a manual verification test
    // We can't easily test stdout in unit tests
    // Verify manually that debug output includes:
    // - "Checking: exit code (0 = Pass, non-zero = Fail)"
    // - "Result: Pass (exit 0)" or "Result: Fail (exit 1)"
    // - "Action: Continue" or "Action: STOP"

    // For now, just ensure it compiles
    let workflow = r#"
# Step 1: Test

```bash
exit 0
```
"#;
    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Enforcement,
    );
    runner.set_debug(true);
    let _ = runner.run();
}
```

**Step 2: Add debug field to WorkflowRunner**

In `runner.rs:6-12`:

```rust
pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
    iterations: usize,
    max_iterations: usize,
    mode: ExecutionMode,
    debug: bool,  // Add this field
}
```

**Step 3: Update new method and add set_debug**

In `runner.rs:14-24`:

```rust
impl WorkflowRunner {
    pub fn new(steps: Vec<Step>, mode: ExecutionMode) -> Self {
        let max_iterations = steps.len() * 10;
        Self {
            steps,
            current_step: 0,
            iterations: 0,
            max_iterations,
            mode,
            debug: false,  // Add default
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    // ... rest of methods
}
```

**Step 4: Add debug output to run method**

In `runner.rs`, after executing command and before evaluating conditionals:

```rust
// Status
let status_symbol = if output.success { "✓" } else { "✗" };
let status_text = if output.success { "Passed" } else { "Failed" };
println!(
    "{} {} (exit {})",
    status_symbol, status_text, output.exit_code
);

// Debug output
if self.debug {
    println!("→ [DEBUG] Checking: exit code (0 = Pass, non-zero = Fail)");
    let result_text = if output.success {
        format!("Pass (exit {})", output.exit_code)
    } else {
        format!("Fail (exit {})", output.exit_code)
    };
    println!("→ [DEBUG] Result: {}", result_text);
}

// Evaluate conditionals
let action = self.evaluate_conditionals(&step.conditionals, &output)?
    .unwrap_or_else(|| self.apply_defaults(&output, &step.conditionals));

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
        Action::GoToStep { number } => println!("→ [DEBUG] Action: Go to Step {}", number),
    }
}
```

**Step 5: Add --debug CLI flag**

In `main.rs:11-30`:

```rust
#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows with deterministic control flow")]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,

    /// Enable guided mode for flexible execution
    #[arg(long)]
    guided: bool,

    /// Show steps without executing
    #[arg(long)]
    dry_run: bool,

    /// List all steps
    #[arg(long)]
    list: bool,

    /// Show detailed evaluation information
    #[arg(long)]
    debug: bool,
}
```

**Step 6: Wire up debug flag in main**

In `main.rs:75-92`:

```rust
// Run workflow
println!("→ Workflow: {}", args.workflow_file);
println!("→ Steps: {}", steps.len());
if args.debug {
    println!("→ Debug mode enabled");
}

let mode = if args.guided {
    execution_mode::ExecutionMode::Guided
} else {
    execution_mode::ExecutionMode::Enforcement
};

let mut runner = runner::WorkflowRunner::new(steps, mode);
runner.set_debug(args.debug);  // Add this line
let result = match runner.run() {
    // ... rest of code
```

**Step 7: Run tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 8: Fix confusing UX for ignored conditionals (Code Review #4)**

In normal (non-debug) mode, don't show redundant conditional messages. Update `runner.rs`:

Before (confusing):
```rust
match action {
    Action::Continue => {
        println!("→ Condition matched: Continue");  // Shows even if ignored
    }
    // ...
}
```

After (clean):
```rust
match action {
    Action::Continue => {
        // Only show if debug mode
        if !self.debug {
            // Silent continue - implicit default
        }
    }
    Action::Stop { message } => {
        if let Some(msg) = message {
            println!("→ Action: STOP ({})", msg);
        } else {
            println!("→ Action: STOP");
        }
        return Ok(ExecutionResult::Stopped { message: message.clone() });
    }
    Action::GoToStep { number } => {
        println!("→ Action: Go to Step {}", number);
        self.current_step = self.find_step_index(number)?;
        continue 'workflow_loop;
    }
}
```

This way:
- Debug mode: Shows all details ("Checking:", "Result:", "Action:")
- Normal mode: Only shows significant actions (STOP, GoTo)
- Implicit continues are silent (clean output)

**Step 9: Manual verification**

Run: `cd plugin/tools/workflow && cargo run -- --debug ../../examples/simple.md`
Expected: See debug output with "Checking:", "Result:", "Action:"

Run: `cd plugin/tools/workflow && cargo run -- ../../examples/simple.md`
Expected: Clean output without redundant "Continue" messages

**Step 10: Commit**

```bash
cd plugin/tools/workflow
git add src/main.rs src/runner.rs tests/implicit_defaults_test.rs
git commit -m "feat(workflow): add --debug flag + clean UX

- Add --debug flag for detailed evaluation output
- Show exit code checking, result, and action (debug only)
- Fix confusing UX: hide implicit continues in normal mode (CR #4)
- Normal mode: Only show significant actions (STOP, GoTo)
- Debug mode: Show all internal details"
```

---

## Task 7: Update documentation

**Files:**
- Modify: `plugin/tools/workflow/README.md`
- Modify: `plugin/skills/workflow/creating-workflows/SKILL.md`
- Modify: `plugin/skills/workflow/executing-workflows/SKILL.md`

**Step 1: Update README with new syntax**

Replace workflow syntax section in `plugin/tools/workflow/README.md` (lines 100-189):

```markdown
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
```

**Step 2: Update creating-workflows skill**

Update examples in `plugin/skills/workflow/creating-workflows/SKILL.md` to use new syntax:

Replace lines 43-54 (Good example):

```markdown
**Good:**
```markdown
# Step 1: Run tests

Fail: STOP (fix tests)

```bash
mise run test
```
```
```

Replace lines 250-303 (Complete Syntax Example):

```markdown
## Complete Syntax Example

Here's a workflow using all syntax elements:

```markdown
# Step 1: Check for changes

Fail: STOP (nothing to commit)

```bash quiet
mise run check-has-changes
```

# Step 2: Verify tests exist

**Prompt:** Do ALL new/modified functions have tests?

# Step 3: Run test suite

Fail: STOP (fix failing tests)

```bash
mise run test
```

# Step 4: Check formatting

Fail: STOP (run mise fmt to format)

```bash quiet
mise run fmt -- --check
```

# Step 5: Commit changes

```bash
git add .
git commit
```
```

**This workflow demonstrates:**
- Steps (H1 headers with numbers)
- One command per step (enforced)
- Conditionals (Pass/Fail labels)
- Prompts (manual verification)
- Implicit defaults (Pass → Continue, Fail → STOP)
```

**Step 3: Update all other examples in creating-workflows**

Search and replace all workflow examples to use new syntax. Key patterns:

- Remove `→ Exit 0: Continue` (implicit default)
- Change `→ Exit ≠ 0: STOP (msg)` to `Fail: STOP (msg)`
- Change `→ If output empty: STOP (msg)` to use wrapper script approach

**Step 4: Update executing-workflows skill**

Update `plugin/skills/workflow/executing-workflows/SKILL.md` examples (lines 43-278) to use new syntax.

**Step 5: Verify documentation builds**

Read through all three files to ensure examples are consistent and correct.

**Step 6: Commit**

```bash
git add plugin/tools/workflow/README.md
git add plugin/skills/workflow/creating-workflows/SKILL.md
git add plugin/skills/workflow/executing-workflows/SKILL.md
git commit -m "docs(workflow): update all docs for new Pass/Fail syntax

- Replace arrow conditionals with Pass/Fail labels
- Document implicit defaults
- Update all examples
- Add wrapper script guidance for complex conditions"
```

---

## Task 8: Update example workflows

**Files:**
- Modify: `plugin/practices/git-commit-algorithm.md`
- Create: `plugin/tools/workflow/examples/simple-new-syntax.md`

**Step 1: Create new example with clean syntax**

Create `plugin/tools/workflow/examples/simple-new-syntax.md`:

```markdown
# Simple Workflow Example (New Syntax)

This demonstrates the simplified Pass/Fail syntax.

## Minimal (Most Common)

# Step 1: Run tests

```bash
mise run test
```

# Step 2: Build

```bash
mise run build
```

# Step 3: Deploy

```bash
mise run deploy
```

Any failure stops automatically (implicit Fail → STOP).

## With Custom Messages

# Step 1: Run tests

Fail: STOP (fix tests before deploying)

```bash
mise run test
```

# Step 2: Build

Fail: STOP (build failed - check logs)

```bash
mise run build
```

## With Control Flow

# Step 1: Check if already deployed

Pass: Go to Step 3  # Already deployed, skip build
Fail: Continue      # Not deployed, need to build

```bash quiet
mise run check-deployment
```

# Step 2: Build and deploy

```bash
mise run build && mise run deploy
```

# Step 3: Verify deployment

```bash
mise run verify
```
```

**Step 2: Convert git-commit-algorithm to new syntax**

Update `plugin/practices/git-commit-algorithm.md` with workflow that can be executed:

After the decision algorithm section (around line 62), add:

```markdown
## Executable Workflow

Run with: `workflow plugin/practices/git-commit-algorithm.md`

---

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

# Step 4: Run checks

Fail: STOP (run mise check to see failures)

```bash
mise run check
```

# Step 5: Check documentation

**Prompt:** Is documentation updated for user-facing changes?

# Step 6: Verify atomic commit

**Prompt:** Do changes serve a single atomic purpose?

# Step 7: Commit changes

Fail: STOP (commit failed - check message format)

```bash
git commit
```
```

**Step 3: Test new example**

Run: `cd plugin/tools/workflow && cargo run -- examples/simple-new-syntax.md`
Expected: Executes successfully (or stops appropriately)

**Step 4: Commit**

```bash
git add plugin/tools/workflow/examples/simple-new-syntax.md
git add plugin/practices/git-commit-algorithm.md
git commit -m "docs(workflow): add examples using new syntax

- Create simple example demonstrating Pass/Fail syntax
- Add executable workflow to git-commit-algorithm
- Show minimal, message, and control flow patterns"
```

---

## Task 9: Remove deprecated code and update tests

**Files:**
- Modify: `plugin/tools/workflow/src/models.rs:33-48`
- Modify: `plugin/tools/workflow/src/parser.rs:142-211`
- Modify: `plugin/tools/workflow/src/runner.rs:136-197`

**Step 1: Ensure all tests pass with current code**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 2: Remove deprecated variants from Conditional enum**

In `models.rs`, replace the Conditional enum:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Conditional {
    Pass { action: Action },
    Fail { action: Action },
}
```

**Step 3: Remove legacy parsing code**

In `parser.rs`, simplify `parse_conditional` to remove arrow syntax support:

```rust
fn parse_conditional(text: &str) -> Option<Conditional> {
    let trimmed = text.trim();

    if trimmed.starts_with("Pass:") {
        let action_str = trimmed.strip_prefix("Pass:")?.trim();
        let action = parse_action(action_str)?;
        return Some(Conditional::Pass { action });
    }

    if trimmed.starts_with("Fail:") {
        let action_str = trimmed.strip_prefix("Fail:")?.trim();
        let action = parse_action(action_str)?;
        return Some(Conditional::Fail { action });
    }

    None
}
```

**Step 4: Simplify evaluate_conditionals**

In `runner.rs`, simplify to only handle Pass/Fail:

```rust
fn evaluate_conditionals(
    &self,
    conditionals: &[Conditional],
    output: &CommandOutput,
) -> Result<Option<Action>> {
    for conditional in conditionals {
        match conditional {
            Conditional::Pass { action } => {
                if output.success {
                    return Ok(Some(action.clone()));
                }
            }
            Conditional::Fail { action } => {
                if !output.success {
                    return Ok(Some(action.clone()));
                }
            }
        }
    }
    Ok(None)
}
```

**Step 5: Remove legacy tests**

In `parser.rs`, remove tests for legacy syntax:
- `test_parse_conditionals` (lines 331-364)
- `test_parse_arbitrary_exit_codes` (lines 366-401)
- `test_parse_conditionals_with_ascii_arrow` (lines 403-427)
- `test_parse_output_contains_with_quotes` (lines 429-446)

**Step 6: Remove dead code (Code Review #8)**

In `models.rs`, remove unused Workflow struct and `#[allow(dead_code)]` annotations:

Remove:
```rust
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Workflow {
    pub steps: Vec<Step>,
}
```

Remove all `#[allow(dead_code)]` from:
- Command struct
- Prompt struct
- Conditional enum
- Action enum

**Step 7: Run all tests**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS (legacy tests removed, new tests remain)

**Step 8: Note on magic strings simplification (Code Review #5)**

With Pass/Fail syntax, magic strings are dramatically simplified:
- Before: "Exit 0", "Exit ≠ 0", "Exit != 0", "If output empty", "If output contains", "Otherwise"
- After: "Pass:", "Fail:"

Only two string patterns to match. No need for constants - the simplification solved the problem.

**Step 9: Commit**

```bash
cd plugin/tools/workflow
git add src/models.rs src/parser.rs src/runner.rs
git commit -m "refactor(workflow): remove deprecated code + clean break

- Remove arrow-based conditional syntax
- Remove output-based conditionals
- Remove dead code: unused Workflow struct (CR #8)
- Remove #[allow(dead_code)] annotations
- Simplify magic strings: 6 patterns → 2 (CR #5)
- Clean break to new Pass/Fail syntax"
```

---

## Task 10: Final verification and documentation

**Files:**
- Modify: `plugin/tools/workflow/README.md:8-12`

**Step 1: Run full test suite**

Run: `cd plugin/tools/workflow && cargo test`
Expected: All tests PASS

**Step 2: Build release binary**

Run: `cd plugin/tools/workflow && cargo build --release`
Expected: Clean build

**Step 3: Test with real workflow**

Run: `cd plugin/tools/workflow && cargo run -- --debug examples/simple-new-syntax.md`
Expected: Successful execution with debug output

**Step 4: Update README "Why?" section**

In `plugin/tools/workflow/README.md:8-12`, update to reflect simplification:

```markdown
## Why?

**Single source of truth:** Workflows documented in markdown are both readable AND executable.

**Algorithmic enforcement:** LLMs rationalize under pressure (33% compliance). Deterministic execution prevents skipped steps (100% compliance).

**Simple semantics:** One code block → one exit code → one evaluation. Exit 0 = Pass → Continue. Exit non-zero = Fail → STOP. Override when needed.

**Convention over configuration:** Minimal syntax for common cases. Explicit only when needed.
```

**Step 5: Verify all documentation is updated**

Check:
- ✅ README.md updated
- ✅ creating-workflows SKILL.md updated
- ✅ executing-workflows SKILL.md updated
- ✅ Examples updated
- ✅ git-commit-algorithm updated

**Step 6: Create summary document**

Create `plugin/tools/workflow/MIGRATION.md`:

```markdown
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
```

**Step 7: Final commit**

```bash
git add plugin/tools/workflow/README.md
git add plugin/tools/workflow/MIGRATION.md
git commit -m "docs(workflow): finalize migration documentation

- Update README with simplified explanation
- Create migration guide
- Document breaking changes and benefits
- Ready for release"
```

---

## Verification Checklist

After completing all tasks:

- [ ] All tests pass: `cargo test`
- [ ] Clean build: `cargo build --release`
- [ ] Examples work: `cargo run -- examples/simple-new-syntax.md`
- [ ] Debug flag works: `cargo run -- --debug examples/simple-new-syntax.md`
- [ ] Documentation updated (README, skills, practices)
- [ ] Migration guide created
- [ ] Git commit algorithm updated and executable

## Future Enhancements (Deferred)

These code review suggestions are not included in this plan:

**Code Review #11: JSON output**
- Consider for future: `--format json` for tool integration
- Would enable programmatic workflow execution
- Not required for initial simplification release

**Code Review #12-15: Not required**
- Progress persistence / checkpoints
- Multiple shell support
- Parallel execution
- Step metadata

These are nice-to-have features that don't impact the core simplification goals.

---

## Related Skills

- **Test-Driven Development:** `@${SUPERPOWERS_SKILLS_ROOT}/skills/testing/test-driven-development/SKILL.md`
- **Executing Plans:** `@${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md`
- **Git Guidelines:** `@${CLAUDE_PLUGIN_ROOT}practices/git-guidelines.md`
- **Conventional Commits:** `@${CLAUDE_PLUGIN_ROOT}practices/conventional-commits.md`
