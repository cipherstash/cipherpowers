# Workflow Executor Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Build a Rust CLI tool that executes markdown-based workflow files with shell commands, conditionals, and interactive prompts.

**Architecture:** Single Rust binary that parses markdown files using conventional syntax (headers=steps, code blocks=commands, arrows=conditionals, bold=prompts), executes commands sequentially with configurable output, and handles control flow (continue/stop/jump).

**Tech Stack:** Rust, pulldown-cmark (markdown parsing), clap (CLI), subprocess execution

---

## Task 1: Project Scaffolding

**Files:**
- Create: `plugin/tools/workflow/Cargo.toml`
- Create: `plugin/tools/workflow/src/main.rs`
- Create: `plugin/tools/workflow/README.md`

**Step 1: Create Cargo project**

```bash
cd plugin/tools
cargo new workflow
```

**Step 2: Configure Cargo.toml with dependencies**

Edit `plugin/tools/workflow/Cargo.toml`:

```toml
[package]
name = "workflow"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "workflow"
path = "src/main.rs"

[dependencies]
pulldown-cmark = "0.9"
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
```

**Step 3: Write minimal main.rs that prints args**

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows", long_about = None)]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,
}

fn main() {
    let args = Args::parse();
    println!("Workflow file: {}", args.workflow_file);
}
```

**Step 4: Test binary runs**

```bash
cd plugin/tools/workflow
cargo run -- test.md
```

Expected output: `Workflow file: test.md`

**Step 5: Write README**

Create `plugin/tools/workflow/README.md`:

```markdown
# Workflow Executor

Executes markdown-based workflows with shell commands, conditionals, and prompts.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
workflow path/to/workflow.md
```

See `docs/plans/2025-10-19-workflow-executor.md` for implementation details.
```

**Step 6: Commit**

```bash
git add plugin/tools/workflow/
git commit -m "feat(workflow): add project scaffolding"
```

---

## Task 2: Markdown Parser - Extract Steps

**Files:**
- Create: `plugin/tools/workflow/src/parser.rs`
- Modify: `plugin/tools/workflow/src/main.rs`
- Create: `plugin/tools/workflow/src/models.rs`

**Step 1: Write failing test for step extraction**

Create `plugin/tools/workflow/src/parser.rs`:

```rust
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
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_steps_from_markdown
```

Expected: Compilation error - `parse_workflow` doesn't exist

**Step 3: Create models for workflow data structures**

Create `plugin/tools/workflow/src/models.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Workflow {
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub number: usize,
    pub description: String,
    pub commands: Vec<Command>,
    pub prompts: Vec<Prompt>,
    pub conditionals: Vec<Conditional>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub code: String,
    pub quiet: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conditional {
    ExitCode { code: i32, action: Action },
    ExitNotZero { action: Action },
    OutputEmpty { action: Action },
    OutputContains { text: String, action: Action },
    Otherwise { action: Action },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,
    Stop { message: Option<String> },
    GoToStep { number: usize },
}
```

**Step 4: Implement minimal parse_workflow**

Add to `plugin/tools/workflow/src/parser.rs`:

```rust
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
```

**Step 5: Add mod declarations to main.rs**

Add to `plugin/tools/workflow/src/main.rs`:

```rust
mod models;
mod parser;
```

**Step 6: Run test to verify it passes**

```bash
cargo test test_parse_steps_from_markdown
```

Expected: PASS

**Step 7: Commit**

```bash
git add plugin/tools/workflow/src/
git commit -m "feat(workflow): parse steps from markdown headers"
```

---

## Task 3: Parse Code Blocks as Commands

**Files:**
- Modify: `plugin/tools/workflow/src/parser.rs`

**Step 1: Write failing test for command extraction**

Add to `plugin/tools/workflow/src/parser.rs` tests:

```rust
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
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_commands_in_steps
```

Expected: FAIL - commands Vec is empty

**Step 3: Implement command parsing**

Update `parse_workflow` in `parser.rs`:

```rust
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
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_parse_commands_in_steps
```

