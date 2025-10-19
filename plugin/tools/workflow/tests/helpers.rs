use workflow::*;

#[allow(dead_code)]
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

#[allow(dead_code)]
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
