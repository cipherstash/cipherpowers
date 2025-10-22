use crate::models::*;
use anyhow::Result;
use std::process::Command as ProcessCommand;

pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

pub fn execute_command(cmd: &Command) -> Result<CommandOutput> {
    // Inherit stdin/stdout/stderr to support interactive commands (git add -p, git commit, etc.)
    // This means all output goes directly to terminal and user can interact with commands
    let status = ProcessCommand::new("sh")
        .arg("-c")
        .arg(&cmd.code)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    Ok(CommandOutput {
        stdout: String::new(),  // Not captured - output goes directly to terminal
        stderr: String::new(),  // Not captured - output goes directly to terminal
        exit_code: status.code().unwrap_or(-1),
        success: status.success(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_simple_command() {
        let cmd = Command {
            code: "echo 'hello'".to_string(),
        };

        let output = execute_command(&cmd).unwrap();
        assert!(output.success);
        assert_eq!(output.exit_code, 0);
        // Output no longer captured - goes directly to terminal
    }

    #[test]
    fn test_execute_failing_command() {
        let cmd = Command {
            code: "exit 1".to_string(),
        };

        let output = execute_command(&cmd).unwrap();
        assert!(!output.success);
        assert_eq!(output.exit_code, 1);
    }

    #[test]
    fn test_execute_command_not_found() {
        let cmd = Command {
            code: "nonexistent_command_12345_xyz".to_string(),
        };
        let output = execute_command(&cmd).unwrap();
        assert!(!output.success);
        assert_ne!(output.exit_code, 0);
        // Error output goes directly to terminal (not captured)
    }

    #[test]
    fn test_execute_with_stderr() {
        let cmd = Command {
            code: "echo 'to stdout'; echo 'to stderr' >&2".to_string(),
        };
        let output = execute_command(&cmd).unwrap();
        assert!(output.success);
        // Both stdout and stderr go directly to terminal (not captured)
    }

    #[test]
    fn test_execute_exit_code_captured() {
        let cmd = Command {
            code: "exit 42".to_string(),
        };
        let output = execute_command(&cmd).unwrap();
        assert_eq!(output.exit_code, 42);
        assert!(!output.success);
    }
}
