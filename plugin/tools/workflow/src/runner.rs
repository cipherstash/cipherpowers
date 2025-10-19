use crate::execution_mode::ExecutionMode;
use crate::executor::{execute_command, CommandOutput};
use crate::models::*;
use anyhow::Result;

/// Maximum iterations multiplier per step.
///
/// This determines how many times we can loop through steps before detecting an infinite loop.
/// For a workflow with N steps, the maximum total iterations is N * MAX_ITERATION_MULTIPLIER.
///
/// The value of 10 allows for reasonable looping patterns (e.g., retry logic, conditional jumps)
/// while catching truly infinite loops. For example, a 5-step workflow can iterate up to 50 times.
const MAX_ITERATION_MULTIPLIER: usize = 10;

pub struct WorkflowRunner {
    steps: Vec<Step>,
    current_step: usize,
    iterations: usize,
    max_iterations: usize,
    mode: ExecutionMode,
    debug: bool,
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
            debug: false,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
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

                // Show stdout based on quiet flag (suppress successful quiet commands)
                let should_suppress = command.quiet && output.success;
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

                // Debug output
                if self.debug {
                    println!("→ [DEBUG] Checking: exit code (0 = Pass, non-zero = Fail)");
                    let result_text = if output.success {
                        format!("Pass (exit {})", output.exit_code)
                    } else {
                        format!("Fail (exit {})", output.exit_code)
                    };
                    println!("→ [DEBUG] Result: {}", result_text);
                }

                // Evaluate conditionals
                let action = self.evaluate_conditionals(&step.conditionals, &output)?
                    .unwrap_or_else(|| self.apply_defaults(&output, &step.conditionals));

                if self.debug {
                    match &action {
                        Action::Continue => println!("→ [DEBUG] Action: Continue"),
                        Action::Stop { message } => {
                            if let Some(msg) = message {
                                println!("→ [DEBUG] Action: STOP ({})", msg);
                            } else {
                                println!("→ [DEBUG] Action: STOP");
                            }
                        }
                        Action::GoToStep { number } => println!("→ [DEBUG] Action: Go to Step {}", number),
                    }
                }

