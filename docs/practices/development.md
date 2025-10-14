
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


### Async Operations and Concurrency

When working with concurrent operations, follow these patterns to prevent race conditions and ensure data consistency:

**Queue-Based Processing:**
- Use message queues for async operations (processing files, sending notifications)
- Implement single-producer/multiple-consumer pattern
- Design operations to be idempotent - assume retries will happen
- Store operation state in database with unique constraints

**Locking and Synchronization:**
- Use distributed locks (Redis/etcd) for coordinating access to shared resources
- Always acquire locks in consistent order to prevent deadlocks (alphabetically by resource ID)
- Set appropriate lock timeouts (30 seconds minimum for file operations)
- Always release locks using try-finally blocks to handle error paths

**Preventing Race Conditions:**
- Never perform read-modify-write on shared state without holding a lock
- Use atomic operations for counters and state transitions
- Validate state transitions (check current state before applying change)
- Use database transactions for operations affecting multiple resources

**Testing:**
- Write tests that verify concurrent access patterns
- Use chaos testing to inject delays and failures
- Test lock timeout and retry behavior
- Verify idempotency of operations

**Common Pitfalls:**
- Short lock timeouts cause cascading failures under load
- Forgetting to release locks on error paths
- Inconsistent lock ordering leads to deadlocks
- Not making operations idempotent leads to duplicate processing

For implementation details, see CLAUDE.md Concurrency Patterns section.
