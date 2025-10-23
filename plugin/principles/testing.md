---
name: Testing
description: Create comprehensive unit tests for all business logic, test behavior over implementation, and ensure all tests pass before committing changes.
when_to_use: when writing or modifying code to ensure correctness through automated testing and verification
applies_to: all projects
related_practices: code-review.md, development.md
version: 1.0.0
---

# Testing principles

- **Test behavior, not implementation**: Tests should verify what the code does, not how it does it
- **Test edge cases**: Include tests for boundary conditions, error cases, and unexpected inputs
- **Keep tests isolated**: Each test should be independent and not rely on other tests
- **Make tests readable**: Use clear test names and well-structured arrange-act-assert patterns
- **Test first**: Consider writing tests before implementation (TDD)


## Test-driven development (TDD)

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


## Property-based testing

For mathematical operations, algorithms with invariants, or domain logic with mathematical properties, use property-based tests alongside example-based tests.

**When to use property tests:**
- Operations with mathematical properties (reversibility, commutativity, associativity)
- Domain conversions (units, encodings, transformations)
- Algorithms with invariants that should hold for ANY input
- Numerical stability requirements

**Example pattern:**

```rust
use proptest::prelude::*;
use approx::assert_relative_eq;

// Example-based test: Documents specific known case
#[test]
fn test_currency_conversion_known_case() {
    let usd = Money::usd(100);
    let eur = convert(usd, EUR_RATE);
    assert_eq!(eur.cents(), 85_00); // Known conversion
}

// Property-based test: Verifies property holds for arbitrary inputs
proptest! {
    #[test]
    fn test_conversion_reversibility(amount in 0.01..1e9) {
        let original = Money::usd(amount);
        let converted = convert(original, RATE);
        let back = convert(converted, 1.0 / RATE);
        assert_relative_eq!(back.amount(), original.amount(), epsilon = 1e-6);
    }
}
```

**Key benefits:**
- Finds edge cases you didn't think of (negative values, very large/small numbers, boundary conditions)
- Documents mathematical properties as executable tests
- Acts as regression detector when properties must hold across refactors

**Libraries:**
- **Rust:** `proptest` crate
- **Python:** `hypothesis` library
- **JavaScript:** `fast-check` library
- **Java:** `QuickCheck` or `jqwik`
