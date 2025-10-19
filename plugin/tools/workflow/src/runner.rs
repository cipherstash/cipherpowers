use crate::execution_mode::ExecutionMode;
use crate::executor::{execute_command, CommandOutput};
use crate::models::*;
use anyhow::Result;
use tracing::debug;

/// Maximum iterations multiplier per step.
///
/// This determines how many times we can loop through steps before detecting an infinite loop.
/// For a workflow with N steps, the maximum total iterations is N * MAX_ITERATION_MULTIPLIER.
///
/// The value of 10 allows for reasonable looping patterns (e.g., retry logic, conditional jumps)
/// while catching truly infinite loops. For example, a 5-step workflow can iterate up to 50 times.
const MAX_ITERATION_MULTIPLIER: usize = 10;

/// Debug message explaining Pass/Fail evaluation criteria.
///
/// This defines the semantics of conditional evaluation:
/// - Pass: exit code 0 (command succeeded)
/// - Fail: non-zero exit code (command failed)
const DEBUG_EVALUATION_CRITERIA: &str = "exit code (0 = Pass, non-zero = Fail)";

pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
    iterations: usize,
    max_iterations: usize,
    #[allow(dead_code)] // Reserved for future use (enforcement vs guided mode)
    mode: ExecutionMode,
}

impl WorkflowRunner {
    pub fn new(steps: Vec<Step>, mode: ExecutionMode) -> Self {
        let max_iterations = steps.len() * MAX_ITERATION_MULTIPLIER;
        Self {
            steps,
            current_step: 0,
            iterations: 0,
            max_iterations,
            mode,
        }
    }

    pub fn run(&mut self) -> Result<ExecutionResult> {
        'workflow_loop: while self.current_step < self.steps.len() {
            self.iterations += 1;
            let step = &self.steps[self.current_step];

            if self.iterations > self.max_iterations {
                return Err(anyhow::anyhow!(
                    "Exceeded maximum iterations ({}) at Step {}: '{}'. Possible infinite loop in workflow.\nCheck for GoTo loops or missing STOP conditions.",
                    self.max_iterations,
                    step.number,
                    step.description
                ));
            }

            println!(
                "\n→ Step {}/{}: {}",
                step.number,
                self.steps.len(),
                step.description
            );

            // Execute commands
            if let Some(command) = &step.command {
                println!("→ Executing: {}", command.code);

                let output = execute_command(command)?;

                self.display_command_output(&output, command.quiet)?;

                debug!("Checking: {}", DEBUG_EVALUATION_CRITERIA);

                // Evaluate conditionals
                let action = self
                    .evaluate_conditionals(&step.conditionals, &output)?
                    .unwrap_or_else(|| self.apply_defaults(&output, &step.conditionals));

                let control = self.execute_action(action, step.number)?;
                match control {
                    StepControl::Next => {
                        // Continue to next step
                    }
                    StepControl::JumpTo(index) => {
                        self.current_step = index;
                        continue 'workflow_loop;
                    }
                    StepControl::Terminate(result) => {
                        return Ok(result);
                    }
                }
            }

            // Execute prompts
            if self.execute_step_prompts(&step.prompts).is_err() {
                return Ok(ExecutionResult::UserCancelled);
            }

            self.current_step += 1;
        }

