# Quality Hooks Integration Tests

Manual integration tests to verify quality hooks work with real agents.

## Prerequisites

- Quality hooks installed and registered
- `gates.json` configured with test commands
- Claude Code with plugin loaded

## Test 1: PostToolUse Hook Trigger

**Setup:**
```bash
# Ensure gates.json has Edit tool enabled
jq '.hooks.PostToolUse.enabled_tools' plugin/hooks/gates.json
# Should include "Edit"
```

**Test:**
1. Create a test file: `echo "# Test" > /tmp/test-hooks.md`
2. Use Edit tool to modify file
3. Observe PostToolUse hook execution

**Expected:**
- Hook runs after Edit completes
- Check gate executes
- If gate passes: No output (CONTINUE)
- If gate fails: BLOCK decision with error output

## Test 2: SubagentStop Hook Trigger

**Setup:**
```bash
# Ensure gates.json has rust-agent enabled
jq '.hooks.SubagentStop.enabled_agents' plugin/hooks/gates.json
# Should include "rust-agent"
```

**Test:**
1. Dispatch rust-agent with simple task
2. Agent completes work
3. Observe SubagentStop hook execution

**Expected:**
- Hook runs when agent completes
- Both check and test gates execute
- Gates run in sequence
- Results appear in agent's context

## Test 3: Gate Chaining

**Setup:**
```bash
# Configure gate chaining
cat > plugin/hooks/gates.json <<'EOF'
{
  "gates": {
    "first": {
      "command": "echo 'First gate'",
      "on_pass": "second"
    },
    "second": {
      "command": "echo 'Second gate'",
      "on_pass": "CONTINUE"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["first"]
    }
  }
}
EOF
```

**Test:**
1. Edit a file with Edit tool
2. Observe hook execution

**Expected:**
- First gate executes
- On pass, second gate executes (chaining)
- Both gates must pass for CONTINUE

## Test 4: BLOCK Action

**Setup:**
```bash
# Configure gate to fail and block
cat > plugin/hooks/gates.json <<'EOF'
{
  "gates": {
    "block-test": {
      "command": "exit 1",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["block-test"]
    }
  }
}
EOF
```

**Test:**
1. Edit a file with Edit tool
2. Observe BLOCK behavior

**Expected:**
- Gate fails
- Hook outputs: `{"decision": "block", "reason": "..."}`
- Agent cannot proceed
- Error message includes gate output

## Test 5: CONTINUE on Failure (Warn Only)

**Setup:**
```bash
# Configure gate to fail but continue
cat > plugin/hooks/gates.json <<'EOF'
{
  "gates": {
    "warn-test": {
      "command": "exit 1",
      "on_fail": "CONTINUE"
    }
  },
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["warn-test"]
    }
  }
}
EOF
```

**Test:**
1. Edit a file with Edit tool
2. Observe warning behavior

**Expected:**
- Gate fails
- Hook outputs: `{"additionalContext": "⚠️ Gate 'warn-test' failed..."}`
- Execution continues despite failure
- Warning appears in context

## Test 6: Missing Gate Error

**Setup:**
```bash
# Configure gates.json with reference to non-existent gate
cat > plugin/hooks/gates.json <<'EOF'
{
  "gates": {},
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["nonexistent"]
    }
  }
}
EOF
```

**Test:**
1. Edit a file with Edit tool
2. Observe error handling

**Expected:**
- Hook outputs: `{"continue": false, "message": "Gate 'nonexistent' referenced but not defined..."}`
- Claude stops entirely (STOP action)
- Clear error message

## Test 7: Tool Filtering

**Setup:**
```bash
# Configure PostToolUse for Edit only (not Read)
jq '.hooks.PostToolUse.enabled_tools = ["Edit"]' plugin/hooks/gates.json > /tmp/gates.json
mv /tmp/gates.json plugin/hooks/gates.json
```

**Test:**
1. Use Read tool
2. Use Edit tool

**Expected:**
- Read tool: No hook execution (not in enabled_tools)
- Edit tool: Hook executes normally

## Test 8: Agent Filtering

**Setup:**
```bash
# Configure SubagentStop for rust-agent only
jq '.hooks.SubagentStop.enabled_agents = ["rust-agent"]' plugin/hooks/gates.json > /tmp/gates.json
mv /tmp/gates.json plugin/hooks/gates.json
```

**Test:**
1. Dispatch rust-agent
2. Dispatch code-review-agent

**Expected:**
- rust-agent: Hook executes when agent completes
- code-review-agent: No hook execution (not in enabled_agents)

## Verification Checklist

After running all tests:

- [ ] PostToolUse hook triggers on enabled tools
- [ ] PostToolUse hook ignores non-enabled tools
- [ ] SubagentStop hook triggers on enabled agents
- [ ] SubagentStop hook ignores non-enabled agents
- [ ] Gate chaining works correctly
- [ ] BLOCK action prevents agent continuation
- [ ] CONTINUE action proceeds with/without warning
- [ ] STOP action halts Claude
- [ ] Missing gate produces STOP with error
- [ ] Error messages are clear and actionable
- [ ] Hook output is valid JSON
- [ ] Hooks handle missing config gracefully

## Troubleshooting

**Hook doesn't run:**
- Check `hooks.json` registered correctly
- Verify `CLAUDE_PLUGIN_ROOT` is set
- Check tool/agent is in enabled list

**Gate command fails:**
- Verify command exists: `which <command>`
- Test command manually: `<command>`
- Check gate configuration in `gates.json`

**JSON parse errors:**
- Validate `gates.json`: `jq . plugin/hooks/gates.json`
- Check hook script syntax: `bash -n plugin/hooks/*.sh`
- Review error messages for formatting issues
