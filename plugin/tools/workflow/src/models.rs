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

#[derive(Debug, Clone, PartialEq)]
pub struct Prompt {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Conditional {
    ExitCode { code: i32, action: Action },
    ExitNotZero { action: Action },
    OutputEmpty { action: Action },
    OutputContains { text: String, action: Action },
    Otherwise { action: Action },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,
    Stop { message: Option<String> },
    GoToStep { number: usize },
}