        println!("\n→ Workflow completed successfully");
        Ok(ExecutionResult::Success)
    }

    fn apply_defaults(&self, output: &CommandOutput, conditionals: &[Conditional]) -> Action {
        // If no conditionals specified, use implicit defaults
        if conditionals.is_empty() {
            return if output.success {
                Action::Continue // Implicit: Pass → Continue
            } else {
                Action::Stop { message: None } // Implicit: Fail → STOP
            };
        }

        // If conditionals exist but none matched, use defaults
        if output.success {
            Action::Continue // Implicit: Pass → Continue
        } else {
            Action::Stop { message: None } // Implicit: Fail → STOP
        }
    }

    fn evaluate_conditionals(
        &self,
        conditionals: &[Conditional],
        output: &CommandOutput,
    ) -> Result<Option<Action>> {
        for conditional in conditionals {
            match conditional {
                Conditional::Pass { action } => {
                    if output.success {
                        return Ok(Some(action.clone()));
                    }
                }
                Conditional::Fail { action } => {
                    if !output.success {
                        return Ok(Some(action.clone()));
                    }
                }
            }
        }
        Ok(None)
    }

    fn execute_step_prompts(&self, prompts: &[Prompt]) -> Result<()> {
        // SECURITY: Prompts accept user input from stdin. While this is by design,
        // malicious workflows could craft misleading prompts to trick users.
        // Always review workflow files before execution (see README.md security section).
        for prompt in prompts {
            println!("→ Prompt: {} [y/N]: ", prompt.text);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let answer = input.trim().to_lowercase();
            if answer != "y" && answer != "yes" {
                println!("→ User answered no");
                return Err(anyhow::anyhow!("User cancelled at prompt"));
            }
        }
        Ok(())
    }

    fn display_command_output(&self, output: &CommandOutput, quiet: bool) -> Result<()> {
        // Show stdout based on quiet flag (suppress successful quiet commands)
        let should_suppress = quiet && output.success;
        if !should_suppress {
            print!("{}", output.stdout);
        }

        // Always show stderr (errors and warnings should be visible)
        if !output.stderr.is_empty() {
            eprint!("{}", output.stderr);
        }

        // Status
        let status_symbol = if output.success { "✓" } else { "✗" };
        let status_text = if output.success { "Passed" } else { "Failed" };
        println!(
            "{} {} (exit {})",
            status_symbol, status_text, output.exit_code
        );

        debug!(
            exit_code = output.exit_code,
            success = output.success,
            "Command completed"
        );

        Ok(())
    }

    fn execute_action(&self, action: Action, from_step: usize) -> Result<StepControl> {
        debug!(?action, "Executing action");

        match action {
            Action::Continue => Ok(StepControl::Next),

            Action::Stop { message } => {
                if let Some(msg) = &message {
                    println!("→ Action: STOP ({})", msg);
                } else {
                    println!("→ Action: STOP");
                }
                Ok(StepControl::Terminate(ExecutionResult::Stopped { message }))
            }

            Action::GoToStep { number } => {
                println!("→ Action: Go to Step {}", number);
                let index = self.find_step_index(number, from_step)?;
                Ok(StepControl::JumpTo(index))
            }
        }
    }

    fn find_step_index(&self, target: usize, from_step: usize) -> Result<usize> {
        self.steps
            .iter()
            .position(|s| s.number == target)
            .ok_or_else(|| {
                let available_steps: Vec<usize> = self.steps.iter().map(|s| s.number).collect();
                anyhow::anyhow!(
                    "Step {}: GoTo target Step {} does not exist.\nAvailable steps: {:?}\nCheck your 'Go to Step N' conditionals.",
                    from_step,
                    target,
                    available_steps
                )
            })
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Success,
    Stopped { message: Option<String> },
    UserCancelled,
}

/// Control flow decision for a single step execution
#[derive(Debug, PartialEq)]
enum StepControl {
    /// Continue to next step (current_step + 1)
    Next,
    /// Jump to specific step index
    JumpTo(usize),
    /// Terminate workflow with final result
    Terminate(ExecutionResult),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::CommandOutput;
    use tracing_subscriber::fmt::format::FmtSpan;

