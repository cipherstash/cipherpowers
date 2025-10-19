use crate::models::*;
use anyhow::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

pub fn parse_workflow(markdown: &str) -> Result<Vec<Step>> {
    let parser = Parser::new(markdown);
    let mut steps = Vec::new();
    let mut current_step: Option<Step> = None;
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_lang = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
                if let Some(step) = current_step.take() {
                    steps.push(step);
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_content.clear();
                if let pulldown_cmark::CodeBlockKind::Fenced(lang) = kind {
                    code_block_lang = lang.to_string();
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                if code_block_lang.starts_with("bash") {
                    let quiet = code_block_lang.contains("quiet");
                    if let Some(step) = current_step.as_mut() {
                        step.commands.push(Command {
                            code: code_block_content.trim().to_string(),
                            quiet,
                        });
                    }
                }
                code_block_content.clear();
                code_block_lang.clear();
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else if let Some(captures) = extract_step_header(&text) {
                    current_step = Some(Step {
                        number: captures.0,
                        description: captures.1,
                        commands: Vec::new(),
                        prompts: Vec::new(),
                        conditionals: Vec::new(),
                    });
                }
            }
            _ => {}
        }
    }

    if let Some(step) = current_step {
        steps.push(step);
    }

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

#[cfg(test)]
mod tests {
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
        assert_eq!(steps[0].commands.len(), 1);
        assert_eq!(steps[0].commands[0].code, "mise run test");
        assert_eq!(steps[0].commands[0].quiet, false);

        assert_eq!(steps[1].commands.len(), 1);
        assert_eq!(steps[1].commands[0].code, "git status");
        assert_eq!(steps[1].commands[0].quiet, true);
    }
}
