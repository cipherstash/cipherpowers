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

    #[test]
    fn test_execute_command_not_found() {
        let cmd = Command {
            code: "nonexistent_command_12345_xyz".to_string(),
            quiet: false,
        };
        let output = execute_command(&cmd).unwrap();
        assert!(!output.success);
        assert_ne!(output.exit_code, 0);
        // Shell should report an error message in stderr
        assert!(!output.stderr.is_empty());
    }

    #[test]
    fn test_execute_stderr_captured() {
        let cmd = Command {
            code: "echo 'to stdout'; echo 'to stderr' >&2".to_string(),
            quiet: false,
        };
        let output = execute_command(&cmd).unwrap();
        assert!(output.stdout.contains("to stdout"));
        assert!(output.stderr.contains("to stderr"));
    }

    #[test]
    fn test_execute_exit_code_captured() {
        let cmd = Command {
            code: "exit 42".to_string(),
            quiet: false,
        };
        let output = execute_command(&cmd).unwrap();
        assert_eq!(output.exit_code, 42);
        assert!(!output.success);
    }
}
