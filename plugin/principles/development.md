---
name: Development Principles
description: Core development philosophy ensuring software is simple, consistent, documented, tested, debugged systematically, validated thoroughly, and reviewed rigorously.
when_to_use: when writing code to ensure it follows core development principles
applies_to: all projects
version: 1.1.0
---

# Development Principles

Our goal is to create software that is modular, maintainable, & extensible.

## Core principles

Simple (not clever)
Consistent (not stamped with individuality)
Documented (with the why)
Tested (behavior, not implementation)
Debugged (root cause, not symptoms)
Validated (at every layer)
Reviewed (technical correctness over social comfort)


### Simple

- Keep it simple (not clever)
- Do the simplest thing that could possibly work
- Implement requirements when actually needed
- Embrace test-driven development to improve code design
- Avoid over-engineering
- Avoid unnecessary layers of abstraction
- Avoid complex, convoluted & clever code


### Consistent

- Always follow the idioms, practices & guidelines
- Make names consistent
- Don't repeat yourself
  - Extract repeated business logic into reusable modules or functions
  - Create utility functions for common operations
- Don't reinvent the wheel
  - Use third-party libraries, frameworks and dependencies
- YOU MUST check formatting and style
  - Run checks, linters and formatters for your language
  - Address ALL linter warnings by fixing root cause
  - Use disable/allow/ignore directives ONLY when unavoidable


### Documented

- Optimize for readability & understanding
  - Explain the "why" not the "what"
- Avoid complex, convoluted & clever code
  - Simple code is easier to understand
- Use tests to document behaviour and semantics
- Use clear, concise, & descriptive variable names
- Use clear & concise comments
  - Provide context over description
  - Avoid unnecessary comments
  - Follow doc comment conventions for your language
  - Use doc tests where appropriate
- Documentation drifts without systematic maintenance
  - Update docs when changing code (context is freshest)
  - Capture learnings while exhaustion marks significance
  - Discarded approaches are valuable documentation


### Tested

- Test behavior, not implementation
- Test edge cases and boundaries
- Keep tests isolated (independent, no shared state)
- Make tests readable (clear names, arrange-act-assert)
- Test first (TDD) - write test before implementation
- All tests must pass before committing
- Never test mock behavior - test real code
- Never add test-only methods to production classes


### Debugged

- Root cause over symptom fixing
- Investigate before guessing
- No fixes without understanding the problem
- Trace backward through call chain to find origin
- When 3+ fixes fail, question architecture
- Systematic debugging is faster than emergency guessing


### Validated

- Validate at every layer data passes through
- Make invalid states structurally impossible
- Entry → Business → Environment → Debug layers
- Multiple validation layers catch different failures


### Reviewed

- Technical correctness over social comfort
- Verify feedback before implementing
- Ask before assuming reviewer intent
- No performative agreement
- Push back with technical reasoning when warranted

