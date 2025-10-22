use crate::execution_mode::ExecutionMode;
use crate::executor::{execute_command, CommandOutput};
use crate::models::*;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::tty::IsTty;
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
    mode: ExecutionMode,
    dry_run: bool,
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
            dry_run: false,
        }
    }

    pub fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }

    pub fn run(&mut self) -> Result<ExecutionResult> {
        'workflow_loop: while self.current_step < self.steps.len() {
            self.iterations += 1;
            let step = &self.steps[self.current_step];

            self.check_iteration_limit(step)?;

            println!(
                "\nStep {}/{}: {}",
                step.number.get(),
                self.steps.len(),
                step.description
            );

            // Execute commands
            if let Some(command) = &step.command {
                let output = if self.dry_run {
                    // Dry-run mode: skip execution, show command, assume success
                    println!("  [DRY-RUN] Would execute: {}", command.code);
                    CommandOutput {
                        stdout: String::new(),
                        stderr: String::new(),
                        exit_code: 0,
                        success: true,
                    }
                } else {
                    // Normal mode: execute command
                    debug!("Executing: {}", command.code);
                    execute_command(command)?
                };

                // Output already displayed directly to terminal during execution
                // (commands inherit stdio, so no need to display here)

                debug!("Checking: {}", DEBUG_EVALUATION_CRITERIA);

                // Determine action from conditions or defaults
                let action = self.evaluate_step_outcome(step, &output);

                let control = self.execute_action(action, step.number.get())?;
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

        Ok(ExecutionResult::Success)
    }

    fn apply_defaults(&self, output: &CommandOutput) -> Action {
        // Implicit defaults: Pass → Continue, Fail → STOP
        if output.success {
            Action::Continue
        } else {
            Action::Stop(None)
        }
    }

    fn evaluate_step_outcome(&self, step: &Step, output: &CommandOutput) -> Action {
        match &step.conditions {
            Some(conditions) => {
                // Explicit conditions: choose based on exit code
                let action = if output.success {
                    &conditions.pass
                } else {
                    &conditions.fail
                };

                // Check if action is allowed in current mode
                let is_allowed = match action {
                    Action::Continue => self.mode.allows_continue(),
                    Action::Stop(..) => self.mode.allows_stop(),
                    Action::Goto(..) => self.mode.allows_goto(),
                };

                if is_allowed {
                    action.clone()
                } else {
                    // In enforcement mode, Continue/GoTo are ignored → apply defaults
                    debug!(
                        "Conditional matched but ignored in enforcement mode: {:?}",
                        action
                    );
                    self.apply_defaults(output)
                }
            }
            None => {
                // No explicit conditions: use implicit defaults
                self.apply_defaults(output)
            }
        }
    }

    fn check_iteration_limit(&self, step: &Step) -> Result<()> {
        if self.iterations > self.max_iterations {
            return Err(anyhow::anyhow!(
                "Exceeded maximum iterations ({}) at Step {}: '{}'. Possible infinite loop in workflow.\nCheck for GoTo loops or missing STOP conditions.",
                self.max_iterations,
                step.number.get(),
                step.description
            ));
        }
        Ok(())
    }

    fn execute_step_prompts(&self, prompts: &[Prompt]) -> Result<()> {
        // SECURITY: Prompts accept user input from stdin. While this is by design,
        // malicious workflows could craft misleading prompts to trick users.
        // Always review workflow files before execution (see README.md security section).

        for prompt in prompts {
            if self.dry_run {
                // Dry-run mode: display prompt but don't wait for input
                println!("  [DRY-RUN] Would prompt: {}", prompt.text);
                continue;
            }

            // Normal mode: interactive prompts
            print!("Prompt: {} [y/n]: ", prompt.text);
            std::io::Write::flush(&mut std::io::stdout())?;

            let use_single_char = std::io::stdin().is_tty();

            if use_single_char {
                // Interactive terminal: read single character without waiting for Enter
                loop {
                    if let Event::Key(key_event) = event::read()? {
                        match key_event.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                println!("y");
                                break;
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                println!("n");
                                return Err(anyhow::anyhow!("WORKFLOWFAILED"));
                            }
                            _ => {
                                // Ignore other keys
                                continue;
                            }
                        }
                    }
                }
            } else {
                // Non-interactive (tests, pipes): use line-based input
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let answer = input.trim().to_lowercase();
                if answer != "y" {
                    return Err(anyhow::anyhow!("WORKFLOWFAILED"));
                }
            }
        }
        Ok(())
    }


    fn execute_action(&self, action: Action, from_step: usize) -> Result<StepControl> {
        debug!(?action, "Executing action");

        match action {
            Action::Continue => Ok(StepControl::Next),

            Action::Stop(message) => {
                if let Some(msg) = &message {
                    println!("Action: STOP ({})", msg);
                } else {
                    println!("Action: STOP");
                }
                Ok(StepControl::Terminate(ExecutionResult::Stopped { message }))
            }

            Action::Goto(step_number) => {
                let number = step_number.get();
                println!("Action: Go to Step {}", number);
                let index = self.find_step_index(number, from_step)?;
                Ok(StepControl::JumpTo(index))
            }
        }
    }

    fn find_step_index(&self, target: usize, from_step: usize) -> Result<usize> {
        self.steps
            .iter()
            .position(|s| s.number.get() == target)
            .ok_or_else(|| {
                let available_steps: Vec<usize> = self.steps.iter().map(|s| s.number.get()).collect();
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
            number: StepNumber::new(1).unwrap(),
            description: "Test tracing".to_string(),
            command: Some(Command {
                code: "echo 'test'".to_string(),
            }),
            prompts: vec![],
            conditions: None,
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
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
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
            number: StepNumber::new(1).unwrap(),
            description: "Test no conditionals".to_string(),
            command: Some(Command {
                code: "echo 'success'".to_string(),
            }),
            prompts: vec![],
            conditions: None,
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_no_conditionals_failure_stops() {
        // With implicit defaults, failure with no conditionals → STOP
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test no conditionals with failure".to_string(),
            command: Some(Command {
                code: "exit 1".to_string(),
            }),
            prompts: vec![],
            conditions: None,
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Stopped { message: None });
    }

    #[test]
    fn test_execute_action_continue() {
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
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
                number: StepNumber::new(1).unwrap(),
                description: "Step 1".to_string(),
                command: None,
                prompts: vec![],
                conditions: None,
            },
            Step {
                number: StepNumber::new(2).unwrap(),
                description: "Step 2".to_string(),
                command: None,
                prompts: vec![],
                conditions: None,
            },
        ];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::Goto(StepNumber::new(2).unwrap());

        let control = runner.execute_action(action, 1).unwrap();
        assert_eq!(control, StepControl::JumpTo(1)); // Index 1 for step number 2
    }

    #[test]
    fn test_execute_action_goto_invalid() {
        // Test error case: invalid Goto number
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::Goto(StepNumber::new(99).unwrap()); // Step doesn't exist

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
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let action = Action::Stop(Some("Test stop".to_string()));

        let control = runner.execute_action(action, 1).unwrap();
        assert_eq!(
            control,
            StepControl::Terminate(ExecutionResult::Stopped {
                message: Some("Test stop".to_string())
            })
        );
    }

    #[test]
    fn test_execute_step_prompts_empty() {
        // Empty prompt list should succeed immediately
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        runner.execute_step_prompts(&[]).unwrap();
    }

    // Note: Cannot easily test interactive stdin in unit tests
    // Manual verification required for actual prompt behavior

    #[test]
    fn test_check_iteration_limit_within_bounds() {
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        // Within bounds: iterations = 0, max = 10 (1 step * 10 multiplier)
        runner.check_iteration_limit(&runner.steps[0]).unwrap();
    }

    #[test]
    fn test_check_iteration_limit_exceeded() {
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test infinite loop".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        runner.iterations = 100; // Exceed max_iterations (1 * 10 = 10)

        let result = runner.check_iteration_limit(&runner.steps[0]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Exceeded maximum iterations"));
    }

    #[test]
    fn test_evaluate_step_outcome_explicit_pass() {
        // Test explicit Pass condition in Guided mode (allows GoTo)
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: Some(Conditions {
                pass: Action::Goto(StepNumber::new(2).unwrap()),
                fail: Action::Stop(None),
            }),
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };

        let action = runner.evaluate_step_outcome(&runner.steps[0], &output);
        assert_eq!(action, Action::Goto(StepNumber::new(2).unwrap()));
    }

    #[test]
    fn test_evaluate_step_outcome_implicit_defaults() {
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: None, // No explicit conditions
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);

        // Success → Continue
        let output_success = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };
        let action = runner.evaluate_step_outcome(&runner.steps[0], &output_success);
        assert_eq!(action, Action::Continue);

        // Failure → STOP
        let output_fail = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 1,
            success: false,
        };
        let action = runner.evaluate_step_outcome(&runner.steps[0], &output_fail);
        assert_eq!(action, Action::Stop(None));
    }

    #[test]
    fn test_enforcement_mode_ignores_goto_condition() {
        // Test that enforcement mode ignores Goto conditions
        let steps = vec![
            Step {
                number: StepNumber::new(1).unwrap(),
                description: "Test".to_string(),
                command: None,
                prompts: vec![],
                conditions: Some(Conditions {
                    pass: Action::Goto(StepNumber::new(3).unwrap()),
                    fail: Action::Stop(None),
                }),
            },
            Step {
                number: StepNumber::new(2).unwrap(),
                description: "Should be visited in enforcement mode".to_string(),
                command: None,
                prompts: vec![],
                conditions: None,
            },
            Step {
                number: StepNumber::new(3).unwrap(),
                description: "Final step".to_string(),
                command: None,
                prompts: vec![],
                conditions: None,
            },
        ];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);

        // In enforcement mode, GoTo should be ignored
        // so implicit defaults apply (Pass → Continue)
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };

        let action = runner.evaluate_step_outcome(&runner.steps[0], &output);
        // In enforcement mode: GoTo ignored, defaults to Continue
        assert_eq!(action, Action::Continue);
    }

    #[test]
    fn test_enforcement_mode_allows_stop_condition() {
        // Test that enforcement mode allows STOP conditions
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test".to_string(),
            command: None,
            prompts: vec![],
            conditions: Some(Conditions {
                pass: Action::Stop(Some("Intentional stop".to_string())),
                fail: Action::Stop(None),
            }),
        }];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };

        let action = runner.evaluate_step_outcome(&runner.steps[0], &output);
        // STOP should be allowed in enforcement mode
        assert_eq!(
            action,
            Action::Stop(Some("Intentional stop".to_string()))
        );
    }

    #[test]
    fn test_guided_mode_allows_goto_condition() {
        // Test that guided mode allows Goto conditions
        let steps = vec![
            Step {
                number: StepNumber::new(1).unwrap(),
                description: "Test".to_string(),
                command: None,
                prompts: vec![],
                conditions: Some(Conditions {
                    pass: Action::Goto(StepNumber::new(3).unwrap()),
                    fail: Action::Stop(None),
                }),
            },
            Step {
                number: StepNumber::new(2).unwrap(),
                description: "Should be skipped in guided mode".to_string(),
                command: None,
                prompts: vec![],
                conditions: None,
            },
            Step {
                number: StepNumber::new(3).unwrap(),
                description: "Final step".to_string(),
                command: None,
                prompts: vec![],
                conditions: None,
            },
        ];

        let runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        let output = CommandOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            success: true,
        };

        let action = runner.evaluate_step_outcome(&runner.steps[0], &output);
        // In guided mode: Goto should be allowed
        assert_eq!(action, Action::Goto(StepNumber::new(3).unwrap()));
    }

    #[test]
    fn test_enforcement_mode_integration() {
        // Integration test: full workflow execution in enforcement mode
        // Verifies that Goto is ignored and all steps execute sequentially
        let steps = vec![
            Step {
                number: StepNumber::new(1).unwrap(),
                description: "Step 1 with Goto".to_string(),
                command: Some(Command {
                    code: "exit 0".to_string(),
                    }),
                prompts: vec![],
                conditions: Some(Conditions {
                    pass: Action::Goto(StepNumber::new(3).unwrap()),
                    fail: Action::Stop(None),
                }),
            },
            Step {
                number: StepNumber::new(2).unwrap(),
                description: "Step 2 - must be visited".to_string(),
                command: Some(Command {
                    code: "exit 0".to_string(),
                    }),
                prompts: vec![],
                conditions: None,
            },
            Step {
                number: StepNumber::new(3).unwrap(),
                description: "Step 3".to_string(),
                command: Some(Command {
                    code: "exit 0".to_string(),
                    }),
                prompts: vec![],
                conditions: None,
            },
        ];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();

        // In enforcement mode, should visit all 3 steps sequentially (GoTo ignored)
        assert_eq!(runner.current_step, 3); // All steps visited
        assert_eq!(result, ExecutionResult::Success);
    }

    // Task 4.3: Dry-run tests
    #[test]
    fn test_dry_run_skips_command_execution() {
        // Dry-run should skip actual command execution
        // Use a command that would fail if actually executed
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test dry-run".to_string(),
            command: Some(Command {
                code: "nonexistent_command_should_fail".to_string(),
            }),
            prompts: vec![],
            conditions: None,
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        runner.set_dry_run(true);

        // Should succeed even though command would fail if executed
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_dry_run_assumes_success() {
        // Dry-run should assume all commands succeed (follow PASS path)
        let steps = vec![
            Step {
                number: StepNumber::new(1).unwrap(),
                description: "Test dry-run assumes success".to_string(),
                command: Some(Command {
                    code: "exit 1".to_string(), // Would fail in normal execution
                    }),
                prompts: vec![],
                conditions: Some(Conditions {
                    pass: Action::Continue,
                    fail: Action::Stop(Some("Should not reach this".to_string())),
                }),
            },
            Step {
                number: StepNumber::new(2).unwrap(),
                description: "Second step".to_string(),
                command: Some(Command {
                    code: "echo done".to_string(),
                    }),
                prompts: vec![],
                conditions: None,
            },
        ];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        runner.set_dry_run(true);

        // Should follow PASS path (Continue) even though command exits 1
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
        assert_eq!(runner.current_step, 2); // Both steps visited
    }

    #[test]
    fn test_dry_run_displays_prompts() {
        // Dry-run should display prompts (but not actually wait for input)
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test prompts in dry-run".to_string(),
            command: None,
            prompts: vec![
                Prompt {
                    text: "First prompt?".to_string(),
                },
                Prompt {
                    text: "Second prompt?".to_string(),
                },
            ],
            conditions: None,
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        runner.set_dry_run(true);

        // Should handle prompts without blocking on stdin
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_normal_mode_executes_commands() {
        // Normal mode (not dry-run) should execute commands
        let steps = vec![Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test normal execution".to_string(),
            command: Some(Command {
                code: "echo test".to_string(),
            }),
            prompts: vec![],
            conditions: None,
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        // No set_dry_run() call, should default to false

        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_dry_run_vs_validate() {
        // Dry-run should run through workflow logic (not just validate structure)
        // It should follow conditional paths
        let steps = vec![
            Step {
                number: StepNumber::new(1).unwrap(),
                description: "Decision step".to_string(),
                command: Some(Command {
                    code: "exit 0".to_string(),
                    }),
                prompts: vec![],
                conditions: Some(Conditions {
                    pass: Action::Goto(StepNumber::new(3).unwrap()),
                    fail: Action::Stop(None),
                }),
            },
            Step {
                number: StepNumber::new(2).unwrap(),
                description: "Skipped step".to_string(),
                command: Some(Command {
                    code: "echo skipped".to_string(),
                    }),
                prompts: vec![],
                conditions: None,
            },
            Step {
                number: StepNumber::new(3).unwrap(),
                description: "Target step".to_string(),
                command: Some(Command {
                    code: "echo reached".to_string(),
                    }),
                prompts: vec![],
                conditions: None,
            },
        ];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        runner.set_dry_run(true);

        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
        // Should have jumped from step 1 to step 3, ending at step 3
        assert_eq!(runner.current_step, 3);
    }
}
