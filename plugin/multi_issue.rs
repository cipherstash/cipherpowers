// Critical: SQL injection risk
fn query(user_input: String) -> String {
    format!("SELECT * FROM users WHERE id = {}", user_input)
}

// Medium: Magic number
fn calculate() -> i32 { 42 }

// Low: Poor naming
fn x() -> bool { true }
