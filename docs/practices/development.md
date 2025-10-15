
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
- Always check formatting and style
  - Run linters and formatters for your language
  - Address linter warnings by fixing the root cause of the issue wherever possible
  - Use disable/allow directives sparingly and only when it cannot be easily avoided


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


### Error Handling

Follow consistent error handling patterns throughout the codebase:

**Error Types:**
- Create specific error types for different failure scenarios
- Use ValidationError for input validation failures
- Use AuthenticationError for auth-related failures
- Use ResourceNotFoundError for missing resources

**Error Handling:**
- Handle errors at appropriate boundaries
- Log errors with sufficient context for debugging
- Transform internal errors into user-friendly API responses
- Never expose internal implementation details in error messages

**API Error Format:**
All API errors follow a consistent structure documented in README_API.md


