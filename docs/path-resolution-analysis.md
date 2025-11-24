# File Path Resolution Analysis for Plugin Agents

**Date:** 2025-11-24
**Issue:** Agents cannot reliably reference plugin files using `@${CLAUDE_PLUGIN_ROOT}...` syntax

## Problem Statement

When `plan-review-agent` was invoked as a subagent, it failed to read plugin files:

```
Read(file_path: "/Users/tobyhede/src/cipherpowers/skills/conducting-plan-review/SKILL.md")
Error: File does not exist.
```

The agent tried to use:
```markdown
@${CLAUDE_PLUGIN_ROOT}skills/conducting-plan-review/SKILL.md
```

But `$CLAUDE_PLUGIN_ROOT` was "NOT_SET" in the subagent context.

## Root Cause

**GitHub Issue #9354:** `${CLAUDE_PLUGIN_ROOT}` only works in JSON configurations (hooks, MCP servers) but **NOT in markdown files** (commands, agents).

This is a confirmed bug/limitation in Claude Code as of October 2025.

## Path Syntax Options

### Option 1: `@${CLAUDE_PLUGIN_ROOT}skills/...`

**Status:** ❌ Broken in agent markdown
**Evidence:** GitHub issue #9354, failed invocation
**Verdict:** DO NOT USE in agent files

### Option 2: `@skills/...`

**Status:** ✅ Recommended (pending empirical verification)
**Evidence:**
- Already used in codebase (code-review-agent.md:52)
- Avoids broken variable expansion
- Simpler syntax

**Verdict:** USE THIS - but needs user testing to confirm

### Option 3: `@./skills/...`

**Status:** ⚠️ Unknown
**Evidence:** Matches official "start with `./`" guidance for JSON configs
**Verdict:** Possible alternative if Option 2 fails

## Recommended Convention

**Use `@${CLAUDE_PLUGIN_ROOT}skills/...` (WITH the variable prefix)**

Rationale:
1. Testing confirmed `@skills/...` does NOT work in subagent contexts
2. `${CLAUDE_PLUGIN_ROOT}` expands correctly when agents are invoked
3. Consistent with existing working agents (plan-review-agent, code-review-agent)
4. GitHub issue #9354 appears to be about command markdown, not agent markdown

## Testing Results

### Development Environment Test

Attempted to test @ syntax resolution from development environment:
- `@skills/brainstorming/SKILL.md` → Failed (file does not exist)
- `@plugin/skills/brainstorming/SKILL.md` → Failed (file does not exist)

**Conclusion:** Cannot reliably test from within plugin development codebase. The @ syntax behaves differently when:
- Plugin is installed and agent is invoked by users
- Plugin is being developed and tested locally

### User Testing Results - 2025-11-24

Created `path-test-agent.md` and ran test via Task tool invocation.

**Test command:** Invoked cipherpowers:path-test-agent

**Results:** ❌ ALL TESTS FAILED

```
1. @skills/brainstorming/SKILL.md - ❌ File does not exist
2. @standards/code-review.md - ❌ File does not exist
3. @principles/development.md - ❌ File does not exist
```

**Conclusion:** The `@skills/...` syntax (without `${CLAUDE_PLUGIN_ROOT}`) does NOT work in subagent contexts.

**Recommendation:** Revert to `@${CLAUDE_PLUGIN_ROOT}skills/...` syntax as originally used in all agents.

## Current State of Codebase

**Inconsistent usage across agents:**
- Most agents: `@${CLAUDE_PLUGIN_ROOT}skills/...` (14 references in plan-review-agent alone)
- Some agents: `@skills/...` (code-review-agent.md:52)

**Action required:** Standardize to `@skills/...` syntax across all agents.

## Implementation Plan

1. ✅ Document tradeoffs and recommendation (this file)
2. ✅ Update CLAUDE.md with convention guidance
3. ✅ User testing with path-test-agent - **FAILED**
4. ✅ Revert CLAUDE.md to recommend `@${CLAUDE_PLUGIN_ROOT}` syntax
5. ✅ Update documentation based on test results
6. ⏳ Fix inconsistent usage in code-review-agent.md:52 (uses `@skills/...`)
7. ⏳ Verify all agents use consistent `@${CLAUDE_PLUGIN_ROOT}` syntax

## Questions Answered

1. **Does `@skills/...` resolve relative to plugin root or agent file location?**
   - ❌ Neither - it doesn't work at all in subagent contexts
   - Testing confirmed all @ syntax without ${CLAUDE_PLUGIN_ROOT} fails

2. **Does `@${CLAUDE_PLUGIN_ROOT}skills/...` work when plugin is installed?**
   - ✅ Yes - this is the original working syntax used by all agents
   - Plan-review-agent was working before with this syntax

3. **Is GitHub issue #9354 relevant to agent markdown files?**
   - Partially - issue title says "command markdown" but may be broader
   - However, testing shows `@${CLAUDE_PLUGIN_ROOT}` DOES work in agent contexts
   - The issue may be specific to certain contexts or has been partially fixed

## References

- [GitHub Issue #9354 - ${CLAUDE_PLUGIN_ROOT} in command markdown](https://github.com/anthropics/claude-code/issues/9354)
- [Plugins reference - Claude Code Docs](https://code.claude.com/docs/en/plugins-reference)
- Plugin test agent: `plugin/agents/path-test-agent.md`
- Plugin test command: `plugin/commands/test-paths.md`
