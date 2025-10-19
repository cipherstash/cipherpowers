// TODO: Remove #[allow(dead_code)] once Task 4 (execution engine) uses this type
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Workflow {
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub number: usize,
    pub description: String,
    pub commands: Vec<Command>,
    pub prompts: Vec<Prompt>,
    pub conditionals: Vec<Conditional>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub code: String,
    pub quiet: bool,
}

// TODO: Remove #[allow(dead_code)] once Task 5 (prompt parsing) uses this type
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    pub text: String,
}

// TODO: Remove #[allow(dead_code)] once Task 6 (conditional logic) uses these types
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Conditional {
    // New simplified syntax
    Pass {
        action: Action,
    },
    Fail {
        action: Action,
    },

    // Legacy syntax (deprecated, will be removed)
    #[deprecated(note = "Use Pass/Fail instead")]
    ExitCode {
        code: i32,
        action: Action,
    },
    #[deprecated(note = "Use Fail instead")]
    ExitNotZero {
        action: Action,
    },
    #[deprecated(note = "Use Pass/Fail with wrapper scripts instead")]
    OutputEmpty {
        action: Action,
    },
    #[deprecated(note = "Use Pass/Fail with wrapper scripts instead")]
    OutputContains {
        text: String,
        action: Action,
    },
    #[deprecated(note = "Use Pass/Fail instead")]
    Otherwise {
        action: Action,
    },
}

// TODO: Remove #[allow(dead_code)] once Task 6 (conditional logic) uses this type
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,
    Stop { message: Option<String> },
    GoToStep { number: usize },
}

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
