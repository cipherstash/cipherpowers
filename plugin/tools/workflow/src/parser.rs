use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};
use crate::models::*;
use anyhow::Result;

pub fn parse_workflow(markdown: &str) -> Result<Vec<Step>> {
    let parser = Parser::new(markdown);
    let mut steps = Vec::new();
    let mut current_step: Option<Step> = None;

    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
                if let Some(step) = current_step.take() {
                    steps.push(step);
                }
            }
            Event::Text(text) => {
                if let Some(captures) = extract_step_header(&text) {
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
}
