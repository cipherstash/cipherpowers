---
name: test-paths
description: Test file path resolution in plugin agents
---

This command tests whether file references work correctly in plugin agent contexts.

## Test Scenarios

This will test file path resolution in two scenarios:

1. **Direct subagent invocation** - Spawning path-test-agent via Task tool
2. **File reference verification** - Confirming @ syntax resolves correctly

## Execution

You MUST execute this test by spawning the path-test-agent as a subagent.

Use the Task tool:
```
Task(
  subagent_type: "cipherpowers:path-test-agent",
  description: "Test file path resolution",
  prompt: "Execute the path test procedure exactly as specified in your instructions."
)
```

After the agent completes, analyze the results and report:

1. Which files were successfully read
2. Which files failed (if any)
3. Whether relative paths (@skills/..., @standards/...) work in subagent context
4. Recommendation for convention to use

## Expected Outcome

If the test PASSES, relative paths work correctly and we can use `@skills/...` syntax throughout all agents.

If the test FAILS, we need to investigate alternative approaches.
