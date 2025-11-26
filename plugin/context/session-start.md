# CipherPowers Session Context

**Plugin Environment:** CipherPowers is active. Use `${CLAUDE_PLUGIN_ROOT}` for all plugin file references.

## Agent Selection

Before dispatching work to specialized agents, review the agent selection guide:

@${CLAUDE_PLUGIN_ROOT}skills/selecting-agents/SKILL.md

**Available agents:**
- `cipherpowers:rust-agent` - Rust development with TDD
- `cipherpowers:code-review-agent` - Code review before merging
- `cipherpowers:plan-review-agent` - Plan evaluation before execution
- `cipherpowers:ultrathink-debugger` - Complex multi-component debugging
- `cipherpowers:technical-writer` - Documentation sync after code changes
- `cipherpowers:commit-agent` - Atomic commits with conventional format

**Commands:**
- `/summarise` - Learning capture after work completion (uses capturing-learning skill)

**Selection criteria:**
1. Task type (implementation, debugging, review, documentation)
2. Complexity (simple fix vs multi-component investigation)
3. Technology (Rust vs other languages)
4. Explicit guidance in task description

**Do NOT use naive keyword matching** - analyze what the task requires.
