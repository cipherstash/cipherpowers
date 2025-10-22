use std::num::NonZeroUsize;

/// Step number (1-indexed, never zero)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepNumber(NonZeroUsize);

impl StepNumber {
    pub fn new(n: usize) -> Option<Self> {
        NonZeroUsize::new(n).map(StepNumber)
    }

    pub fn get(&self) -> usize {
        self.0.get()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub number: StepNumber,
    pub description: String,
    pub command: Option<Command>,
    pub prompts: Vec<Prompt>,
    pub conditions: Option<Conditions>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub code: String,
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

/// Atomic conditional unit - both branches always present
#[derive(Debug, Clone, PartialEq)]
pub struct Conditions {
    pub pass: Action,
    pub fail: Action,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,
    Stop(Option<String>),
    Goto(StepNumber),
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

    // StepNumber tests
    #[test]
    fn test_step_number_new_valid() {
        let step_num = StepNumber::new(5).unwrap();
        assert_eq!(step_num.get(), 5);
    }

    #[test]
    fn test_step_number_new_zero_returns_none() {
        let step_num = StepNumber::new(0);
        assert!(step_num.is_none());
    }

    #[test]
    fn test_step_number_ordering() {
        let step1 = StepNumber::new(1).unwrap();
        let step5 = StepNumber::new(5).unwrap();
        assert!(step1 < step5);
    }

    #[test]
    fn test_step_number_equality() {
        let step1a = StepNumber::new(1).unwrap();
        let step1b = StepNumber::new(1).unwrap();
        assert_eq!(step1a, step1b);
    }

    // Step struct tests - verify Step.number uses StepNumber type
    #[test]
    fn test_step_uses_step_number_type() {
        // This test verifies that Step.number uses StepNumber (not raw usize)
        // The type system should prevent creating Step with number = 0
        let step = Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test step".to_string(),
            command: None,
            prompts: vec![],
            conditions: None,
        };

        // Verify .get() is required to access the underlying usize
        assert_eq!(step.number.get(), 1);
    }

    #[test]
    #[should_panic]
    fn test_step_number_prevents_zero_indexed() {
        // This test verifies that StepNumber prevents zero-indexed steps
        // StepNumber::new(0) should return None, so .unwrap() should panic
        let _step_num = StepNumber::new(0).unwrap(); // Should panic
    }

    // Action enum tests
    #[test]
    fn test_action_continue_is_unit_variant() {
        let action = Action::Continue;
        assert!(matches!(action, Action::Continue));
    }

    #[test]
    fn test_action_stop_tuple_variant_with_none() {
        let action = Action::Stop(None);
        assert!(matches!(action, Action::Stop(None)));
    }

    #[test]
    fn test_action_stop_tuple_variant_with_some() {
        let action = Action::Stop(Some("test reason".to_string()));
        match action {
            Action::Stop(Some(msg)) => assert_eq!(msg, "test reason"),
            _ => panic!("Expected Stop with Some message"),
        }
    }

    #[test]
    fn test_action_goto_tuple_variant() {
        let action = Action::Goto(StepNumber::new(5).unwrap());
        match action {
            Action::Goto(step_num) => assert_eq!(step_num.get(), 5),
            _ => panic!("Expected Goto action"),
        }
    }

    // Conditional tests (old syntax, will update later)
    #[test]
    fn test_new_conditional_syntax() {
        let pass_cond = Conditional::Pass {
            action: Action::Continue,
        };
        let fail_cond = Conditional::Fail {
            action: Action::Stop(None),
        };

        assert!(matches!(pass_cond, Conditional::Pass { .. }));
        assert!(matches!(fail_cond, Conditional::Fail { .. }));
    }

    // Conditions (atomic type) tests
    #[test]
    fn test_conditions_atomic_has_both_branches() {
        let conditions = Conditions {
            pass: Action::Continue,
            fail: Action::Stop(Some("fix first".to_string())),
        };

        assert_eq!(conditions.pass, Action::Continue);
        match &conditions.fail {
            Action::Stop(Some(msg)) => assert_eq!(msg, "fix first"),
            _ => panic!("Expected Stop with message"),
        }
    }

    #[test]
    fn test_conditions_equality() {
        let cond1 = Conditions {
            pass: Action::Continue,
            fail: Action::Stop(None),
        };
        let cond2 = Conditions {
            pass: Action::Continue,
            fail: Action::Stop(None),
        };
        assert_eq!(cond1, cond2);
    }

    #[test]
    fn test_conditions_with_goto() {
        let conditions = Conditions {
            pass: Action::Goto(StepNumber::new(5).unwrap()),
            fail: Action::Stop(Some("failed".to_string())),
        };

        match conditions.pass {
            Action::Goto(num) => assert_eq!(num.get(), 5),
            _ => panic!("Expected Goto action"),
        }
    }

    // Step with Option<Conditions> tests
    #[test]
    fn test_step_with_no_conditions_uses_none() {
        // Step with no explicit conditions should have conditions = None
        let step = Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test step".to_string(),
            command: Some(Command {
                code: "echo test".to_string(),
            }),
            prompts: vec![],
            conditions: None,
        };

        assert!(step.conditions.is_none());
    }

    #[test]
    fn test_step_with_explicit_conditions_uses_some() {
        // Step with explicit conditions should have conditions = Some(Conditions)
        let step = Step {
            number: StepNumber::new(1).unwrap(),
            description: "Test step".to_string(),
            command: Some(Command {
                code: "echo test".to_string(),
            }),
            prompts: vec![],
            conditions: Some(Conditions {
                pass: Action::Continue,
                fail: Action::Stop(Some("failed".to_string())),
            }),
        };

        assert!(step.conditions.is_some());
        let conds = step.conditions.unwrap();
        assert_eq!(conds.pass, Action::Continue);
    }
}
