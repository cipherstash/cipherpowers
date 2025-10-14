# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CipherPowers

CipherPowers is a Claude Code plugin providing a comprehensive toolkit for development teams.

## Architecture

CipherPowers combines three layers:

### 1. Skills Layer (`skills/`)

Reusable process knowledge documented using the superpowers framework. Skills are testable, discoverable guides for techniques and workflows.

**Key principles:**
- Written following TDD: test with subagents before writing
- Include rich `when_to_use` frontmatter for discovery
- Follow superpowers SKILL.md structure
- Can reference upstream superpowers skills

**Scope:**
- Organization-specific workflows and practices
- Universal skills under development (before upstreaming)
- Extensions to superpowers skills for team context

### 2. Automation Layer (`commands/`, `agents/`)

Commands and agents that dispatch to skills or provide project-specific workflows.

**Commands:** Slash commands users type (`/plan`, `/work`)
- Provide context and instructions
- Dispatch to agents for complex work
- Reference skills for process guidance

**Agents:** Specialized subagent prompts
- Handle specific roles (work-planner, code-reviewer)
- Follow skills for methodology
- Receive context from commands

### 3. Documentation Layer (`docs/`)

Standards, guidelines, and reference materials.

**Practices:** Coding standards, conventions, guidelines
**Examples:** Real-world examples and templates
**Purpose:** Support skills and provide team reference

## Organizational Benefits

This three-layer separation achieves key software engineering principles:

