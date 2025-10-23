---
name: Logging Best Practices
description: Guidelines for structured logging with appropriate log levels to prevent log pollution and enable effective debugging
when_to_use: when implementing logging in applications to ensure meaningful, actionable log output
applies_to: all projects
version: 1.0.0
---

# Logging Best Practices

## Core principles

**Use appropriate log levels** - `error!()` for unexpected failures only, not debug output
**Use log targets/contexts** - Enable filtering by system/component
**Gate development logs** - Use conditional compilation for temporary debugging

## Log levels

### `error!()` - Unexpected Failures

Use ONLY for conditions that should never happen in correct code:
- Invalid state transitions indicating bugs
- Failed system invariants
- Unrecoverable errors requiring intervention

**Examples:**
```rust
// ✓ Good - unexpected failure
error!("State transition error: {:?}", e);

// ✗ Bad - normal operation logged as error
error!("User not found");  // This is expected behavior
```

```python
# ✓ Good - unexpected failure
logger.error("Database constraint violation: %s", e)

# ✗ Bad - normal operation logged as error
logger.error("Invalid password")  # This is expected behavior
```

### `warn!()` - Recoverable Issues

Use for conditions that are unexpected but recoverable:
- Degraded performance conditions
- Missing optional configuration
- Fallback to default behavior

**Examples:**
```rust
// ✓ Good - degraded state
warn!("Cache miss, falling back to database query");

// ✓ Good - missing optional data
warn!("No API key configured, analytics disabled");
```

### `info!()` - Significant Events

Use for important state changes visible to users:
- Major state transitions (starting → running → completed)
- Significant milestones
- Resource state changes

**Avoid per-request/per-frame noise:**
```rust
// ✗ Bad - creates massive log volume
info!("Processing request for user {:?}", user);  // Happens thousands of times

// ✓ Good - significant milestone
info!("Server started on port {}", port);
```

```python
# ✗ Bad - per-request noise
logger.info(f"API request from {ip}")  # Happens constantly

# ✓ Good - significant event
logger.info(f"Batch processing completed: {count} records")
```

### `debug!()` - Development Debugging

Use for detailed state inspection during development:
- Component/state values
- Calculation intermediate results
- Control flow debugging

**Examples:**
```rust
// ✓ Good - development debugging
debug!("Calculation intermediate: result={:?}", result);

// ✓ Good - state inspection
debug!("Cache state: {} entries, {} MB", count, size_mb);
```

**Gate with conditional compilation if temporary:**
```rust
#[cfg(debug_assertions)]
debug!("DEBUG: State check - {:?}", state);
```

### `trace!()` - Verbose Tracing

Use for extremely detailed tracing (disabled by default):
- Per-request/per-frame events
- Hot loop iterations
- Low-level function entry/exit

**Examples:**
```rust
// ✓ Good - per-frame trace
trace!("Frame tick - entity: {:?}, state: {:?}", entity, state);
```

## Structured logging

**Always use log targets/contexts** to enable filtering by subsystem.

**Rust example (with tracing crate):**
```rust
error!(target: "auth::validation", "Invalid credentials for user {}", username);
warn!(target: "cache::redis", "Connection timeout, retrying");
info!(target: "server::startup", "Listening on {}", addr);
debug!(target: "db::query", "Executing query: {}", sql);
```

**Python example (with logging module):**
```python
# Setup logger with hierarchy
logger = logging.getLogger("myapp.auth.validation")

# Log with context
logger.error("Invalid credentials for user %s", username)
logger.warn("Rate limit exceeded for IP %s", ip)
logger.info("User %s logged in", username)
logger.debug("Query executed in %.2fms", elapsed)
```

**Format:** Use hierarchical naming like `"system::subsystem"` or `"module.component"` for filtering.

## Anti-patterns to avoid

### Log pollution
```rust
// ✗ Bad - per-request logging at info level
for request in requests {
    info!("Processing request {}", request.id);  // Thousands of lines
}

// ✓ Good - batch summary at info level
info!("Processed {} requests", requests.len());
```

### Misusing error level
```rust
// ✗ Bad - expected behavior logged as error
if user.is_none() {
    error!("User not found");  // This is normal, use warn! or info!
}

// ✓ Good - only unexpected failures
if let Err(e) = critical_invariant_check() {
    error!("System invariant violated: {:?}", e);  // This should never happen
}
```

### Missing context
```rust
// ✗ Bad - no context for debugging
error!("Failed");

// ✓ Good - actionable context
error!("Failed to connect to database: host={}, error={:?}", host, err);
```

### Logging sensitive data
```rust
// ✗ Bad - logging passwords/tokens
debug!("User credentials: {:?}", credentials);

// ✓ Good - redact sensitive data
debug!("User authenticated: username={}", username);  // Password omitted
```

## Project configuration

### Configuring log levels

**Development:**
- `error`, `warn`, `info`, `debug` enabled
- `trace` disabled (enable as needed)

**Production:**
- `error`, `warn`, `info` enabled
- `debug`, `trace` disabled

**Example configuration (Rust with env_logger):**
```bash
# Development
RUST_LOG=debug

# Production
RUST_LOG=info

# Filtered (specific subsystem debug)
RUST_LOG=info,myapp::auth=debug
```

**Example configuration (Python):**
```python
import logging

# Development
logging.basicConfig(level=logging.DEBUG)

# Production
logging.basicConfig(level=logging.INFO)

# Filtered
logging.getLogger("myapp.auth").setLevel(logging.DEBUG)
```

## References

**Rust:**
- `tracing` crate - Structured, async-aware logging
- `env_logger` - Simple environment-based configuration
- `log` crate - Logging facade

**Python:**
- `logging` module - Standard library logging
- `structlog` - Structured logging library

**General:**
- [Twelve-Factor App: Logs](https://12factor.net/logs) - Treat logs as event streams
