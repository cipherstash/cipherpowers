# Code-Rust Command Evaluation

**Date:** 2025-10-15
**Status:** Recommended for ELIMINATION

## Current State

`commands_all/code-rust.md` is a 20-line file that:
1. States "Use the rust-engineer subagent"
2. Lists 4 generic instructions
3. Provides no additional value beyond agent invocation

## Analysis

### Duplication Issues

**Command says:**
```markdown
1. Follow code standards and practices
2. Always add unit tests
3. Always ensure that all tests pass
4. Always ensure that all checks pass
```

**Agent now says (with authority):**
```markdown
YOU MUST follow this sequence. NO EXCEPTIONS.
1. Verify worktree and read all context
2. Implement with TDD
3. Run `mise run test` - ALL tests MUST pass
4. Run `mise run check` - ALL checks MUST pass
5. Request code review BEFORE claiming completion
6. Address ALL review feedback
```

**The command is redundant and weaker.**

### Value Proposition

**What command provides:**
- Discoverable entry point via `/code-rust`

**What command costs:**
- Maintenance of duplicate instructions
- Weaker language that conflicts with agent
- Confusion about source of truth

### Recommendation

**ELIMINATE `commands_all/code-rust.md`**

**Rationale:**
1. Agent is comprehensive and authoritative
2. Command adds no context beyond "use agent"
3. Users can invoke agent directly
4. Simpler architecture: practices → agent (not practices → command → agent)

### Migration Path

**Before:**
```
User types: /code-rust
Command: "Use rust-engineer agent" + weak instructions
Agent: Gets invoked with comprehensive workflow
```

**After:**
```
User: Invokes rust-engineer agent directly
Agent: Comprehensive workflow with persuasion principles
```

**User impact:** None. Agent is already proactively invoked for Rust tasks.

## Decision

**Eliminate command. Rely on agent's proactive invocation.**

If command needed later, recreate with ONLY project context (not workflow duplication).
