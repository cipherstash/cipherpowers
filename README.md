# CipherPowers

A Claude Code plugin providing comprehensive development workflows, skills, and documentation standards for teams.

## Overview

CipherPowers is a "batteries included" starter kit for development teams using Claude Code. It provides:

- **Skills**: Reusable process knowledge for workflows, planning, and code quality
- **Commands**: Slash commands for common development tasks
- **Agents**: Specialized subagents for planning, review, and execution
- **Documentation**: Standards, guidelines, and best practices

## Installation

### Prerequisites

1. Install [Claude Code](https://claude.com/claude-code)
2. Install [superpowers](https://github.com/obra/superpowers) - provides universal skills from [superpowers-skills](https://github.com/obra/superpowers-skills)

### Install CipherPowers

```bash
# Clone the repository
git clone <repository-url> cipherpowers

# Install as Claude Code plugin
# (Follow Claude Code plugin installation instructions)
```

## Structure

```
cipherpowers/
├── .claude-plugin/
│   └── plugin.json        # Plugin metadata
├── commands/              # Slash commands
│   ├── plan.md           # Work planning workflow
│   ├── work.md           # Active work management
│   └── ...
├── agents/                # Subagent prompts
│   ├── work-planner.md   # Planning specialist
│   ├── code-reviewer.md  # Review specialist
│   └── ...
├── skills/                # Org-specific skills
│   ├── planning-work/
│   ├── linear-workflow/
│   └── ...
├── tools/
│   └── find-skills        # Unified skill discovery
├── docs/
│   ├── practices/         # Coding standards & guidelines
│   ├── examples/          # Templates & examples
│   └── ...
└── README.md
```

## Authentication

CipherPowers supports OAuth 2.0 authentication for secure access to protected resources.

### Quick Start

```bash
# Configure OAuth credentials
export OAUTH_CLIENT_ID="your-client-id"
export OAUTH_CLIENT_SECRET="your-client-secret"
export OAUTH_REDIRECT_URI="http://localhost:8080/callback"

# Initialize authentication
./scripts/auth-init.sh
```

### Supported Flows

- **Authorization Code Flow**: For web applications with server-side components
- **PKCE (Proof Key for Code Exchange)**: For single-page apps and mobile clients
- **Client Credentials**: For service-to-service authentication

### Configuration

Authentication settings are configured in `.env` or via environment variables:

```bash
OAUTH_CLIENT_ID=<your-client-id>
OAUTH_CLIENT_SECRET=<your-client-secret>
OAUTH_REDIRECT_URI=<your-callback-url>
OAUTH_SCOPE=read,write
OAUTH_TOKEN_ENDPOINT=https://auth.example.com/token
OAUTH_AUTHORIZATION_ENDPOINT=https://auth.example.com/authorize
```

For detailed configuration options and examples, see:
- [Authentication Practices](/Users/tobyhede/src/cipherpowers/docs/practices/authentication.md)
- [OAuth Examples](/Users/tobyhede/src/cipherpowers/docs/examples/oauth/)

## Usage

### Commands

Slash commands provide quick access to common workflows:

```
/plan           # Create detailed implementation plan
/work           # Complete workflow for implementing planned work (uses queue system)
/code-review    # Request code review
/commit         # Create structured commit
/execute        # Execute an active work plan (manages file processing pipeline)
```

**Commands using the file processing pipeline:**
- `/work` - Queues work items and tracks completion via pipeline
- `/execute` - Directly manages pipeline execution with lock coordination
- `/commit` - Uses pipeline to validate file changes before committing

These commands now implement proper locking and queue management to prevent race conditions. See [Concurrency Patterns](/Users/tobyhede/src/cipherpowers/CLAUDE.md#concurrency-and-race-condition-patterns) for implementation details.

### Skills

Skills are discoverable using the custom find-skills tool:

```bash
# Search all skills (cipherpowers + superpowers)
find-skills "planning"

# Search only local skills
find-skills --local "workflow"

# Search only superpowers
find-skills --upstream "testing"
```

### Documentation

Reference materials in `docs/`:

- `practices/` - Coding standards, git conventions, testing guidelines
- `examples/` - Templates and real-world examples

## Architecture

### Three-Layer Design

**1. Skills Layer** (`skills/`)

Reusable process knowledge following the superpowers framework:
- Testable with subagents (TDD for documentation)
- Rich metadata for discovery
- References to upstream superpowers skills

**2. Automation Layer** (`commands/`, `agents/`)

Workflow automation:
- Commands dispatch to agents
- Agents follow skills for methodology
- Project-specific orchestration

**3. Documentation Layer** (`docs/`)

Supporting materials:
- Standards and guidelines
- Templates and examples
- Team reference documentation

### Integration with Superpowers

CipherPowers extends superpowers with organization-specific content:

- Custom `find-skills` searches both collections
- Commands reference skills from either location
- Skills can be developed locally, then contributed upstream

## Creating Skills

When adding new skills to `skills/`:

1. **Check superpowers first** - Don't duplicate existing skills
2. **Follow TDD** - Test with subagents before writing
3. **Read the meta-skill** - `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md`
4. **Use TodoWrite** - Track the skill creation checklist
5. **Consider upstream** - Universal skills can be contributed back

## Contributing

### Upstreaming Skills

Universal skills developed in cipherpowers can be contributed to superpowers:

1. Test thoroughly with subagents
2. Ensure it's broadly applicable (not org-specific)
3. Follow superpowers contribution guidelines
4. Submit PR to [superpowers-skills](https://github.com/obra/superpowers-skills)

### Local Development

Organization-specific skills stay in `skills/`:
- Linear workflow integration
- Team-specific conventions
- Internal tooling guidance

## Team Customization

Fork cipherpowers to create your own team toolkit:

1. Fork this repository
2. Customize commands and agents for your workflow
3. Add org-specific skills
4. Update practices documentation
5. Share with team

## Troubleshooting

### Authentication Issues

**Token expiration errors:**
- Check token refresh logic is properly implemented
- Verify `OAUTH_TOKEN_EXPIRY` buffer is set (default: 300 seconds)
- Review logs: `tail -f logs/auth.log`

**Redirect URI mismatch:**
- Ensure `OAUTH_REDIRECT_URI` exactly matches OAuth provider configuration
- Check for trailing slashes and protocol (http vs https)
- Verify port numbers match in development

**PKCE code challenge failures:**
- Confirm code verifier is stored between authorization and token requests
- Check SHA-256 hashing implementation
- Verify base64url encoding (not standard base64)

**Rate limiting on token endpoint:**
- Implement exponential backoff (handled by default in `src/auth/retry.js`)
- Check token caching is enabled to reduce requests
- Monitor rate limit headers in responses

**"Invalid client" errors:**
- Verify client credentials are correctly set
- Check if client secret needs URL encoding
- Confirm OAuth provider has client ID whitelisted

For additional help, see [Authentication Practices](/Users/tobyhede/src/cipherpowers/docs/practices/authentication.md).

### API Error Handling

**Validation errors (400):**
- Check request payload matches expected schema
- Verify all required fields are present
- Review field types and format requirements
- See API documentation for schema details

**Resource not found errors (404):**
- Confirm resource ID exists and is accessible
- Check permissions for the requested resource
- Verify correct API endpoint path

**General error responses:**
- All errors follow consistent JSON structure (see README_API.md)
- Error codes indicate specific failure types
- Details field provides debugging context

For error handling patterns and implementation details, see [Development Practices](/Users/tobyhede/src/cipherpowers/docs/practices/development.md#error-handling).

### File Processing Pipeline Issues

**Queue deadlocks:**
- Symptom: Pipeline stops processing, queue depth increases but no worker activity
- Cause: Locks acquired in inconsistent order across workers
- Solution: Verify lock acquisition follows alphabetical order by resource ID
- Debug: Check `logs/pipeline.log` for "waiting for lock" messages with timestamps

**Race condition in file state:**
- Symptom: Duplicate processing of same file, or files marked complete without processing
- Cause: Read-modify-write operations on file state without proper locking
- Solution: Ensure all state updates hold appropriate lock using try-finally
- Debug: Enable detailed lock logging with `LOCK_DEBUG=true`, check for concurrent state changes

**Lock timeout cascades:**
- Symptom: Sporadic lock timeout errors under normal load
- Cause: Lock timeout set too short (< 30 seconds) for file operations
- Solution: Increase lock timeout to 30+ seconds, implement exponential backoff
- Debug: Monitor lock acquisition time with `GET /metrics/locks` endpoint

**Queue starvation:**
- Symptom: Low-priority items never process despite available workers
- Cause: Priority queue with no fairness mechanism
- Solution: Implement max-wait time (5 minutes) to force low-priority processing
- Debug: Check queue metrics with `/queue/stats` showing age distribution

**Partial failure state inconsistency:**
- Symptom: Database shows file as processed but output files missing
- Cause: Non-atomic updates to file state and file system operations
- Solution: Use database transactions wrapping both state update and file operations
- Debug: Enable transaction logging, check for rollback events in logs

**Lock not released on error:**
- Symptom: Specific resources permanently locked, manual intervention needed
- Cause: Missing try-finally around lock usage in error paths
- Solution: Audit all lock acquisition code for try-finally pattern
- Debug: Use `scripts/list-stuck-locks.sh` to identify locks held longer than 5 minutes

For concurrency patterns and implementation details, see [Development Practices](/Users/tobyhede/src/cipherpowers/docs/practices/development.md#async-operations-and-concurrency).

## License

[Add your license here]

## Credits

Built on [superpowers](https://github.com/obra/superpowers) by @obra
- Skills from [superpowers-skills](https://github.com/obra/superpowers-skills)
