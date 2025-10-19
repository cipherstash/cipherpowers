use workflow::*;

mod helpers;
use helpers::*;

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
        execution_mode::ExecutionMode::Guided,  // Need Guided mode to allow Continue
    );
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Success);
}

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
