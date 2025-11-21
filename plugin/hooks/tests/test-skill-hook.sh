#!/usr/bin/env bash

# Create test context file
mkdir -p /tmp/test-skill/.claude/context
echo "TDD requirements" > /tmp/test-skill/.claude/context/test-driven-development-start.md

# Mock hook input
INPUT=$(jq -n '{
  hook_event_name: "SkillStart",
  skill: "test-driven-development",
  user_message: "implement feature with TDD",
  cwd: "/tmp/test-skill"
}')

# Run dispatcher
result=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

# Verify context was injected
if echo "$result" | jq -e '.additionalContext' > /dev/null 2>&1; then
  content=$(echo "$result" | jq -r '.additionalContext')
  if [ "$content" = "TDD requirements" ]; then
    echo "PASS: Context injected for SkillStart"
  else
    echo "FAIL: Wrong content: $content"
    exit 1
  fi
else
  echo "FAIL: No additionalContext in output"
  exit 1
fi

rm -rf /tmp/test-skill
