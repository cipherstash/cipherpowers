#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub number: usize,
    pub description: String,
    pub command: Option<Command>,
    pub prompts: Vec<Prompt>,
    pub conditionals: Vec<Conditional>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub code: String,
    pub quiet: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conditional {
    Pass { action: Action },
    Fail { action: Action },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,
    Stop { message: Option<String> },
    GoToStep { number: usize },
}

// Note: Action uses Clone instead of Copy because Stop contains Option<String>.
// This is acceptable because:
// 1. Actions are cloned infrequently (once per step evaluation)
// 2. The enum is small (3 variants with minimal data)
// 3. Stop messages are typically short strings
// 4. The performance impact is negligible compared to command execution

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_conditional_syntax() {
        // This will fail until we add the new enum variants
        let pass_cond = Conditional::Pass {
            action: Action::Continue,
        };
        let fail_cond = Conditional::Fail {
            action: Action::Stop { message: None },
        };

        assert!(matches!(pass_cond, Conditional::Pass { .. }));
        assert!(matches!(fail_cond, Conditional::Fail { .. }));
    }
}
