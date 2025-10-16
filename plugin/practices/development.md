---
name: Development Practices
description: Create software that is simple, consistent, and documented to ensure modularity, maintainability, and extensibility.
when_to_use: when writing code to ensure it follows core principles of simplicity, consistency, and clear documentation
applies_to: all projects
related_practices: documentation.md, testing.md
version: 1.0.0
---

# Development Practices

Our goal is to create software that is modular, maintainable, & extensible.

## Core Principles

Simple (not elegant)
Consistent (not stamped with individuality)
Documented (with the why)


### Simple

- Keep it simple (not elegant)
- Do the simplest thing that could possibly work
- Implement requirements when actually needed
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
  - Run linters and formatters for your language
  - Address ALL linter warnings by fixing root cause
  - Use disable/allow directives ONLY when unavoidable


### Documented

- Optimize for readability & understanding
  - Explain the "why" not the "what"
- Avoid complex, convoluted & clever code
  - Simple code is easier to understand
- Use clear, concise, & descriptive variable names
- Use clear & concise comments
  - Provide context over description
  - Avoid unnecessary comments
  - Follow doc comment conventions for your language
  - Use doc tests where appropriate


### File Structure
- Keep files focused on a single responsibility
- Keep code modular
- Group related functionality together
- Use consistent naming conventions


### Code Structure

- Objects and functions should have only one single responsibility
- Use encapsulation to ensure loose coupling
- Minimise dependencies between modules for flexibility

