#!/usr/bin/env bash

# Create test context file
mkdir -p /tmp/test-slash/.claude/context
echo "Code review requirements" > /tmp/test-slash/.claude/context/code-review-start.md

# Mock hook input
INPUT=$(jq -n '{
  hook_event_name: "SlashCommandStart",
  command: "/code-review",
  user_message: "review this code",
  cwd: "/tmp/test-slash"
}')

# Run dispatcher
result=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

# Verify context was injected
if echo "$result" | jq -e '.additionalContext' > /dev/null 2>&1; then
  content=$(echo "$result" | jq -r '.additionalContext')
  if [ "$content" = "Code review requirements" ]; then
    echo "PASS: Context injected for SlashCommandStart"
  else
    echo "FAIL: Wrong content: $content"
    exit 1
  fi
else
  echo "FAIL: No additionalContext in output"
  exit 1
fi

rm -rf /tmp/test-slash
