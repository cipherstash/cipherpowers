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
    ExitCode { code: i32, action: Action },
    ExitNotZero { action: Action },
    OutputEmpty { action: Action },
    OutputContains { text: String, action: Action },
    Otherwise { action: Action },
}

// TODO: Remove #[allow(dead_code)] once Task 6 (conditional logic) uses this type
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,
    Stop { message: Option<String> },
    GoToStep { number: usize },
}
