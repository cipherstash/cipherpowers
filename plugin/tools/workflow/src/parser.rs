use crate::models::*;
use anyhow::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

// TODO: Remove #[allow(dead_code)] once Task 4 (execution engine) uses this function
#[allow(dead_code)]
pub fn parse_workflow(markdown: &str) -> Result<Vec<Step>> {
    let parser = Parser::new(markdown);
    let mut steps = Vec::new();
    let mut current_step: Option<Step> = None;
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_lang = String::new();
    let mut in_strong = false;
    let mut strong_content = String::new();
    let mut capturing_prompt = false;
    let mut current_prompt = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
                if let Some(step) = current_step.take() {
                    steps.push(step);
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                // Finalize prompt if we're starting a code block
                if capturing_prompt && !current_prompt.is_empty() {
                    if let Some(step) = current_step.as_mut() {
                        step.prompts.push(Prompt {
                            text: current_prompt.trim().to_string(),
                        });
                    }
                    current_prompt.clear();
                    capturing_prompt = false;
                }
                in_code_block = true;
                code_block_content.clear();
                if let pulldown_cmark::CodeBlockKind::Fenced(lang) = kind {
                    code_block_lang = lang.to_string();
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                // Parse language tag properly using whitespace splitting
                let parts: Vec<&str> = code_block_lang.split_whitespace().collect();
                let is_bash = parts.first().is_some_and(|&lang| lang == "bash");
                let quiet = parts.contains(&"quiet");

                if is_bash {
                    if let Some(step) = current_step.as_mut() {
                        // Error if command already exists
                        if step.command.is_some() {
                            anyhow::bail!(
                                "Multiple code blocks per step not allowed. Step {} already has a command block. \
                                 Suggestion: (1) Combine commands using && or ; operators, or (2) Split into separate steps.",
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
            Event::Start(Tag::Strong) => {
                // Finalize prompt if we're starting a new strong tag
                if capturing_prompt && !current_prompt.is_empty() {
                    if let Some(step) = current_step.as_mut() {
                        step.prompts.push(Prompt {
                            text: current_prompt.trim().to_string(),
                        });
                    }
                    current_prompt.clear();
                    capturing_prompt = false;
                }
                in_strong = true;
                strong_content.clear();
            }
            Event::End(Tag::Strong) => {
                in_strong = false;
                // Check if this was "Prompt:"
                if strong_content.trim() == "Prompt:" {
                    capturing_prompt = true;
                }
                strong_content.clear();
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else if in_strong {
                    strong_content.push_str(&text);
                } else if capturing_prompt {
                    // Accumulate all text until we hit a non-text event
                    current_prompt.push_str(&text);
                } else if let Some(captures) = extract_step_header(&text) {
                    current_step = Some(Step {
                        number: captures.0,
                        description: captures.1,
                        command: None,
                        prompts: Vec::new(),
                        conditionals: Vec::new(),
                    });
                } else if let Some(conditional) = parse_conditional(&text) {
                    if let Some(step) = current_step.as_mut() {
                        step.conditionals.push(conditional);
                    }
                }
            }
            Event::Code(code) => {
                if capturing_prompt {
                    // Include inline code with backticks
                    current_prompt.push('`');
                    current_prompt.push_str(&code);
                    current_prompt.push('`');
                }
            }
            Event::End(Tag::Paragraph) => {
                // Finalize prompt at end of paragraph
                if capturing_prompt && !current_prompt.is_empty() {
                    if let Some(step) = current_step.as_mut() {
                        step.prompts.push(Prompt {
                            text: current_prompt.trim().to_string(),
                        });
                    }
                    current_prompt.clear();
                    capturing_prompt = false;
                }
            }
            _ => {}
        }
    }

    if let Some(step) = current_step {
        steps.push(step);
    }

    // Validate that workflow is not empty
    if steps.is_empty() {
        anyhow::bail!(
            "No steps found in workflow. Expected H1 headers like '# Step 1: Description'"
        );
    }

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
}

fn extract_step_header(text: &str) -> Option<(usize, String)> {
    // Parse "Step 1: Description" format
    let parts: Vec<&str> = text.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }

    let step_part = parts[0].trim();
    if !step_part.starts_with("Step ") {
        return None;
    }

    let number: usize = step_part.strip_prefix("Step ")?.trim().parse().ok()?;
    let description = parts[1].trim().to_string();

    Some((number, description))
}

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

fn parse_action(text: &str) -> Option<Action> {
    let trimmed = text.trim();

    if trimmed == "Continue" {
        Some(Action::Continue)
    } else if trimmed == "STOP" {
        Some(Action::Stop { message: None })
    } else if trimmed.starts_with("STOP (") && trimmed.ends_with(")") {
        let message = trimmed
            .strip_prefix("STOP (")?
            .strip_suffix(")")?
            .to_string();
        Some(Action::Stop {
            message: Some(message),
        })
    } else if trimmed.starts_with("Go to Step ") {
        let number: usize = trimmed.strip_prefix("Go to Step ")?.trim().parse().ok()?;
        Some(Action::GoToStep { number })
    } else {
        None
    }
}

fn validate_workflow(steps: &[Step]) -> Result<()> {
    for step in steps {
        // Warn if step has no executable content
        // NOTE: Validation warnings are written to stderr (via eprintln!) to ensure
        // they're visible even when stdout is redirected. This follows standard Unix
        // conventions where warnings/diagnostics go to stderr.
        if step.command.is_none() && step.prompts.is_empty() {
            eprintln!(
                "Warning: Step {} '{}' has no commands or prompts",
                step.number, step.description
            );
        }

        // Detect simple infinite loops (GoTo self with no STOP)
        // NOTE: Loop warnings also go to stderr for same reason as above.
        for conditional in &step.conditionals {
            let goto_number = match conditional {
                Conditional::Pass {
                    action: Action::GoToStep { number },
                }
                | Conditional::Fail {
                    action: Action::GoToStep { number },
                } => Some(number),
                #[allow(deprecated)]
                Conditional::ExitCode {
                    action: Action::GoToStep { number },
                    ..
                }
                | Conditional::ExitNotZero {
                    action: Action::GoToStep { number },
                }
                | Conditional::OutputEmpty {
                    action: Action::GoToStep { number },
                }
                | Conditional::OutputContains {
                    action: Action::GoToStep { number },
                    ..
                }
                | Conditional::Otherwise {
                    action: Action::GoToStep { number },
                } => Some(number),
                _ => None,
            };
            if let Some(number) = goto_number {
                if *number == step.number {
                    eprintln!(
                        "Warning: Step {} has GoTo self - possible infinite loop",
                        step.number
                    );
                }
            }
        }

        // Validate GoTo targets exist
        for conditional in &step.conditionals {
            let goto_number = match conditional {
                Conditional::Pass {
                    action: Action::GoToStep { number },
                }
                | Conditional::Fail {
                    action: Action::GoToStep { number },
                } => Some(number),
                #[allow(deprecated)]
                Conditional::ExitCode {
                    action: Action::GoToStep { number },
                    ..
                }
                | Conditional::ExitNotZero {
                    action: Action::GoToStep { number },
                }
                | Conditional::OutputEmpty {
                    action: Action::GoToStep { number },
                }
                | Conditional::OutputContains {
                    action: Action::GoToStep { number },
                    ..
                }
                | Conditional::Otherwise {
                    action: Action::GoToStep { number },
                } => Some(number),
                _ => None,
            };
            if let Some(number) = goto_number {
                if *number < 1 || *number > steps.len() {
                    anyhow::bail!(
                        "Step {}: GoTo target Step {} does not exist (workflow has {} steps)",
                        step.number,
                        number,
                        steps.len()
                    );
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parsing {
        use super::*;

        #[test]
        fn test_parse_steps_from_markdown() {
            let markdown = r#"
# Step 1: First step

Some description

# Step 2: Second step

More description
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 2);
            assert_eq!(steps[0].number, 1);
            assert_eq!(steps[0].description, "First step");
            assert_eq!(steps[1].number, 2);
            assert_eq!(steps[1].description, "Second step");
        }

        #[test]
        fn test_parse_commands_in_steps() {
            let markdown = r#"
# Step 1: Run tests

```bash
mise run test
```

# Step 2: Check status

```bash quiet
git status
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert!(steps[0].command.is_some());
            assert_eq!(steps[0].command.as_ref().unwrap().code, "mise run test");
            assert!(!steps[0].command.as_ref().unwrap().quiet);

            assert!(steps[1].command.is_some());
            assert_eq!(steps[1].command.as_ref().unwrap().code, "git status");
            assert!(steps[1].command.as_ref().unwrap().quiet);
        }

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

        #[test]
        fn test_empty_markdown_returns_error() {
            let markdown = "";
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.to_string().contains("No steps found"));
        }

        #[test]
        fn test_non_sequential_steps_returns_error() {
            let markdown = r#"
# Step 1: First step

# Step 5: Fifth step
"#;
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.to_string().contains("sequential"));
            assert!(err.to_string().contains("Expected Step 2"));
        }

        #[test]
        fn test_code_block_bashquiet_not_quiet() {
            // "bashquiet" (no space) should NOT be treated as quiet
            let markdown = r#"
# Step 1: Test

```bashquiet
echo test
```
"#;
            let steps = parse_workflow(markdown).unwrap();
            // Should not parse as bash at all since language is "bashquiet" not "bash"
            assert!(steps[0].command.is_none());
        }

        #[test]
        fn test_code_block_bash_quiet_is_quiet() {
            // Verify the fix: "bash quiet" (with space) should be quiet
            let markdown = r#"
# Step 1: Test

```bash quiet
echo test
```
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert!(steps[0].command.is_some());
            assert!(steps[0].command.as_ref().unwrap().quiet);
        }

        #[test]
        fn test_parse_prompts() {
            let markdown = r#"
# Step 1: Verify tests

**Prompt:** Do all functions have tests?

Some other text
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].prompts.len(), 1);
            assert_eq!(steps[0].prompts[0].text, "Do all functions have tests?");
        }

        #[test]
        fn test_parse_prompt_with_inline_code() {
            let markdown = r#"
# Step 1: Verify

**Prompt:** Do you want to update `main.rs` file?
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].prompts.len(), 1);
            assert_eq!(
                steps[0].prompts[0].text,
                "Do you want to update `main.rs` file?"
            );
        }
    }

    mod validation {
        use super::*;

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
        #[allow(deprecated)]
        fn test_validation_invalid_goto() {
            let markdown = r#"
# Step 1: Bad goto

→ Exit 0: Go to Step 99

```bash
echo "test"
```
"#;
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.to_string().contains("does not exist"));
        }
    }

    mod conditionals {
        use super::*;

        #[test]
        #[allow(deprecated)]
        fn test_parse_conditionals() {
            let markdown = r#"
# Step 1: Run tests

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].conditionals.len(), 2);

            match &steps[0].conditionals[0] {
                Conditional::ExitCode { code, action } => {
                    assert_eq!(*code, 0);
                    assert_eq!(*action, Action::Continue);
                }
                _ => panic!("Expected ExitCode conditional"),
            }

            match &steps[0].conditionals[1] {
                Conditional::ExitNotZero { action } => match action {
                    Action::Stop { message } => {
                        assert_eq!(message.as_deref(), Some("fix tests"));
                    }
                    _ => panic!("Expected Stop action"),
                },
                _ => panic!("Expected ExitNotZero conditional"),
            }
        }

        #[test]
        #[allow(deprecated)]
        fn test_parse_arbitrary_exit_codes() {
            let markdown = r#"
# Step 1: Test
→ Exit 42: STOP (custom exit code)
→ Exit 127: STOP (command not found)
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].conditionals.len(), 2);

            match &steps[0].conditionals[0] {
                Conditional::ExitCode { code, action } => {
                    assert_eq!(*code, 42);
                    match action {
                        Action::Stop { message } => {
                            assert_eq!(message.as_deref(), Some("custom exit code"));
                        }
                        _ => panic!("Expected Stop action"),
                    }
                }
                _ => panic!("Expected ExitCode conditional"),
            }

            match &steps[0].conditionals[1] {
                Conditional::ExitCode { code, action } => {
                    assert_eq!(*code, 127);
                    match action {
                        Action::Stop { message } => {
                            assert_eq!(message.as_deref(), Some("command not found"));
                        }
                        _ => panic!("Expected Stop action"),
                    }
                }
                _ => panic!("Expected ExitCode conditional"),
            }
        }

        #[test]
        #[allow(deprecated)]
        fn test_parse_conditionals_with_ascii_arrow() {
            let markdown = r#"
# Step 1: Test
-> Exit 0: Continue
-> Exit != 0: STOP
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].conditionals.len(), 2);

            match &steps[0].conditionals[0] {
                Conditional::ExitCode { code, action } => {
                    assert_eq!(*code, 0);
                    assert_eq!(*action, Action::Continue);
                }
                _ => panic!("Expected ExitCode conditional"),
            }

            match &steps[0].conditionals[1] {
                Conditional::ExitNotZero { action } => {
                    assert_eq!(*action, Action::Stop { message: None });
                }
                _ => panic!("Expected ExitNotZero conditional"),
            }
        }

        #[test]
        #[allow(deprecated)]
        fn test_parse_output_contains_with_quotes() {
            let markdown = r#"
# Step 1: Test
→ If output contains "error": STOP
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].conditionals.len(), 1);

            match &steps[0].conditionals[0] {
                Conditional::OutputContains { text, action } => {
                    assert_eq!(text, "error");
                    assert_eq!(*action, Action::Stop { message: None });
                }
                _ => panic!("Expected OutputContains conditional"),
            }
        }

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
    }
}
