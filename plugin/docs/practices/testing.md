# Testing

- YOU MUST create unit tests for all business logic
  - Structure code to enable unit tests
    - Core algorithms and calculations
    - Business rules and validation
    - Data transformations
    - State management
  - Extract business logic into small, testable functions
  - ALL tests MUST pass before committing

- Manual testing for complex integration scenarios
  - UI/UX flows
  - End-to-end workflows
  - Visual elements
  - Performance under load
  - Always ask the user to test, validate and verify when manual testing is needed

## Testing Principles

- **Test behavior, not implementation**: Tests should verify what the code does, not how it does it
- **Test edge cases**: Include tests for boundary conditions, error cases, and unexpected inputs
- **Keep tests isolated**: Each test should be independent and not rely on other tests
- **Make tests readable**: Use clear test names and well-structured arrange-act-assert patterns
- **Test first when possible**: Consider writing tests before implementation (TDD)

## Project Configuration

### Commands

**Run tests:**
- Command: `mise run test`
- What it runs: ALL test suites with project-specific configuration
- Requirement: ALL tests MUST pass before committing
- Never use language-specific commands directly (e.g., `cargo test`) - they miss project configuration

**Run checks:**
- Command: `mise run check`
- What it runs: Linters, formatters, type checkers with project configuration
- Requirement: ALL checks MUST pass before code review
- Includes: formatting, linting, clippy (Rust), type checking
- Never use language-specific commands directly (e.g., `cargo clippy`) - check task may include more

### Test Organization

**Test location:**
- Follow language conventions (e.g., Rust: `tests/` or inline, Python: `tests/`)
- Colocate tests with code when appropriate

**Test naming:**
- Descriptive test names explaining what is being tested
- Pattern: `test_<functionality>_<scenario>_<expected_outcome>`
