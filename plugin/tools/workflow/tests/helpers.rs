use workflow::*;

/// Helper function to create a simple test step without conditionals.
/// Used by integration tests to reduce boilerplate.
pub fn simple_step(num: usize, description: &str, cmd: &str) -> models::Step {
    models::Step {
        number: num,
        description: description.to_string(),
        command: Some(models::Command {
            code: cmd.to_string(),
            quiet: false,
        }),
        prompts: vec![],
        conditionals: vec![],
    }
}

/// Helper function to create a test step with a Fail conditional.
/// Used by integration tests to test error handling scenarios.
pub fn step_with_fail_stop(
    num: usize,
    description: &str,
    cmd: &str,
    message: &str,
) -> models::Step {
    models::Step {
        number: num,
        description: description.to_string(),
        command: Some(models::Command {
            code: cmd.to_string(),
            quiet: false,
        }),
        prompts: vec![],
        conditionals: vec![models::Conditional::Fail {
            action: models::Action::Stop {
                message: Some(message.to_string()),
            },
        }],
    }
}
