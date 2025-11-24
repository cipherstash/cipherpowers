# Following Plans - Plan Compliance System

## Overview

A simple, explicit system to prevent agents from deviating from implementation plans without approval.

**Problem:** Agents sometimes rationalize "simpler" approaches that were already considered and rejected during design, leading to expensive rework when the divergence is discovered later.

**Solution:** Algorithmic decision tree + STATUS reporting + gate enforcement + user escalation.

## Components

### 1. following-plans Skill (Algorithmic)
**Location:** `plugin/skills/following-plans/SKILL.md`

**Purpose:** Embedded in agent prompts to define clear boundaries:
- What changes are allowed (syntax fixes)
- What requires BLOCKED report (approach/architecture changes)

**Decision tree format:** Boolean questions with no room for interpretation.

**Key principle:** Better to report BLOCKED unnecessarily than deviate without approval.

### 2. STATUS Reporting Protocol
**Required in every agent completion:**

```
STATUS: OK
TASK: {task identifier}
SUMMARY: {what was done}
```

Or:

```
STATUS: BLOCKED
REASON: {why plan approach won't work}
TASK: {task identifier}
```

### 3. Plan Compliance Gate
**Location:** `plugin/hooks/gates/plan-compliance.sh`

**Runs on:** SubagentStop hook

**Checks:**
- STATUS missing → BLOCK (agent must provide status)
- STATUS: BLOCKED → BLOCK (dispatcher handles escalation)
- STATUS: OK → CONTINUE (chain to check/test gates)

### 4. Dispatcher Handling (executing-plans skill)
**Location:** `plugin/skills/executing-plans/SKILL.md`

**When agent reports BLOCKED:**
1. Read BLOCKED reason
2. Review plan/design context
3. Ask user via AskUserQuestion (4 options: approve, revise, enforce, investigate)
4. Execute user decision

**No automatic retries. No automatic approvals. User decides.**

## Setup

**No setup required!** The plan-compliance gate runs automatically on all SubagentStop events, just like the commands gate runs on all UserPromptSubmit events.

### Optional: Add Additional Gates

If you want to chain additional gates after plan-compliance (like check/test), edit your `.claude/gates.json`:

```json
{
  "gates": {
    "check": {
      "description": "Run quality checks",
      "command": "mise run check",
      "on_pass": "test",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run tests",
      "command": "mise run test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "SubagentStop": {
      "enabled_agents": ["general-purpose", "cipherpowers:rust-agent", "cipherpowers:code-agent"],
      "gates": ["check"]
    }
  }
}
```

**Flow:** plan-compliance (built-in) → check → test

### Example Configuration

See `${CLAUDE_PLUGIN_ROOT}/hooks/examples/plan-execution.json` for a complete example.

## Usage

### During Plan Execution

The executing-plans skill automatically:

1. **Embeds following-plans skill** in agent prompts
2. **Checks agent STATUS** after completion
3. **Handles BLOCKED** by escalating to user

### Agent Behavior

Agents following the embedded skill will:

**For syntax fixes:** Make the change, note in completion
```
STATUS: OK
TASK: Task 3 - Implement auth
SUMMARY: Implemented auth. Fixed function name from plan (was getUserData, actually getUser).
```

**For approach changes:** Report BLOCKED
```
STATUS: BLOCKED
REASON: Plan specifies JWT but existing service uses OAuth2. JWT would require refactoring entire auth system.
TASK: Task 3 - Implement auth middleware
```

### User Decisions

When agent reports BLOCKED, you get clear options:

1. **Trust agent** - Approve deviation, update plan
2. **Revise plan** - Update with different approach
3. **Enforce plan** - Agent must follow plan as written
4. **Investigate** - Need more context

## Benefits

✅ **Prevents silent deviations** - Agents can't rationalize around plan
✅ **Early detection** - Blockers caught immediately, not discovered later
✅ **Explicit approval** - User decides on all plan deviations
✅ **Simple** - No automatic retries, no state tracking, no complexity
✅ **Clear boundaries** - Algorithmic decision tree (no interpretation)
✅ **Audit trail** - STATUS in agent output provides record

## Example Scenarios

### Scenario 1: Syntax Fix (Allowed)
**Plan:** "Call getUserData() to fetch user"
**Reality:** Function is actually `getUser()`
**Agent action:** Fix syntax, report STATUS: OK with note
**Result:** No BLOCKED, continues

### Scenario 2: Approach Change (Blocked)
**Plan:** "Implement manual JWT verification"
**Agent thought:** "Library X is simpler"
**Agent action:** Report STATUS: BLOCKED
**Result:** User decides: trust agent, revise plan, or enforce

### Scenario 3: Plan Error (Blocked)
**Plan:** Task 3 says PostgreSQL, Task 5 says MongoDB
**Agent action:** Report STATUS: BLOCKED (plan contradiction)
**Result:** User fixes plan, execution continues

## Testing

Test the gate manually:

```bash
# Test with STATUS: OK
echo '{"output": "STATUS: OK\nTask complete"}' | \
  HOOK_INPUT='{"output": "STATUS: OK\nTask complete"}' \
  ${CLAUDE_PLUGIN_ROOT}/hooks/gates/plan-compliance.sh

# Test with STATUS: BLOCKED
echo '{"output": "STATUS: BLOCKED\nREASON: Plan approach won't work"}' | \
  HOOK_INPUT='{"output": "STATUS: BLOCKED\nREASON: Plan approach won't work"}' \
  ${CLAUDE_PLUGIN_ROOT}/hooks/gates/plan-compliance.sh

# Test with missing STATUS
echo '{"output": "Task complete"}' | \
  HOOK_INPUT='{"output": "Task complete"}' \
  ${CLAUDE_PLUGIN_ROOT}/hooks/gates/plan-compliance.sh
```

## Design Principles

**Simplicity over automation:** No automatic retries. User decides on deviations.

**Explicit over implicit:** STATUS required. BLOCKED is explicit escalation.

**Algorithmic over imperative:** Decision tree, not guidelines. No interpretation.

**User control:** Agent reports, gate enforces, user decides.