✅ **DRY (Don't Repeat Yourself)**
- Standards live in one place (`docs/practices/`)
- Skills reference practices instead of duplicating them
- Commands reference skills instead of reimplementing workflows
- Changes propagate automatically through references

✅ **SRP (Single Responsibility Principle)**
- **Practices** define standards (WHAT to follow)
- **Skills** define workflows (HOW to do it)
- **Commands** dispatch to skills (WHEN to invoke)
- Each component has exactly one reason to change

✅ **Reusability**
- Skills are universal workflows (portable, can be upstreamed to superpowers)
- Practices are project-specific standards (customized for your team)
- Commands add project context to universal workflows
- Mix local and upstream skills seamlessly

✅ **Testability**
- Skills include TDD test scenarios using subagents
- Baseline tests prove problems exist without the skill
- With-skill tests verify effectiveness under pressure
- Test scenarios document expected violations and how skill prevents them
- See `skills/*/test-scenarios.md` for examples

✅ **Maintainability**
- Update standards once in practices, all skills benefit
- Change skill workflow without touching commands
- Add new commands without modifying skills
- Clear boundaries prevent coupling and drift

**Example: Documentation Structure**
- `docs/practices/documentation.md` = Standards (formatting, completeness, structure)
- `skills/documentation/maintaining-docs-after-changes/` = Workflow (two-phase sync process)
- `skills/documentation/capturing-learning/` = Workflow (retrospective capture process)
- `commands/doc-review.md` = Dispatcher (triggers maintenance workflow with project context)
- `commands/summarise.md` = Dispatcher (triggers learning capture with work tracking integration)

All five components work together without duplication. Change documentation standards once, both skills and commands use the updated version automatically.

## Integration with Superpowers

**Custom find-skills tool** (`tools/find-skills`):
- Searches `${CIPHERPOWERS_ROOT}/skills/` (org-specific)
- Searches `${SUPERPOWERS_SKILLS_ROOT}` (universal skills)
- Provides unified discovery across both collections
- Flags: `--local`, `--upstream`, or default (both)

**Skill references:**
Commands and agents reference skills from either collection transparently using standard paths.

## Using the find-skills Tool

The custom `tools/find-skills` script provides unified discovery:

```bash
# From repository root
./tools/find-skills "search pattern"

# With scope flags
./tools/find-skills --local "pattern"      # cipherpowers only
./tools/find-skills --upstream "pattern"   # superpowers only
```

## Working with Skills in this Repository

When creating or editing skills in `skills/`:

1. **Read the meta-skill:** `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md`
2. **Follow TDD:** Test with subagents BEFORE writing
3. **Use TodoWrite:** Create todos for the skill creation checklist
4. **Consider upstream:** Universal skills may be contributed to superpowers later

## Authentication Patterns

CipherPowers implements OAuth 2.0 authentication. Follow these patterns when working with authentication code.

### Implementation Lessons

**Token Management:**
- Always implement token refresh with 5-minute buffer before expiry
- Store tokens securely using environment-specific key management
- Never log full tokens (use last 4 characters only for debugging)
- Cache tokens in memory with automatic invalidation on expiry

**Error Handling:**
- Implement exponential backoff for token endpoint requests
- Handle "invalid_grant" by clearing cached tokens and re-authenticating
- Distinguish between client errors (4xx) and server errors (5xx)
- Log authentication failures with context (but never credentials)

**Security Best Practices:**
- Use PKCE for all OAuth flows, even server-side (defense in depth)
- Store code verifier in secure session storage, never localStorage
- Validate state parameter to prevent CSRF attacks
- Use separate OAuth clients for development/staging/production

**Common Pitfalls Discovered:**
1. **Redirect URI encoding**: URL-encode redirect URIs when registering with OAuth provider, but use exact unencoded match in code
2. **Token refresh race conditions**: Implement mutex/lock when refreshing tokens to prevent concurrent refresh attempts
3. **Scope creep**: Request minimal scopes needed; add more later if needed
4. **Time synchronization**: Token expiry relies on server time; sync clocks in containerized environments

**Testing Authentication:**
- Mock OAuth provider responses in unit tests
- Use test OAuth client credentials in CI/CD
- Test token expiry and refresh flows explicitly
- Verify PKCE code challenge generation and verification

**File Locations:**
- Token management: `src/auth/token-manager.js`
- OAuth flows: `src/auth/flows/`
- Token storage: `src/auth/storage/secure-storage.js`
- Retry logic: `src/auth/retry.js`

### Development Workflow

When modifying authentication:
1. Update relevant flow handler in `src/auth/flows/`
2. Update token management if expiry/refresh logic changes
3. Update integration tests in `tests/integration/auth/`
4. Update docs/practices/authentication.md with pattern changes
5. Update README.md troubleshooting if new issues discovered

## Error Handling Patterns

CipherPowers implements standardized error handling across all components. Follow these patterns when working with errors.

### Implementation Lessons

**Error Type Design:**
- Create specific error types for different failure modes (ValidationError, AuthenticationError, ResourceNotFoundError)
- Include contextual information: user-facing message, error code, debugging details
- Implement common Error interface/base class for consistency
- Use error codes for programmatic handling, messages for user display

**Error Handler Implementation:**
- Centralize error handling at API boundary layer
- Log errors with full context before transforming for client response
- Never expose internal implementation details in API error messages
- Include request ID in error responses for debugging correlation

**API Error Response Format:**
```json
{
  "error": {
    "type": "ValidationError",
    "message": "Request validation failed",
    "code": "VALIDATION_FAILED",
    "details": {
      "field": "email",
      "reason": "Invalid email format"
    },
    "request_id": "req_abc123"
  }
}
```

**Common Pitfalls Discovered:**
1. **Over-specific error types**: Start with broad categories (Validation, Authentication, NotFound, ServerError) before creating sub-types
2. **Logging sensitive data**: Never log passwords, tokens, or PII in error details
3. **Inconsistent error codes**: Use UPPER_SNAKE_CASE convention, maintain error code registry
4. **Missing request context**: Always include request ID for tracing errors in distributed systems

**Testing Error Paths:**
- Test each error type explicitly (unit tests)
- Verify error response format matches schema (integration tests)
- Test error handling under failure conditions (network errors, timeouts)
- Ensure error logging doesn't throw secondary errors

**File Locations:**
- Error types: `src/errors/types.ts`
- Error handlers: `src/middleware/error-handler.ts`
- API error responses: `src/api/error-formatter.ts`

### Development Workflow

When modifying error handling:
1. Update error types in `src/errors/types.ts` if adding new error categories
2. Update error handler middleware if changing response format
3. Update integration tests for new error scenarios
4. Update docs/practices/development.md with any pattern changes
5. Update README.md troubleshooting section with common error solutions

## Concurrency and Race Condition Patterns

CipherPowers implements concurrent file processing pipelines. Follow these patterns when working with async operations and shared state.

### Implementation Lessons

**Queue Management:**
- Use single-producer/multiple-consumer pattern for file processing queues
- Implement per-resource locks rather than global locks to maximize throughput
- Always acquire locks in consistent order (alphabetically by resource ID) to prevent deadlocks
- Set lock timeout to 30 seconds with exponential backoff retry (3 attempts max)

**Race Condition Prevention:**
- Never read-modify-write shared state without holding appropriate lock
- Use atomic operations for counter increments (queue depth, processed count)
- Implement idempotent operations - assume any operation may be retried
- Store operation state in database with unique constraint on operation ID

**Lock Mechanism Design:**
- Use distributed locks (Redis/etcd) for multi-node deployments
- Implement lock renewal for long-running operations (renew every 10 seconds)
- Always use try-finally to ensure lock release even on errors
- Log lock acquisition and release with operation context for debugging

**Common Pitfalls Discovered:**
1. **Lock timeout cascades**: Short lock timeouts (< 10s) cause cascading failures under load; use 30s minimum
2. **Missing lock release on error paths**: Always use try-finally or defer for lock cleanup
3. **Queue starvation**: Priority queues can starve low-priority items; implement fairness with max-wait limits
4. **State inconsistency on partial failure**: Update database state atomically with file operations using transactions

**Testing Concurrent Operations:**
- Use chaos testing to inject delays and failures during lock operations
- Test queue behavior under overload (queue full, consumer failures)
- Verify deadlock prevention with concurrent operations on same resources
- Test lock renewal during long-running operations

**Debugging Race Conditions:**
- Enable detailed lock acquisition logging in development (timestamp, resource, holder)
- Use distributed tracing to correlate operations across queue workers
- Add operation checksums to detect state corruption from races
- Monitor lock contention metrics (wait time, timeout rate)

**File Locations:**
- Queue implementation: `src/pipeline/queue-manager.js`
- Lock mechanism: `src/pipeline/distributed-lock.js`
- Pipeline workers: `src/pipeline/workers/`
- Lock renewal: `src/pipeline/lock-renewal.js`

### Development Workflow

When modifying pipeline or concurrent operations:
1. Update lock ordering if adding new lockable resources
2. Update queue tests to verify new operation types
3. Run load tests to verify no performance regression
4. Update docs/practices/development.md with concurrency pattern changes
5. Update README.md troubleshooting if new race conditions discovered

## Team Usage

1. Install cipherpowers as a Claude Code plugin
2. Install superpowers for universal skills
3. Both collections work together seamlessly
4. Commands dispatch to skills from either location