Expected: PASS

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/parser.rs
git commit -m "feat(workflow): parse bash code blocks as commands"
```

---

## Task 4: Parse Prompts

**Files:**
- Modify: `plugin/tools/workflow/src/parser.rs`

**Step 1: Write failing test for prompt extraction**

Add to tests in `parser.rs`:

```rust
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
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_prompts
```

Expected: FAIL - prompts Vec is empty

**Step 3: Implement prompt parsing**

Update `parse_workflow` to track strong emphasis and extract prompts:

```rust
pub fn parse_workflow(markdown: &str) -> Result<Vec<Step>> {
    let parser = Parser::new(markdown);
    let mut steps = Vec::new();
    let mut current_step: Option<Step> = None;
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_lang = String::new();
    let mut in_strong = false;
    let mut strong_text = String::new();

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
            Event::Start(Tag::Strong) => {
                in_strong = true;
                strong_text.clear();
            }
            Event::End(Tag::Strong) => {
                in_strong = false;
                // Check if this is a prompt
                if strong_text.trim() == "Prompt:" {
                    // Next text event will contain the prompt text
                    // We'll handle this in the next Text event
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else if in_strong {
                    strong_text.push_str(&text);
                } else if let Some(prompt_text) = extract_prompt(&text) {
                    if let Some(step) = current_step.as_mut() {
                        step.prompts.push(Prompt {
                            text: prompt_text,
                        });
                    }
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

fn extract_prompt(text: &str) -> Option<String> {
    // Look for pattern after "Prompt:"
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Simple heuristic: if previous strong was "Prompt:", this is the prompt text
    // For now, we'll use a regex-like approach
    if let Some(idx) = text.find("Prompt:") {
        let prompt = text[idx + 7..].trim();
        if !prompt.is_empty() {
            return Some(prompt.to_string());
        }
    }
    None
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_parse_prompts
```

Expected: PASS

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/parser.rs
git commit -m "feat(workflow): parse bold prompts"
```

---

## Task 5: Parse Conditionals

**Files:**
- Modify: `plugin/tools/workflow/src/parser.rs`

**Step 1: Write failing test for conditional parsing**

Add to tests:

```rust
#[test]
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
        Conditional::ExitNotZero { action } => {
            match action {
                Action::Stop { message } => {
                    assert_eq!(message.as_deref(), Some("fix tests"));
                }
                _ => panic!("Expected Stop action"),
            }
        }
        _ => panic!("Expected ExitNotZero conditional"),
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_parse_conditionals
```

Expected: FAIL - conditionals Vec is empty

**Step 3: Implement conditional parsing**

Add conditional parsing to `parse_workflow`:

```rust
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
                } else if let Some(conditional) = parse_conditional(&text) {
                    if let Some(step) = current_step.as_mut() {
                        step.conditionals.push(conditional);
                    }
                } else if let Some(prompt_text) = extract_prompt(&text) {
                    if let Some(step) = current_step.as_mut() {
                        step.prompts.push(Prompt {
                            text: prompt_text,
                        });
                    }
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

fn parse_conditional(text: &str) -> Option<Conditional> {
    let trimmed = text.trim();
    if !trimmed.starts_with("→") && !trimmed.starts_with("->") {
        return None;
    }

    // Remove arrow prefix
    let content = trimmed.strip_prefix("→")
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

    // Parse condition type
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
        let text = condition.strip_prefix("If output contains")?
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
        let message = trimmed.strip_prefix("STOP (")?
            .strip_suffix(")")?
            .to_string();
        Some(Action::Stop { message: Some(message) })
    } else if trimmed.starts_with("Go to Step ") {
        let number: usize = trimmed.strip_prefix("Go to Step ")?.trim().parse().ok()?;
        Some(Action::GoToStep { number })
    } else {
        None
    }
}
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_parse_conditionals
```

Expected: PASS

**Step 5: Commit**

```bash
git add plugin/tools/workflow/src/parser.rs
git commit -m "feat(workflow): parse arrow conditionals"
```

---

## Task 6: Command Executor - Basic Execution

**Files:**
- Create: `plugin/tools/workflow/src/executor.rs`
- Modify: `plugin/tools/workflow/src/main.rs`

**Step 1: Write failing test for command execution**

Create `plugin/tools/workflow/src/executor.rs`:

```rust
use crate::models::*;
use std::process::Command as ProcessCommand;
use anyhow::Result;

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

pub fn execute_command(cmd: &Command) -> Result<CommandOutput> {
    // TODO: implement
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_simple_command() {
        let cmd = Command {
            code: "echo 'hello'".to_string(),
            quiet: false,
        };

        let output = execute_command(&cmd).unwrap();
        assert!(output.success);
        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("hello"));
    }

    #[test]
    fn test_execute_failing_command() {
        let cmd = Command {
            code: "exit 1".to_string(),
            quiet: false,
        };

        let output = execute_command(&cmd).unwrap();
        assert!(!output.success);
        assert_eq!(output.exit_code, 1);
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_execute_simple_command
```

Expected: FAIL - `unimplemented!()`

**Step 3: Implement command execution**

Update `execute_command` in `executor.rs`:

```rust
pub fn execute_command(cmd: &Command) -> Result<CommandOutput> {
    let output = ProcessCommand::new("sh")
        .arg("-c")
        .arg(&cmd.code)
        .output()?;

    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
        success: output.status.success(),
    })
}
```

**Step 4: Add mod declaration to main.rs**

```rust
mod executor;
```

**Step 5: Run test to verify it passes**

```bash
cargo test test_execute_simple_command
cargo test test_execute_failing_command
```

Expected: Both PASS

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/executor.rs plugin/tools/workflow/src/main.rs
git commit -m "feat(workflow): add command executor"
```

---

## Task 7: Workflow Runner - Step Execution

**Files:**
- Create: `plugin/tools/workflow/src/runner.rs`
- Modify: `plugin/tools/workflow/src/main.rs`

**Step 1: Write failing test for step execution**

Create `plugin/tools/workflow/src/runner.rs`:

```rust
use crate::models::*;
use crate::executor::{execute_command, CommandOutput};
use anyhow::Result;

pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
}

impl WorkflowRunner {
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            current_step: 0,
        }
    }

    pub fn run(&mut self) -> Result<ExecutionResult> {
        // TODO: implement
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Success,
    Stopped { message: Option<String> },
    UserCancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple_workflow() {
        let steps = vec![
            Step {
                number: 1,
                description: "Echo test".to_string(),
                commands: vec![
                    Command {
                        code: "echo 'step 1'".to_string(),
                        quiet: false,
                    }
                ],
                prompts: vec![],
                conditionals: vec![
                    Conditional::ExitCode {
                        code: 0,
                        action: Action::Continue,
                    }
                ],
            },
        ];

        let mut runner = WorkflowRunner::new(steps);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_run_simple_workflow
```

Expected: FAIL - `unimplemented!()`

**Step 3: Implement basic workflow execution**

Update `runner.rs`:

```rust
impl WorkflowRunner {
    pub fn run(&mut self) -> Result<ExecutionResult> {
        while self.current_step < self.steps.len() {
            let step = &self.steps[self.current_step];

            println!("\n→ Step {}: {}", step.number, step.description);

            // Execute commands
            for command in &step.commands {
                println!("→ Executing: {}", command.code);

                let output = execute_command(command)?;

                // Show output based on quiet flag
                if !command.quiet || !output.success {
                    print!("{}", output.stdout);
                    print!("{}", output.stderr);
                }

                // Status
                let status_symbol = if output.success { "✓" } else { "✗" };
                let status_text = if output.success { "Passed" } else { "Failed" };
                println!("{} {} (exit {})", status_symbol, status_text, output.exit_code);

                // Evaluate conditionals
                let action = self.evaluate_conditionals(&step.conditionals, &output)?;

                match action {
                    Some(Action::Continue) => {
                        println!("→ Condition matched: Continue");
                    }
                    Some(Action::Stop { message }) => {
                        if let Some(msg) = message {
                            println!("→ Condition matched: STOP ({})", msg);
                            return Ok(ExecutionResult::Stopped { message: Some(msg) });
                        } else {
                            println!("→ Condition matched: STOP");
                            return Ok(ExecutionResult::Stopped { message: None });
                        }
                    }
                    Some(Action::GoToStep { number }) => {
                        println!("→ Condition matched: Go to Step {}", number);
                        self.current_step = self.find_step_index(number)?;
                        continue;
                    }
                    None => {
                        // No matching conditional - continue
                    }
                }
            }

            // Execute prompts
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

            self.current_step += 1;
        }

        println!("\n→ Workflow completed successfully");
        Ok(ExecutionResult::Success)
    }

    fn evaluate_conditionals(&self, conditionals: &[Conditional], output: &CommandOutput) -> Result<Option<Action>> {
        for conditional in conditionals {
            match conditional {
                Conditional::ExitCode { code, action } => {
                    if output.exit_code == *code {
                        return Ok(Some(action.clone()));
                    }
                }
                Conditional::ExitNotZero { action } => {
                    if output.exit_code != 0 {
                        return Ok(Some(action.clone()));
                    }
                }
                Conditional::OutputEmpty { action } => {
                    if output.stdout.trim().is_empty() {
                        return Ok(Some(action.clone()));
                    }
                }
                Conditional::OutputContains { text, action } => {
                    if output.stdout.contains(text) || output.stderr.contains(text) {
                        return Ok(Some(action.clone()));
                    }
                }
                Conditional::Otherwise { action } => {
                    return Ok(Some(action.clone()));
                }
            }
        }
        Ok(None)
    }

    fn find_step_index(&self, number: usize) -> Result<usize> {
        self.steps.iter()
            .position(|s| s.number == number)
            .ok_or_else(|| anyhow::anyhow!("Step {} not found", number))
    }
}
```

**Step 4: Add mod declaration to main.rs**

```rust
mod runner;
```

**Step 5: Run test to verify it passes**

```bash
cargo test test_run_simple_workflow
```

Expected: PASS

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/runner.rs plugin/tools/workflow/src/main.rs
git commit -m "feat(workflow): add workflow runner with step execution"
```

---

## Task 8: CLI Integration - Wire Up Parser and Runner

**Files:**
- Modify: `plugin/tools/workflow/src/main.rs`

**Step 1: Write integration test**

Add to `main.rs`:

```rust
#[cfg(test)]
mod integration_tests {
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_end_to_end_workflow() {
        let workflow = r#"
# Step 1: Test echo

```bash
echo "test output"
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP
"#;

        let temp_file = "/tmp/test_workflow.md";
        fs::write(temp_file, workflow).unwrap();

        // This will be implemented when we wire up main
        // For now, just test that parser + runner work together
        let steps = crate::parser::parse_workflow(workflow).unwrap();
        let mut runner = crate::runner::WorkflowRunner::new(steps);
        let result = runner.run().unwrap();

        assert_eq!(result, crate::runner::ExecutionResult::Success);
    }
}
```

**Step 2: Run test to verify it passes**

```bash
cargo test test_end_to_end_workflow
```

Expected: PASS

**Step 3: Update main.rs to wire everything together**

Update `main.rs`:

```rust
use clap::Parser;
use anyhow::Result;
use std::fs;

mod models;
mod parser;
mod executor;
mod runner;

#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows", long_about = None)]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,

    /// Show steps without executing
    #[arg(long)]
    dry_run: bool,

    /// Start from specific step number
    #[arg(long)]
    start_step: Option<usize>,

    /// Verbose output
    #[arg(long, short)]
    verbose: bool,

    /// List all steps
    #[arg(long)]
    list: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read workflow file
    let markdown = fs::read_to_string(&args.workflow_file)?;

    // Parse workflow
    let steps = parser::parse_workflow(&markdown)?;

    if steps.is_empty() {
        eprintln!("Error: No steps found in workflow file");
        std::process::exit(3);
    }

    // Handle --list flag
    if args.list {
        println!("→ Workflow: {}", args.workflow_file);
        println!("→ Steps: {}\n", steps.len());
        for step in &steps {
            println!("Step {}: {}", step.number, step.description);
            println!("  Commands: {}", step.commands.len());
            println!("  Prompts: {}", step.prompts.len());
            println!("  Conditionals: {}", step.conditionals.len());
        }
        return Ok(());
    }

    // Handle --dry-run flag
    if args.dry_run {
        println!("→ Workflow: {}", args.workflow_file);
        println!("→ Steps: {}\n", steps.len());
        for step in &steps {
            println!("Step {}: {}", step.number, step.description);
            for cmd in &step.commands {
                println!("  Would execute: {}", cmd.code);
            }
            for prompt in &step.prompts {
                println!("  Would prompt: {}", prompt.text);
            }
        }
        return Ok(());
    }

    // Run workflow
    println!("→ Workflow: {}", args.workflow_file);
    println!("→ Steps: {}", steps.len());

    let mut runner = runner::WorkflowRunner::new(steps);
    let result = runner.run()?;

    match result {
        runner::ExecutionResult::Success => {
            std::process::exit(0);
        }
        runner::ExecutionResult::Stopped { message: _ } => {
            println!("\n→ Workflow stopped");
            std::process::exit(1);
        }
        runner::ExecutionResult::UserCancelled => {
            println!("\n→ Workflow cancelled by user");
            std::process::exit(2);
        }
    }
}
```

**Step 4: Test with a real workflow file**

Create test workflow:

```bash
cat > /tmp/test.md << 'EOF'
# Step 1: Test command

```bash
echo "Hello from workflow"
```

→ Exit 0: Continue
EOF
```

Run it:

```bash
cargo run -- /tmp/test.md
```

Expected: Output shows step execution and "Hello from workflow"

**Step 5: Test --list flag**

```bash
cargo run -- --list /tmp/test.md
```

Expected: Shows workflow summary

**Step 6: Test --dry-run flag**

```bash
cargo run -- --dry-run /tmp/test.md
```

Expected: Shows what would execute without running

**Step 7: Commit**

```bash
git add plugin/tools/workflow/src/main.rs
git commit -m "feat(workflow): wire up CLI with parser and runner"
```

---

## Task 9: Test with Real git-commit-algorithm

**Files:**
- None (testing only)

**Step 1: Run workflow tool on actual git-commit-algorithm**

```bash
cd plugin/tools/workflow
cargo build --release
./target/release/workflow ../../../plugin/practices/git-commit-algorithm.md
```

**Step 2: Verify it parses correctly**

```bash
./target/release/workflow --list ../../../plugin/practices/git-commit-algorithm.md
```

Expected: Shows all 10 steps

**Step 3: Test dry-run mode**

```bash
./target/release/workflow --dry-run ../../../plugin/practices/git-commit-algorithm.md
```

Expected: Shows execution plan

**Step 4: Document any parsing issues found**

If the real workflow file doesn't parse correctly, note the issues for fixing in Task 10.

Common issues to watch for:
- Step numbering format variations
- Conditional syntax variations
- Code block language variations

**Step 5: No commit needed** (testing only)

---

## Task 10: Bug Fixes and Edge Cases

**Files:**
- Modify: `plugin/tools/workflow/src/parser.rs`
- Modify: `plugin/tools/workflow/src/runner.rs`

**Step 1: Fix any issues found in Task 9**

Based on testing with git-commit-algorithm.md, fix parsing or execution issues.

Example fixes might include:
- Handle steps without numbers
- Handle missing conditionals
- Handle empty code blocks
- Better error messages

**Step 2: Add error handling for missing step references**

Add test:

```rust
#[test]
fn test_invalid_goto_step() {
    let steps = vec![
        Step {
            number: 1,
            description: "Test".to_string(),
            commands: vec![],
            prompts: vec![],
            conditionals: vec![
                Conditional::ExitCode {
                    code: 0,
                    action: Action::GoToStep { number: 99 },
                }
            ],
        },
    ];

    let mut runner = WorkflowRunner::new(steps);
    let result = runner.run();
    assert!(result.is_err());
}
```

**Step 3: Add validation for workflow structure**

Ensure:
- At least one step exists
- Step numbers are valid
- GoToStep references exist
- No infinite loops (optional - can detect simple cases)

**Step 4: Improve error messages**

Make errors helpful:
```
Error: workflow.md:23: Step 5 references "Go to Step 10" but workflow only has 7 steps
Error: workflow.md:15: No command found in step
```

**Step 5: Run full test suite**

```bash
cargo test
```

Expected: All tests pass

**Step 6: Commit**

```bash
git add plugin/tools/workflow/src/
git commit -m "fix(workflow): add validation and improve error handling"
```

---

## Task 11: Documentation and Installation

**Files:**
- Modify: `plugin/tools/workflow/README.md`
- Create: `plugin/tools/workflow/examples/simple.md`
- Modify: Root `README.md` or plugin docs

**Step 1: Write comprehensive README**

Update `plugin/tools/workflow/README.md`:

```markdown
# Workflow Executor

Execute markdown-based workflows with shell commands, conditionals, and interactive prompts.

## Why?

Workflows documented in markdown stay readable AND executable. Single source of truth - no duplication between docs and scripts.

**Problem:** LLM agents don't consistently follow algorithmic workflows despite testing.
**Solution:** Execute workflows deterministically - no cognitive load, no rationalization.

## Installation

```bash
cd plugin/tools/workflow
cargo install --path .
```

Or from anywhere in the cipherpowers repo:

```bash
cargo install --path plugin/tools/workflow
```

## Usage

```bash
# Run a workflow
workflow path/to/workflow.md

# Dry run (show steps without executing)
workflow --dry-run workflow.md

# List all steps
workflow --list workflow.md

# Start from specific step
workflow --start-step 3 workflow.md
```

## Workflow Syntax

Workflows are standard markdown with simple conventions:

### Steps (Headers)

```markdown
# Step 1: Description of step
# Step 2: Another step
```

### Commands (Code Blocks)

```markdown
# Step 1: Run tests

```bash
mise run test
```
```

By default, commands show full output. Add `quiet` flag to only show failures:

```markdown
```bash quiet
git diff --check
```
```

### Conditionals (Arrow Notation)

```markdown
→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests)
→ If output empty: STOP (nothing to commit)
→ If output contains "error": STOP
→ Otherwise: Continue
→ Exit 0: Go to Step 5
```

### Prompts (Bold)

```markdown
**Prompt:** Do all functions have tests?
```

Prompts always wait for y/n input. Answering 'n' or pressing Enter stops the workflow.

### Example Workflow

```markdown
# Step 1: Check for changes

```bash
git status --porcelain
```

→ If output empty: STOP (nothing to commit)
→ Otherwise: Continue

# Step 2: Verify test coverage

**Prompt:** Do ALL new/modified functions have tests?

# Step 3: Run tests

```bash
mise run test
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (fix tests first)

# Step 4: Commit

```bash
git commit
```
```

## Exit Codes

- `0` - Workflow completed successfully
- `1` - Workflow stopped due to failed condition
- `2` - User answered 'no' to prompt
- `3` - Error parsing or executing workflow

## Integration with CipherPowers

The workflow tool executes algorithmic workflows defined in `plugin/practices/`:

```bash
workflow plugin/practices/git-commit-algorithm.md
```

Agents can call this tool directly instead of trying to follow the algorithm themselves.

## Design

See implementation plan: `docs/plans/2025-10-19-workflow-executor.md`

## Philosophy

**Automate what can be automated.** Reserve human/LLM judgment for what requires it.

If the logic is algorithmic (binary decisions, shell commands, conditionals), execute it deterministically. Don't ask LLMs to "follow" algorithms - they rationalize under pressure.

Evidence: Algorithmic enforcement achieves 100% compliance vs 0-33% with imperative instructions.
```

**Step 2: Create example workflow**

Create `plugin/tools/workflow/examples/simple.md`:

```markdown
# Step 1: Greet user

```bash
echo "Hello from workflow executor!"
```

→ Exit 0: Continue

# Step 2: Check directory

```bash quiet
ls -la
```

→ Exit 0: Continue

# Step 3: Confirm

**Prompt:** Does everything look good?

# Step 4: Done

```bash
echo "Workflow complete!"
```
```

**Step 3: Test example workflow**

```bash
cargo run -- examples/simple.md
```

Expected: Executes successfully with prompts

**Step 4: Update main cipherpowers README**

Add section about workflow tool to main README or plugin documentation.

**Step 5: Commit**

```bash
git add plugin/tools/workflow/README.md plugin/tools/workflow/examples/
git commit -m "docs(workflow): add comprehensive documentation and examples"
```

---

## Task 12: Integration with CipherPowers Commands

**Files:**
- Create: `plugin/commands/workflow.md` (optional - if you want a slash command)
- Modify: `plugin/commands/commit.md` (optional - to reference workflow tool)

**Step 1: Consider if slash command is needed**

Decision: Do you want `/workflow` command or just direct binary usage?

Direct binary:
```bash
workflow plugin/practices/git-commit-algorithm.md
```

Slash command approach:
```markdown
Use the workflow tool: workflow @git-commit-algorithm.md
```

For now, recommend **direct binary usage** - simpler, no extra abstraction needed.

**Step 2: Update commit command to reference workflow tool**

Add to `plugin/commands/commit.md` (optional):

```markdown
**Note:** You can also use the workflow executor directly:

```bash
workflow plugin/practices/git-commit-algorithm.md
```

This executes the algorithm deterministically without requiring agent interpretation.
```

**Step 3: No commit needed if no changes**

---

## Summary

**What we built:**
- Rust CLI tool that executes markdown workflows
- Parses steps, commands, conditionals, prompts from markdown
- Deterministic execution with configurable output
- Error handling and validation
- Integration with CipherPowers practices

**How to use:**
```bash
cargo install --path plugin/tools/workflow
workflow plugin/practices/git-commit-algorithm.md
```

**Next steps:**
- Use workflow tool for git-commit-algorithm
- Create additional workflows for other practices
- Consider adding workflow execution to agents (they invoke the tool)
- Test under pressure scenarios to verify 100% compliance

**Files created:**
- `plugin/tools/workflow/` - Complete Rust project
- `plugin/tools/workflow/examples/simple.md` - Example workflow
- `docs/plans/2025-10-19-workflow-executor.md` - This plan
