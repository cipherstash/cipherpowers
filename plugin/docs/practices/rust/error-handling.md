
### Error Handling

Follow consistent error handling patterns throughout the codebase:

Use the `thiserror` crate.

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



