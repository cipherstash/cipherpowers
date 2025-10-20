use anyhow::Result;
use clap::Parser;
use std::fs;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod execution_mode;
pub mod executor;
pub mod models;
pub mod parser;
pub mod runner;

#[derive(Parser, Debug)]
#[command(name = "workflow")]
#[command(about = "Execute markdown-based workflows with deterministic control flow")]
#[command(
    long_about = "Execute markdown workflows in enforcement mode (sequential, STOP only) or guided mode (full control flow). Enforcement prevents rationalization, guided enables flexibility."
)]
struct Args {
    /// Path to workflow markdown file
    workflow_file: String,

    /// Enable guided mode for flexible execution
    #[arg(
        long,
        help = "Enable guided mode (allows Continue/GoTo conditionals). Default is enforcement mode (sequential, STOP only)."
    )]
    guided: bool,

    /// Show steps without executing
    #[arg(long)]
    dry_run: bool,

    /// List all steps
    #[arg(long)]
    list: bool,

    /// Validate workflow structure without executing
    #[arg(long, help = "Validate workflow structure without executing")]
    validate: bool,
}

fn main() -> Result<()> {
    // Initialize tracing (respects RUST_LOG env var)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "workflow=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // Read workflow file
    let markdown = fs::read_to_string(&args.workflow_file)?;

    // Handle --validate flag (validate only, no execution)
    if args.validate {
        match parser::parse_workflow(&markdown) {
            Ok(_) => {
                println!("✓ Workflow is valid");
                return Ok(());
            }
            Err(e) => {
                eprintln!("✗ Workflow validation failed: {}", e);
                std::process::exit(1);
            }
        }
    }

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
            println!("Step {}: {}", step.number.get(), step.description);
            println!("  Commands: {}", step.command.as_ref().map_or(0, |_| 1));
            println!("  Prompts: {}", step.prompts.len());
            println!("  Conditions: {}", if step.conditions.is_some() { "Yes" } else { "No" });
        }
        return Ok(());
    }

    // Run workflow (both normal and dry-run modes)
    println!("→ Workflow: {}", args.workflow_file);
    println!("→ Steps: {}", steps.len());

    if args.dry_run {
        println!("→ Mode: Dry-run (simulated execution)\n");
    }

    let mode = if args.guided {
        execution_mode::ExecutionMode::Guided
    } else {
        execution_mode::ExecutionMode::Enforcement
    };

    let mut runner = runner::WorkflowRunner::new(steps, mode);

    // Set dry-run mode if requested
    if args.dry_run {
        runner.set_dry_run(true);
    }

    let result = match runner.run() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("\n→ Workflow execution error: {}", e);
            std::process::exit(4); // Distinct code for execution errors
        }
    };

    match result {
        runner::ExecutionResult::Success => {
            std::process::exit(0);
        }
        runner::ExecutionResult::Stopped { message } => {
            if let Some(msg) = message {
                println!("\n→ Workflow stopped: {}", msg);
            } else {
                println!("\n→ Workflow stopped");
            }
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
    use super::*;

    #[test]
    fn test_end_to_end_workflow() {
        let workflow = r#"
## 1. Test echo

```bash
echo "test output"
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP
"#;

        let steps = crate::parser::parse_workflow(workflow).unwrap();
        let mut runner = crate::runner::WorkflowRunner::new(
            steps,
            crate::execution_mode::ExecutionMode::Enforcement,
        );
        let result = runner.run().unwrap();

        assert_eq!(result, crate::runner::ExecutionResult::Success);
    }

    #[test]
    fn test_end_to_end_workflow_with_stop() {
        let workflow = r#"
## 1. Test failure

FAIL: STOP Command failed as expected

```bash
exit 1
```
"#;

        let steps = crate::parser::parse_workflow(workflow).unwrap();
        let mut runner = crate::runner::WorkflowRunner::new(
            steps,
            crate::execution_mode::ExecutionMode::Enforcement,
        );
        let result = runner.run().unwrap();

        assert_eq!(
            result,
            crate::runner::ExecutionResult::Stopped {
                message: Some("Command failed as expected".to_string())
            }
        );
    }

    #[test]
    fn test_guided_mode_flag_parsing() {
        // This will compile once we add the flag
        let args = Args::parse_from(vec!["workflow", "--guided", "test.md"]);
        assert!(args.guided);
    }

    // Task 3.2: --validate flag tests
    #[test]
    fn test_validate_flag_parsing() {
        let args = Args::parse_from(vec!["workflow", "--validate", "test.md"]);
        assert!(args.validate);
    }

    #[test]
    fn test_validate_flag_success() {
        // Valid workflow should validate without error
        let valid_workflow = r#"
## 1. First step

```bash
echo "test"
```

## 2. Second step

```bash
echo "test"
```
"#;
        let result = crate::parser::parse_workflow(valid_workflow);
        assert!(result.is_ok(), "Valid workflow should parse successfully");
    }

    #[test]
    fn test_validate_flag_catches_errors() {
        // Invalid workflow should fail validation
        let invalid_workflow = r#"
## 1. First step

```bash
echo "test"
```

## 4. Fourth step (missing step 2 and 3)

```bash
echo "test"
```
"#;
        let result = crate::parser::parse_workflow(invalid_workflow);
        assert!(result.is_err(), "Invalid workflow should fail validation");
        let err = result.unwrap_err();
        assert!(err.to_string().contains("sequential"), "Error should mention sequential numbering");
    }

    // Task 4.3: --dry-run flag tests
    #[test]
    fn test_dry_run_flag_parsing() {
        let args = Args::parse_from(vec!["workflow", "--dry-run", "test.md"]);
        assert!(args.dry_run);
    }
}
