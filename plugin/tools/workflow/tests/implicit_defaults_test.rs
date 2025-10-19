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