                match action {
                    Action::Continue => {
                        // Only show if debug mode
                        // Silent continue in normal mode for clean UX
                    }
                    Action::Stop { message } => {
                        if let Some(msg) = message {
                            println!("→ Action: STOP ({})", msg);
                            return Ok(ExecutionResult::Stopped { message: Some(msg) });
                        } else {
                            println!("→ Action: STOP");
                            return Ok(ExecutionResult::Stopped { message: None });
                        }
                    }
                    Action::GoToStep { number } => {
                        println!("→ Action: Go to Step {}", number);
                        self.current_step = self.find_step_index(number, step.number)?;
                        continue 'workflow_loop;
                    }
                }
            }

            // Execute prompts
            // SECURITY: Prompts accept user input from stdin. While this is by design,
            // malicious workflows could craft misleading prompts to trick users.
            // Always review workflow files before execution (see README.md security section).
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

    fn apply_defaults(&self, output: &CommandOutput, conditionals: &[Conditional]) -> Action {
        // If no conditionals specified, use implicit defaults
        if conditionals.is_empty() {
            return if output.success {
                Action::Continue  // Implicit: Pass → Continue
            } else {
                Action::Stop { message: None }  // Implicit: Fail → STOP
            };
        }

        // If conditionals exist but none matched, use defaults
        if output.success {
            Action::Continue  // Implicit: Pass → Continue
        } else {
            Action::Stop { message: None }  // Implicit: Fail → STOP
        }
    }

    fn evaluate_conditionals(
        &self,
        conditionals: &[Conditional],
        output: &CommandOutput,
    ) -> Result<Option<Action>> {
        for conditional in conditionals {
            let matched_action = match conditional {
                // New syntax (will be implemented properly in Task 5)
                Conditional::Pass { action } => {
                    if output.success {
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                Conditional::Fail { action } => {
                    if !output.success {
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                // Legacy syntax (deprecated)
                #[allow(deprecated)]
                Conditional::ExitCode { code, action } => {
                    if output.exit_code == *code {
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                #[allow(deprecated)]
                Conditional::ExitNotZero { action } => {
                    if output.exit_code != 0 {
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                #[allow(deprecated)]
                Conditional::OutputEmpty { action } => {
                    if output.stdout.trim().is_empty() {
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                #[allow(deprecated)]
                Conditional::OutputContains { text, action } => {
                    if output.stdout.contains(text) || output.stderr.contains(text) {
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                #[allow(deprecated)]
                Conditional::Otherwise { action } => Some(action.clone()),
            };

            // If we matched a conditional, check if it's allowed in current mode
            if let Some(action) = matched_action {
                let is_allowed = match &action {
                    Action::Continue => self.mode.allows_continue(),
                    Action::Stop { .. } => self.mode.allows_stop(),
                    Action::GoToStep { .. } => self.mode.allows_goto(),
                };

                if is_allowed {
                    return Ok(Some(action));
                } else {
                    // In enforcement mode, Continue/GoTo are ignored - continue to next conditional
                    println!(
                        "→ Conditional matched but ignored in enforcement mode: {:?}",
                        action
                    );
                    continue;
                }
            }
        }
        Ok(None)
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

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple_workflow() {
        let steps = vec![Step {
            number: 1,
            description: "Echo test".to_string(),
            command: Some(Command {
                code: "echo 'step 1'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::ExitCode {
                code: 0,
                action: Action::Continue,
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_conditional_output_contains() {
        let steps = vec![Step {
            number: 1,
            description: "Test output matching".to_string(),
            command: Some(Command {
                code: "echo 'ERROR: Something failed'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::OutputContains {
                text: "ERROR".to_string(),
                action: Action::Stop {
                    message: Some("Found error in output".to_string()),
                },
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(
            result,
            ExecutionResult::Stopped {
                message: Some("Found error in output".to_string())
            }
        );
    }

    #[test]
    fn test_conditional_output_empty() {
        let steps = vec![Step {
            number: 1,
            description: "Test empty output".to_string(),
            command: Some(Command {
                code: "echo -n ''".to_string(), // Produces empty output
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::OutputEmpty {
                action: Action::Continue,
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_conditional_otherwise() {
        let steps = vec![Step {
            number: 1,
            description: "Test otherwise fallback".to_string(),
            command: Some(Command {
                code: "exit 42".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![
                Conditional::ExitCode {
                    code: 0,
                    action: Action::Continue,
                },
                Conditional::Otherwise {
                    action: Action::Stop {
                        message: Some("Unexpected exit code".to_string()),
                    },
                },
            ],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(
            result,
            ExecutionResult::Stopped {
                message: Some("Unexpected exit code".to_string())
            }
        );
    }

    #[test]
    fn test_action_goto_step() {
        use crate::execution_mode::ExecutionMode;

        let steps = vec![
            Step {
                number: 1,
                description: "First step".to_string(),
                command: Some(Command {
                    code: "echo 'step 1'".to_string(),
                    quiet: false,
                }),
                prompts: vec![],
                conditionals: vec![Conditional::ExitCode {
                    code: 0,
                    action: Action::GoToStep { number: 3 },
                }],
            },
            Step {
                number: 2,
                description: "Skipped step".to_string(),
                command: None,
                prompts: vec![],
                conditionals: vec![],
            },
            Step {
                number: 3,
                description: "Final step".to_string(),
                command: None,
                prompts: vec![],
                conditionals: vec![],
            },
        ];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_action_stop_with_message() {
        let steps = vec![Step {
            number: 1,
            description: "Test stop with message".to_string(),
            command: Some(Command {
                code: "exit 1".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::ExitNotZero {
                action: Action::Stop {
                    message: Some("Command failed as expected".to_string()),
                },
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(
            result,
            ExecutionResult::Stopped {
                message: Some("Command failed as expected".to_string())
            }
        );
    }

    #[test]
    fn test_invalid_goto_step() {
        use crate::execution_mode::ExecutionMode;

        let steps = vec![Step {
            number: 1,
            description: "Test invalid goto".to_string(),
            command: Some(Command {
                code: "echo 'test'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::ExitCode {
                code: 0,
                action: Action::GoToStep { number: 99 },
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        let result = runner.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Step 99 does not exist"));
    }

    #[test]
    fn test_multiple_conditionals_first_match_wins() {
        let steps = vec![Step {
            number: 1,
            description: "Test conditional order".to_string(),
            command: Some(Command {
                code: "echo 'ERROR: test'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![
                Conditional::OutputContains {
                    text: "ERROR".to_string(),
                    action: Action::Stop {
                        message: Some("Found ERROR".to_string()),
                    },
                },
                Conditional::ExitCode {
                    code: 0,
                    action: Action::Continue,
                },
            ],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        // First conditional should match
        assert_eq!(
            result,
            ExecutionResult::Stopped {
                message: Some("Found ERROR".to_string())
            }
        );
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
    fn test_infinite_loop_protection() {
        use crate::execution_mode::ExecutionMode;

        let steps = vec![Step {
            number: 1,
            description: "Infinite loop test".to_string(),
            command: Some(Command {
                code: "echo 'loop'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::ExitCode {
                code: 0,
                action: Action::GoToStep { number: 1 },
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        let result = runner.run();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Exceeded maximum iterations"));
    }

    #[test]
    fn test_runner_with_enforcement_mode() {
        use crate::execution_mode::ExecutionMode;

        let steps = vec![Step {
            number: 1,
            description: "Test".to_string(),
            command: Some(Command {
                code: "echo 'test'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::ExitCode {
                code: 0,
                action: Action::Continue, // Should be ignored in enforcement mode
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_enforcement_mode_ignores_continue_and_goto() {
        use crate::execution_mode::ExecutionMode;

        let steps = vec![
            Step {
                number: 1,
                description: "Step with Continue".to_string(),
                command: Some(Command {
                    code: "echo 'test'".to_string(),
                    quiet: false,
                }),
                prompts: vec![],
                conditionals: vec![
                    Conditional::ExitCode {
                        code: 0,
                        action: Action::Continue, // Should be ignored
                    },
                    Conditional::ExitCode {
                        code: 1,
                        action: Action::Stop {
                            message: Some("failed".to_string()),
                        },
                    },
                ],
            },
            Step {
                number: 2,
                description: "Second step".to_string(),
                command: None,
                prompts: vec![],
                conditionals: vec![],
            },
        ];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Enforcement);
        let result = runner.run().unwrap();

        // Should complete both steps (Continue was ignored, didn't stop early)
        assert_eq!(result, ExecutionResult::Success);
    }

    #[test]
    fn test_guided_mode_respects_continue() {
        use crate::execution_mode::ExecutionMode;

        let steps = vec![Step {
            number: 1,
            description: "Step with Continue".to_string(),
            command: Some(Command {
                code: "echo 'test'".to_string(),
                quiet: false,
            }),
            prompts: vec![],
            conditionals: vec![Conditional::ExitCode {
                code: 0,
                action: Action::Continue,
            }],
        }];

        let mut runner = WorkflowRunner::new(steps, ExecutionMode::Guided);
        let result = runner.run().unwrap();
        assert_eq!(result, ExecutionResult::Success);
    }
}
