use crate::models::*;
use anyhow::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

/// Convert Vec<Conditional> to Option<Conditions>
/// Returns None if empty, Some(Conditions) if we have both PASS and FAIL
fn convert_conditionals(conditionals: Vec<Conditional>) -> Option<Conditions> {
    if conditionals.is_empty() {
        return None;
    }

    let mut pass_action = None;
    let mut fail_action = None;

    for conditional in conditionals {
        match conditional {
            Conditional::Pass { action } => pass_action = Some(action),
            Conditional::Fail { action } => fail_action = Some(action),
        }
    }

    // If we have both, create Conditions
    match (pass_action, fail_action) {
        (Some(pass), Some(fail)) => Some(Conditions { pass, fail }),
        (Some(pass), None) => Some(Conditions {
            pass,
            fail: Action::Stop(None), // Default FAIL action
        }),
        (None, Some(fail)) => Some(Conditions {
            pass: Action::Continue, // Default PASS action
            fail,
        }),
        (None, None) => None,
    }
}

pub fn parse_workflow(markdown: &str) -> Result<Vec<Step>> {
    let parser = Parser::new(markdown);
    let mut steps = Vec::new();
    let mut current_step: Option<Step> = None;
    let mut pending_conditionals: Vec<Conditional> = Vec::new();
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_lang = String::new();
    let mut in_strong = false;
    let mut strong_content = String::new();
    let mut capturing_prompt = false;
    let mut current_prompt = String::new();
    let mut in_heading = None::<HeadingLevel>;
    let mut implicit_text = String::new(); // Task 2.4: Collect implicit prompt text

    for event in parser {
        match event {
            Event::Start(Tag::Heading(level @ (HeadingLevel::H1 | HeadingLevel::H2), _, _)) => {
                // Finalize any implicit text as a prompt before starting new step
                // Only create implicit prompt if: no code block AND no explicit prompts
                if let Some(step) = current_step.as_mut() {
                    if !implicit_text.trim().is_empty() && step.command.is_none() && step.prompts.is_empty() {
                        step.prompts.push(Prompt {
                            text: implicit_text.trim().to_string(),
                        });
                    }
                    implicit_text.clear();
                }

                if let Some(mut step) = current_step.take() {
                    // Convert pending_conditionals to Conditions
                    step.conditions = convert_conditionals(std::mem::take(&mut pending_conditionals));
                    steps.push(step);
                }
                in_heading = Some(level);
            }
            Event::End(Tag::Heading(_, _, _)) => {
                in_heading = None;
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
                                step.number.get()
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
                } else if in_heading == Some(HeadingLevel::H2) {
                    // Only accept H2 (##) as step headers, not H1 (#)
                    if let Some(captures) = extract_step_header(&text) {
                        current_step = Some(Step {
                            number: captures.0,
                            description: captures.1,
                            command: None,
                            prompts: Vec::new(),
                            conditions: None,  // Will be populated from conditionals later
                        });
                        // Temporary: track conditionals separately
                        pending_conditionals.clear();
                    }
                } else if let Some(conditional) = parse_conditional(&text) {
                    pending_conditionals.push(conditional);
                } else if in_heading.is_none() && current_step.is_some() {
                    // Task 2.4: Collect implicit prompt text (not in heading, not conditional)
                    implicit_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                if capturing_prompt {
                    // Include inline code with backticks
                    current_prompt.push('`');
                    current_prompt.push_str(&code);
                    current_prompt.push('`');
                } else if !in_code_block && in_heading.is_none() && current_step.is_some() {
                    // Task 2.4: Include inline code in implicit text
                    implicit_text.push('`');
                    implicit_text.push_str(&code);
                    implicit_text.push('`');
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                // Task 2.4: Preserve line breaks in implicit text
                if !capturing_prompt && !in_code_block && in_heading.is_none() && current_step.is_some() {
                    implicit_text.push('\n');
                } else if capturing_prompt {
                    current_prompt.push('\n');
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

    // Task 2.4: Finalize implicit text for final step
    // Only create implicit prompt if: no code block AND no explicit prompts
    if let Some(mut step) = current_step {
        if !implicit_text.trim().is_empty() && step.command.is_none() && step.prompts.is_empty() {
            step.prompts.push(Prompt {
                text: implicit_text.trim().to_string(),
            });
        }
        // Convert pending_conditionals to Conditions
        step.conditions = convert_conditionals(std::mem::take(&mut pending_conditionals));
        steps.push(step);
    }

    // Validate that workflow is not empty
    if steps.is_empty() {
        anyhow::bail!(
            "Workflow must contain at least one step (heading starting with '##')"
        );
    }

    // Validate that step numbers are sequential
    for (i, step) in steps.iter().enumerate() {
        let expected = i + 1;
        if step.number.get() != expected {
            anyhow::bail!(
                "Steps must be numbered sequentially. Expected step {}, found step {}.\n\
                 Workflows must have exactly one algorithm with continuous numbering (1, 2, 3...).",
                expected,
                step.number.get()
            );
        }
    }

    // Validate workflow logic
    validate_workflow(&steps)?;

    Ok(steps)
}

/// Strip common separators and whitespace (reusable helper)
fn strip_separator(text: &str) -> &str {
    text.trim_start_matches(['.', ':', '—', '-', ')', ' '])
        .trim()
}

fn extract_step_header(text: &str) -> Option<(StepNumber, String)> {
    let trimmed = text.trim();

    // New syntax: "1. Description" or "1: Description" etc.
    // Parse number from start of text
    let num_end = trimmed.find(|c: char| !c.is_numeric())?;
    if num_end == 0 {
        return None; // No number at start
    }

    // Parse the number
    let number: usize = trimmed[..num_end].parse().ok()?;

    // Convert to StepNumber (rejects zero)
    let step_number = StepNumber::new(number)?;

    // Strip separator and extract title
    let title = strip_separator(&trimmed[num_end..]);

    // Reject "Step" keyword explicitly
    if title.starts_with("Step ") || title == "Step" {
        return None;
    }

    if title.is_empty() {
        return None;
    }

    Some((step_number, title.to_string()))
}

fn parse_conditional(text: &str) -> Option<Conditional> {
    let trimmed = text.trim();

    // Try ALLCAPS first (new syntax)
    if trimmed.starts_with("PASS") {
        let rest = trimmed.strip_prefix("PASS")?;
        let action_str = strip_separator(rest);
        let action = parse_action(action_str)?;
        return Some(Conditional::Pass { action });
    }

    if trimmed.starts_with("FAIL") {
        let rest = trimmed.strip_prefix("FAIL")?;
        let action_str = strip_separator(rest);
        let action = parse_action(action_str)?;
        return Some(Conditional::Fail { action });
    }

    // Fall back to old syntax (Pass: / Fail:) for backward compatibility
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

fn parse_action(text: &str) -> Option<Action> {
    let trimmed = text.trim();

    // New ALLCAPS syntax
    if trimmed == "CONTINUE" {
        return Some(Action::Continue);
    }

    if trimmed == "STOP" {
        return Some(Action::Stop(None));
    }

    if trimmed.starts_with("STOP ") {
        let message = trimmed.strip_prefix("STOP ")?.trim().to_string();
        return Some(Action::Stop(Some(message)));
    }

    if trimmed.starts_with("GOTO ") {
        let number: usize = trimmed.strip_prefix("GOTO ")?.trim().parse().ok()?;
        let step_number = StepNumber::new(number)?;
        return Some(Action::Goto(step_number));
    }

    // Old syntax fallback for backward compatibility
    if trimmed == "Continue" {
        return Some(Action::Continue);
    }

    if trimmed.starts_with("STOP (") && trimmed.ends_with(")") {
        let message = trimmed
            .strip_prefix("STOP (")?
            .strip_suffix(")")?
            .to_string();
        return Some(Action::Stop(Some(message)));
    }

    if trimmed.starts_with("Go to Step ") {
        let number: usize = trimmed.strip_prefix("Go to Step ")?.trim().parse().ok()?;
        let step_number = StepNumber::new(number)?;
        return Some(Action::Goto(step_number));
    }

    None
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
                step.number.get(), step.description
            );
        }

        // Detect simple infinite loops (GoTo self with no STOP) and validate GOTO targets
        // NOTE: Loop warnings go to stderr to ensure they're visible even when stdout is redirected.
        if let Some(conditions) = &step.conditions {
            // Check PASS action
            if let Action::Goto(step_num) = &conditions.pass {
                if step_num.get() == step.number.get() {
                    eprintln!(
                        "Warning: Step {} has GoTo self - possible infinite loop",
                        step.number.get()
                    );
                }
                if step_num.get() < 1 || step_num.get() > steps.len() {
                    anyhow::bail!(
                        "Step {}: GOTO target Step {} does not exist (workflow has {} steps)",
                        step.number.get(),
                        step_num.get(),
                        steps.len()
                    );
                }
            }

            // Check FAIL action
            if let Action::Goto(step_num) = &conditions.fail {
                if step_num.get() == step.number.get() {
                    eprintln!(
                        "Warning: Step {} has GoTo self - possible infinite loop",
                        step.number.get()
                    );
                }
                if step_num.get() < 1 || step_num.get() > steps.len() {
                    anyhow::bail!(
                        "Step {}: GOTO target Step {} does not exist (workflow has {} steps)",
                        step.number.get(),
                        step_num.get(),
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
## 1. First step

Some description

## 2. Second step

More description
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 2);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[0].description, "First step");
            assert_eq!(steps[1].number.get(), 2);
            assert_eq!(steps[1].description, "Second step");
        }

        #[test]
        fn test_parse_commands_in_steps() {
            let markdown = r#"
## 1. Run tests

```bash
mise run test
```

## 2. Check status

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
## 1. Test with multiple blocks

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
            assert!(err.to_string().contains("Workflow must contain at least one step"));
        }

        #[test]
        fn test_non_sequential_steps_returns_error() {
            let markdown = r#"
## 1. First step

## 5. Fifth step
"#;
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.to_string().contains("sequential"));
            assert!(err.to_string().contains("Expected step 2"));
        }

        #[test]
        fn test_code_block_bashquiet_not_quiet() {
            // "bashquiet" (no space) should NOT be treated as quiet
            let markdown = r#"
## 1. Test

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
## 1. Test

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
## 1. Verify tests

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
## 1. Verify

**Prompt:** Do you want to update `main.rs` file?
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].prompts.len(), 1);
            assert_eq!(
                steps[0].prompts[0].text,
                "Do you want to update `main.rs` file?"
            );
        }

        // Task 2.1: New header parsing syntax tests
        #[test]
        fn test_parse_heading_with_dot_separator() {
            let markdown = r#"
## 1. First step

```bash
echo "test"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 1);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[0].description, "First step");
        }

        #[test]
        fn test_parse_heading_with_colon_separator() {
            let markdown = r#"
## 1: First step

```bash
echo "test"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 1);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[0].description, "First step");
        }

        #[test]
        fn test_parse_heading_with_dash_separator() {
            let markdown = r#"
## 1 - First step

```bash
echo "test"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 1);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[0].description, "First step");
        }

        #[test]
        fn test_parse_heading_with_paren_separator() {
            let markdown = r#"
## 1) First step

```bash
echo "test"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 1);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[0].description, "First step");
        }

        #[test]
        fn test_parse_heading_with_just_space() {
            let markdown = r#"
## 1 First step

```bash
echo "test"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 1);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[0].description, "First step");
        }

        #[test]
        fn test_reject_h1_as_step() {
            // H1 should not be parsed as a step (only H2)
            let markdown = r#"
# 1. First step

```bash
echo "test"
```
"#;

            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.to_string().contains("Workflow must contain at least one step"));
        }

        #[test]
        fn test_reject_step_keyword() {
            // "Step" keyword should be rejected with clear error
            let markdown = r#"
## Step 1: First step

```bash
echo "test"
```
"#;

            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(
                err.to_string().contains("Step")
                || err.to_string().contains("format")
                || err.to_string().contains("Workflow must contain at least one step")
            );
        }
    }

    mod validation {
        use super::*;

        #[test]
        fn test_validation_empty_step_warning() {
            let markdown = r#"
## 1. Empty step with no content

## 2. Valid step

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
## 1. Bad goto

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

        // Task 3.1: Sequential numbering validation tests
        #[test]
        fn test_sequential_numbering_valid() {
            let markdown = r#"
## 1. First step

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```

## 3. Third step

```bash
echo "test"
```
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 3);
            assert_eq!(steps[0].number.get(), 1);
            assert_eq!(steps[1].number.get(), 2);
            assert_eq!(steps[2].number.get(), 3);
        }

        #[test]
        fn test_sequential_numbering_gap() {
            let markdown = r#"
## 1. First step

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```

## 4. Fourth step (missing step 3)

```bash
echo "test"
```
"#;
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            let err_str = err.to_string();
            assert!(err_str.contains("sequential"), "Error should mention 'sequential': {}", err_str);
            assert!(err_str.contains("Expected step 3") || err_str.contains("Expected Step 3"), "Error should mention expected step 3: {}", err_str);
            assert!(err_str.contains("found step 4") || err_str.contains("found Step 4"), "Error should mention found step 4: {}", err_str);
        }

        #[test]
        fn test_sequential_numbering_restart() {
            let markdown = r#"
## 1. First step

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```

## 1. First again (restart)

```bash
echo "test"
```
"#;
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            let err_str = err.to_string();
            assert!(err_str.contains("sequential"), "Error should mention 'sequential': {}", err_str);
            assert!(err_str.contains("Expected step 3") || err_str.contains("Expected Step 3"), "Error should mention expected step 3: {}", err_str);
        }

        #[test]
        fn test_sequential_numbering_wrong_start() {
            // Starting with 0 should fail during StepNumber parsing (caught earlier)
            let markdown = r#"
## 0. Zero step

```bash
echo "test"
```
"#;
            let result = parse_workflow(markdown);
            // This should fail during header parsing (StepNumber::new(0) returns None)
            assert!(result.is_err());
        }

        #[test]
        fn test_goto_validation_after_numbering() {
            // GOTO target validation should happen after sequential numbering check
            let markdown = r#"
## 1. First step

PASS: GOTO 3
FAIL: STOP

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```

## 3. Third step

```bash
echo "test"
```
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 3);
            // GOTO 3 should be valid
        }

        #[test]
        fn test_goto_target_beyond_workflow() {
            let markdown = r#"
## 1. First step

PASS: GOTO 10
FAIL: STOP

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```
"#;
            let result = parse_workflow(markdown);
            assert!(result.is_err());
            let err = result.unwrap_err();
            let err_str = err.to_string();
            assert!(err_str.contains("Step 1"), "Error should mention Step 1: {}", err_str);
            assert!(err_str.contains("GOTO") || err_str.contains("GoTo"), "Error should mention GOTO: {}", err_str);
            assert!(err_str.contains("Step 10") || err_str.contains("step 10"), "Error should mention target step 10: {}", err_str);
            assert!(err_str.contains("does not exist") || err_str.contains("workflow has 2 steps"), "Error should indicate workflow size: {}", err_str);
        }

        #[test]
        fn test_goto_valid_target() {
            let markdown = r#"
## 1. First step

PASS: GOTO 3
FAIL: GOTO 2

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```

## 3. Third step

```bash
echo "test"
```
"#;
            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps.len(), 3);
            // Both GOTO 2 and GOTO 3 should be valid
        }
    }

    mod conditionals {
        use super::*;

        #[test]
        fn test_parse_pass_fail_conditionals() {
            // This test now uses new ALLCAPS syntax
            let markdown = r#"
## 1. Run tests

PASS: CONTINUE
FAIL: STOP fix tests

```bash
mise run test
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Should have conditions with both PASS and FAIL
            assert!(steps[0].conditions.is_some());

            let conditions = steps[0].conditions.as_ref().unwrap();
            assert_eq!(conditions.pass, Action::Continue);
            match &conditions.fail {
                Action::Stop(message) => {
                    assert_eq!(message.as_deref(), Some("fix tests"));
                }
                _ => panic!("Expected Stop action"),
            }
        }

        #[test]
        fn test_parse_minimal_syntax_no_conditionals() {
            let markdown = r#"
## 1. Run tests

```bash
mise run test
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            // No explicit conditions = None
            assert!(steps[0].conditions.is_none());
        }

        // Task 2.2: ALLCAPS keyword tests
        #[test]
        fn test_parse_allcaps_keywords() {
            let markdown = r#"
## 1. Run tests

PASS: CONTINUE
FAIL: STOP fix tests first

```bash
mise run test
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert!(steps[0].conditions.is_some());

            let conditions = steps[0].conditions.as_ref().unwrap();
            assert_eq!(conditions.pass, Action::Continue);

            match &conditions.fail {
                Action::Stop(message) => {
                    assert_eq!(message.as_deref(), Some("fix tests first"));
                }
                _ => panic!("Expected Stop action"),
            }
        }

        #[test]
        fn test_reject_lowercase_keywords() {
            // Lowercase keywords should fail
            let markdown = r#"
## 1. Test

pass: CONTINUE
fail: STOP

```bash
echo "test"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Lowercase should not be recognized as conditionals
            assert!(steps[0].conditions.is_none());
        }

        #[test]
        fn test_permissive_separator_parsing() {
            // Test various separators: colon, space, dash
            let markdown_colon = r#"
## 1. Test

PASS: CONTINUE
FAIL: STOP

```bash
echo "test"
```
"#;

            let markdown_space = r#"
## 1. Test

PASS CONTINUE
FAIL STOP

```bash
echo "test"
```
"#;

            let markdown_dash = r#"
## 1. Test

PASS - CONTINUE
FAIL - STOP

```bash
echo "test"
```
"#;

            let steps_colon = parse_workflow(markdown_colon).unwrap();
            let steps_space = parse_workflow(markdown_space).unwrap();
            let steps_dash = parse_workflow(markdown_dash).unwrap();

            assert!(steps_colon[0].conditions.is_some());
            assert!(steps_space[0].conditions.is_some());
            assert!(steps_dash[0].conditions.is_some());
        }

        #[test]
        fn test_parse_goto_keyword() {
            // "GOTO N" not "Go to Step N"
            let markdown = r#"
## 1. Test

PASS: GOTO 3
FAIL: STOP

```bash
echo "test"
```

## 2. Skip

```bash
echo "skipped"
```

## 3. Target

```bash
echo "reached"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            let conditions = steps[0].conditions.as_ref().unwrap();
            match &conditions.pass {
                Action::Goto(num) => assert_eq!(num.get(), 3),
                _ => panic!("Expected Goto action"),
            }
        }

        #[test]
        fn test_parse_stop_with_message() {
            // "STOP message" not "STOP (message)"
            let markdown = r#"
## 1. Test

FAIL: STOP command failed

```bash
exit 1
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            let conditions = steps[0].conditions.as_ref().unwrap();
            match &conditions.fail {
                Action::Stop(Some(msg)) => assert_eq!(msg, "command failed"),
                _ => panic!("Expected Stop with message"),
            }
        }

        #[test]
        fn test_parse_stop_no_message() {
            let markdown = r#"
## 1. Test

FAIL: STOP

```bash
exit 1
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            let conditions = steps[0].conditions.as_ref().unwrap();
            match &conditions.fail {
                Action::Stop(None) => (),
                _ => panic!("Expected Stop without message"),
            }
        }

        // Task 2.3: List-based conditional tests
        #[test]
        fn test_parse_conditional_list() {
            // Test parsing conditionals from markdown list items
            // NOTE: pulldown_cmark parses list items and sends Text events for their content,
            // so this test verifies that list-based conditionals work through existing text parsing
            let markdown = r#"
## 1. Run tests

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP fix tests
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Verify list-based format produces correct parse
            assert!(steps[0].conditions.is_some(), "Should parse both PASS and FAIL from list");

            let conditions = steps[0].conditions.as_ref().unwrap();
            assert_eq!(conditions.pass, Action::Continue);

            match &conditions.fail {
                Action::Stop(Some(msg)) => {
                    assert_eq!(msg, "fix tests");
                }
                _ => panic!("Expected Stop action"),
            }
        }

        #[test]
        fn test_list_format_preserves_order() {
            // Verify that list items are parsed in order (PASS then FAIL)
            let markdown = r#"
## 1. Test

```bash
echo "test"
```

- PASS: GOTO 2
- FAIL: STOP error

## 2. Next
```bash
echo "next"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert!(steps[0].conditions.is_some());

            let conditions = steps[0].conditions.as_ref().unwrap();
            // Check PASS is GOTO 2
            assert!(matches!(conditions.pass, Action::Goto(_)));
            // Check FAIL is STOP
            assert!(matches!(conditions.fail, Action::Stop(_)));
        }

        #[test]
        fn test_detect_list_vs_paragraph_conditionals() {
            // Both paragraph and list formats should work
            let markdown_paragraph = r#"
## 1. Test

PASS: CONTINUE
FAIL: STOP

```bash
echo "test"
```
"#;

            let markdown_list = r#"
## 1. Test

```bash
echo "test"
```

- PASS: CONTINUE
- FAIL: STOP
"#;

            let steps_paragraph = parse_workflow(markdown_paragraph).unwrap();
            let steps_list = parse_workflow(markdown_list).unwrap();

            // Both should parse to same result
            assert!(steps_paragraph[0].conditions.is_some());
            assert!(steps_list[0].conditions.is_some());
        }

        #[test]
        fn test_nested_lists_not_supported() {
            // Nested lists should either error or only parse top-level
            let markdown = r#"
## 1. Test

```bash
echo "test"
```

- PASS: CONTINUE
  - Nested item (should be ignored or error)
- FAIL: STOP
"#;

            // Should still parse successfully, ignoring nested content
            let result = parse_workflow(markdown);
            assert!(result.is_ok());
            let steps = result.unwrap();
            // Should only get PASS and FAIL from top-level items
            assert!(steps[0].conditions.is_some());
        }
    }

    mod implicit_prompts {
        use super::*;

        // Task 2.4: Implicit prompt tests
        #[test]
        fn test_implicit_prompt_no_code_block() {
            // Steps without code blocks should treat paragraph text as prompts
            let markdown = r#"
## 1. Review code

Review the code changes carefully.
Are they atomic and focused?

- PASS: CONTINUE
- FAIL: GOTO 2

## 2. Fix issues
```bash
echo "fix"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Step 1 should have implicit prompts (no **Prompt:** needed)
            assert!(
                !steps[0].prompts.is_empty(),
                "Step without code block should have implicit prompts"
            );
            // Should contain the review questions
            let prompt_text = &steps[0].prompts[0].text;
            assert!(prompt_text.contains("Review the code"));
        }

        #[test]
        fn test_command_with_code_block() {
            // Steps with code blocks should execute command, not prompt
            let markdown = r#"
## 1. Run tests

```bash
mise run test
```

- PASS: CONTINUE
- FAIL: STOP
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Should have command
            assert!(steps[0].command.is_some());
            // Should NOT have implicit prompts (has code block)
            assert!(steps[0].prompts.is_empty());
        }

        #[test]
        fn test_control_flow_only_step() {
            // Steps with only conditionals should be valid (no prompt or command)
            let markdown = r#"
## 1. Branch decision

- PASS: GOTO 2
- FAIL: GOTO 3

## 2. Pass path
```bash
echo "pass"
```

## 3. Fail path
```bash
echo "fail"
```
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Step 1 has no command and no prompts - just control flow
            assert!(steps[0].command.is_none());
            assert!(steps[0].prompts.is_empty());
            assert!(steps[0].conditions.is_some());
        }

        #[test]
        fn test_explicit_prompt_still_works() {
            // Explicit **Prompt:** should still work for backward compatibility
            let markdown = r#"
## 1. Check something

**Prompt:** Is everything ready?

- PASS: CONTINUE
- FAIL: STOP
"#;

            let steps = parse_workflow(markdown).unwrap();
            assert_eq!(steps[0].prompts.len(), 1);
            assert_eq!(steps[0].prompts[0].text, "Is everything ready?");
        }

        #[test]
        fn test_mixed_explicit_and_implicit() {
            // Can have both explicit prompts and implicit text
            let markdown = r#"
## 1. Review

Some context about the review.

**Prompt:** Are you ready to proceed?

Additional information here.

- PASS: CONTINUE
- FAIL: STOP
"#;

            let steps = parse_workflow(markdown).unwrap();
            // Should have explicit prompt
            assert!(steps[0].prompts.iter().any(|p| p.text.contains("ready to proceed")));
            // Should also capture implicit text (implementation detail)
        }
    }
}
