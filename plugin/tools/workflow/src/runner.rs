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
