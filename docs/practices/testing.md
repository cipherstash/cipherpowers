# Testing

- Create unit tests for all business logic
  - Structure code to enable unit tests
    - Core algorithms and calculations
    - Business rules and validation
    - Data transformations
    - State management
  - Extract business logic into small, testable functions
  - All tests must pass before committing

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
