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

/// Test that verifies debug mode output by running the workflow binary
/// and capturing its stdout. This approach avoids the limitations of
/// capturing stdout in unit tests (parallel test execution, thread safety).
#[test]
fn test_debug_mode_shows_evaluation() {
    use std::process::Command;
    use std::fs;

    // Create a temporary workflow file
    let workflow_content = r#"
# Step 1: Test

```bash
exit 0
```
"#;
    let temp_path = "/tmp/test_workflow_debug_pass.md";
    fs::write(temp_path, workflow_content).unwrap();

    // Run the workflow binary with --debug flag
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--debug", temp_path])
        .current_dir("/Users/tobyhede/src/cipherpowers/.worktrees/workflow-executor/plugin/tools/workflow")
        .output()
        .expect("Failed to execute workflow");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Clean up
    fs::remove_file(temp_path).ok();

    // Verify debug output contains expected messages
    assert!(stdout.contains("Checking: exit code (0 = Pass, non-zero = Fail)"),
            "Debug output should show Pass/Fail criteria. Got:\n{}", stdout);
    assert!(stdout.contains("Result: Pass (exit 0)"),
            "Debug output should show evaluation result. Got:\n{}", stdout);
    assert!(stdout.contains("Action: Continue"),
            "Debug output should show action taken. Got:\n{}", stdout);
}

#[test]
fn test_debug_mode_shows_fail_evaluation() {
    use std::process::Command;
    use std::fs;

    // Create a temporary workflow file
    let workflow_content = r#"
# Step 1: Failing test

```bash
exit 1
```
"#;
    let temp_path = "/tmp/test_workflow_debug_fail.md";
    fs::write(temp_path, workflow_content).unwrap();

    // Run the workflow binary with --debug flag
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--debug", temp_path])
        .current_dir("/Users/tobyhede/src/cipherpowers/.worktrees/workflow-executor/plugin/tools/workflow")
        .output()
        .expect("Failed to execute workflow");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Clean up
    fs::remove_file(temp_path).ok();

    // Verify debug output contains expected messages for failure
    assert!(stdout.contains("Checking: exit code (0 = Pass, non-zero = Fail)"),
            "Debug output should show Pass/Fail criteria. Got:\n{}", stdout);
    assert!(stdout.contains("Result: Fail (exit 1)"),
            "Debug output should show failure result. Got:\n{}", stdout);
    assert!(stdout.contains("Action: STOP"),
            "Debug output should show STOP action. Got:\n{}", stdout);
}