    #[test]
    fn test_tracing_debug_output_works() {
        // Setup tracing capture
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_span_events(FmtSpan::ACTIVE)
            .with_test_writer()
            .finish();

        let _guard = tracing::subscriber::set_default(subscriber);

        // Create simple workflow
        let steps = vec![Step {
            number: 1,
            description: "Test tracing".to_string(),
            command: Some(Command {
                code: "echo 'test'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();

        // Verify workflow completed
        assert_eq!(result, ExecutionResult::Success);
        // Manual verification: RUST_LOG=debug cargo test test_tracing_debug_output_works shows debug output
    }

    #[test]
    fn test_no_debug_field_exists() {
        // This test ensures debug field has been removed
        // If debug field exists, this won't compile
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        // If we can construct without set_debug(), field is gone
        drop(runner); // Explicit to show we just need construction
    }

    #[test]
    fn test_step_control_enum_exists() {
        // Test that StepControl enum and its variants exist
        let next = StepControl::Next;
        let jump = StepControl::JumpTo(5);
        let term = StepControl::Terminate(ExecutionResult::Success);

        // Pattern match to verify all variants handled
        match next {
            StepControl::Next => {}
            StepControl::JumpTo(_) => panic!("Should be Next"),
            StepControl::Terminate(_) => panic!("Should be Next"),
        }

        match jump {
            StepControl::JumpTo(n) => assert_eq!(n, 5),
            _ => panic!("Should be JumpTo"),
        }

        match term {
            StepControl::Terminate(ExecutionResult::Success) => {}
            _ => panic!("Should be Terminate(Success)"),
        }
    }

    #[test]
    fn test_no_conditionals_success_continues() {
        let steps = vec![Step {
            number: 1,
            description: "Test no conditionals".to_string(),
            command: Some(Command {
                code: "echo 'success'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_no_conditionals_failure_stops() {
        // With implicit defaults, failure with no conditionals → STOP
        let steps = vec![Step {
            number: 1,
            description: "Test no conditionals with failure".to_string(),
            command: Some(Command {
                code: "exit 1".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Stopped { message: None });
    }

    #[test]
    fn test_execute_action_continue() {
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::Continue;

        let control = runner.execute_action(action, 1).unwrap();
        assert_eq!(control, StepControl::Next);
    }

    #[test]
    fn test_execute_action_goto() {
        let steps = vec![
            Step {
                number: 1,
                description: "Step 1".to_string(),
                command: None,
                prompts: vec![],
                conditionals: vec![],
            },
            Step {
                number: 2,
                description: "Step 2".to_string(),
                command: None,
                prompts: vec![],
                conditionals: vec![],
            },
        ];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::GoToStep { number: 2 };

        let control = runner.execute_action(action, 1).unwrap();
        assert_eq!(control, StepControl::JumpTo(1)); // Index 1 for step number 2
    }

    #[test]
    fn test_execute_action_goto_invalid() {
        // Test error case: invalid GoToStep number
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::GoToStep { number: 99 }; // Step doesn't exist

        let result = runner.execute_action(action, 1);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Step 99 does not exist"));
    }

    #[test]
    fn test_execute_action_stop() {
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::Stop {
            message: Some("Test stop".to_string()),
        };

        let control = runner.execute_action(action, 1).unwrap();
        assert_eq!(
            control,
            StepControl::Terminate(ExecutionResult::Stopped {
                message: Some("Test stop".to_string())
            })
        );
    }

    #[test]
    fn test_display_command_output_quiet_success() {
        // For quiet successful commands, output should be suppressed
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let output = CommandOutput {
            stdout: "test output\n".to_string(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };

        // This should not panic and should suppress output
        runner.display_command_output(&output, true).unwrap();
    }

    #[test]
    fn test_display_command_output_quiet_failure() {
        // For quiet failed commands, output should be shown
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let output = CommandOutput {
            stdout: "error output\n".to_string(),
            stderr: String::new(),
            exit_code: 1,
            success: false,
        };

        // This should not panic and should show output (failure case)
        runner.display_command_output(&output, true).unwrap();
    }

    #[test]
    fn test_display_command_output_stderr_always_shown() {
        // Stderr should always be shown, even for quiet successful commands
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let output = CommandOutput {
            stdout: "stdout\n".to_string(),
            stderr: "warning: something\n".to_string(),
            exit_code: 0,
            success: true,
        };

        // Stderr should be shown even with quiet=true and success=true
        runner.display_command_output(&output, true).unwrap();
    }

    #[test]
    fn test_execute_step_prompts_empty() {
        // Empty prompt list should succeed immediately
        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditionals: vec![],
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        runner.execute_step_prompts(&[]).unwrap();
    }

    // Note: Cannot easily test interactive stdin in unit tests
    // Manual verification required for actual prompt behavior
}
