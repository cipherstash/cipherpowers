#!/usr/bin/env bash

# Setup test directory and ensure cleanup
TEST_DIR="/tmp/test-skill"
trap "rm -rf $TEST_DIR" EXIT

# Create test context file
# Note: Using minimal fixture content for testing; see plugin/hooks/examples/context/test-driven-development-start.md for production example
mkdir -p $TEST_DIR/.claude/context
echo "TDD requirements" > $TEST_DIR/.claude/context/test-driven-development-start.md

# Mock hook input
INPUT=$(jq -n --arg test_dir "$TEST_DIR" '{
  hook_event_name: "SkillStart",
  skill: "test-driven-development",
  user_message: "implement feature with TDD",
  cwd: $test_dir
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
