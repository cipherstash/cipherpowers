#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutionMode {
    /// Sequential execution, STOP only, no skipping
    Enforcement,
    /// Full control flow (Continue, GoTo, STOP)
    Guided,
}

impl ExecutionMode {
    pub fn allows_continue(&self) -> bool {
        match self {
            ExecutionMode::Enforcement => false,
            ExecutionMode::Guided => true,
        }
    }

    pub fn allows_goto(&self) -> bool {
        match self {
            ExecutionMode::Enforcement => false,
            ExecutionMode::Guided => true,
        }
    }

    pub fn allows_stop(&self) -> bool {
        true // Both modes allow STOP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforcement_mode_restrictions() {
        let mode = ExecutionMode::Enforcement;
        assert!(!mode.allows_continue());
        assert!(!mode.allows_goto());
        assert!(mode.allows_stop());
    }

    #[test]
    fn test_guided_mode_allows_all() {
        let mode = ExecutionMode::Guided;
        assert!(mode.allows_continue());
        assert!(mode.allows_goto());
        assert!(mode.allows_stop());
    }
}
