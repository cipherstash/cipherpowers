use workflow::*;

mod helpers;
use helpers::*;

#[test]
fn test_implicit_pass_continue() {
    // No conditionals specified, command succeeds → should continue
    let workflow = r#"
## 1. First step

```bash
exit 0
```

## 2. Second step

```bash
echo "reached step 2"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(steps, execution_mode::ExecutionMode::Enforcement);
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Success);
}

#[test]
fn test_implicit_fail_stop() {
    // No conditionals specified, command fails → should STOP
    let workflow = r#"
## 1. First step

```bash
exit 1
```

## 2. Should not reach

```bash
echo "should not see this"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(steps, execution_mode::ExecutionMode::Enforcement);
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Stopped { message: None });
}

#[test]
fn test_pass_conditional_on_success() {
    let workflow = r#"
## 1. Run command

PASS: Go to Step 3

```bash
exit 0
```

## 2. Should skip

```bash
echo "should not see this"
```

## 3. Should reach

```bash
echo "reached step 3"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(steps, execution_mode::ExecutionMode::Guided);
    let result = runner.run().unwrap();

    assert_eq!(result, runner::ExecutionResult::Success);
}

#[test]
fn test_fail_conditional_on_failure() {
    let workflow = r#"
## 1. Run command

FAIL: STOP command failed as expected

```bash
exit 1
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(steps, execution_mode::ExecutionMode::Enforcement);
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
## 1. Optional hook

FAIL: Continue

```bash
exit 1
```

## 2. Should still reach

```bash
echo "reached despite failure"
```
"#;

    let steps = parser::parse_workflow(workflow).unwrap();
    let mut runner = runner::WorkflowRunner::new(
        steps,
        execution_mode::ExecutionMode::Guided, // Need Guided mode to allow Continue
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

    let mut runner = runner::WorkflowRunner::new(steps, execution_mode::ExecutionMode::Enforcement);
    let result = runner.run().unwrap();
    assert_eq!(result, runner::ExecutionResult::Success);
}
