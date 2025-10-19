use clap::Parser;
use anyhow::Result;
use std::fs;

mod executor;
mod models;
mod parser;
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

#[cfg(test)]
mod integration_tests {
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

        let steps = crate::parser::parse_workflow(workflow).unwrap();
        let mut runner = crate::runner::WorkflowRunner::new(steps);
        let result = runner.run().unwrap();

        assert_eq!(result, crate::runner::ExecutionResult::Success);
    }
}
