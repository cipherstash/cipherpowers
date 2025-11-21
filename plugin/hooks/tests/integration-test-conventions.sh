#!/usr/bin/env bash
set -euo pipefail

echo "=== Integration Test: Convention-Based Context Injection ==="

# Setup test environment
TEST_DIR="/tmp/test-convention-integration-$$"
mkdir -p "$TEST_DIR/.claude/context"
trap "rm -rf $TEST_DIR" EXIT

# Create minimal gates.json so dispatcher doesn't exit early
cat > "$TEST_DIR/.claude/gates.json" << 'GATES_EOF'
{
  "gates": {},
  "hooks": {}
}
GATES_EOF

# Test 1: SlashCommandStart with flat structure
echo "Test 1: SlashCommandStart with flat structure"
echo "Security requirements" > "$TEST_DIR/.claude/context/code-review-start.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandStart",
  command: "/code-review",
  user_message: "review code",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "Security requirements"; then
  echo "✓ PASS: Flat structure context injection"
else
  echo "✗ FAIL: Context not injected for flat structure"
  exit 1
fi

# Test 2: SlashCommandEnd with organized structure
echo "Test 2: SlashCommandEnd with organized structure"
mkdir -p "$TEST_DIR/.claude/context/slash-command"
echo "Review complete checklist" > "$TEST_DIR/.claude/context/slash-command/code-review-end.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandEnd",
  command: "/code-review",
  user_message: "done",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "Review complete"; then
  echo "✓ PASS: Organized structure context injection"
else
  echo "✗ FAIL: Context not injected for organized structure"
  exit 1
fi

# Test 3: SkillStart with hierarchical structure
echo "Test 3: SkillStart with hierarchical structure"
mkdir -p "$TEST_DIR/.claude/context/skill/test-driven-development"
echo "TDD standards" > "$TEST_DIR/.claude/context/skill/test-driven-development/start.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SkillStart",
  skill: "test-driven-development",
  user_message: "implement with TDD",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "TDD standards"; then
  echo "✓ PASS: Hierarchical structure context injection"
else
  echo "✗ FAIL: Context not injected for hierarchical structure"
  exit 1
fi

# Test 4: No context file (should not error)
echo "Test 4: No context file exists"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandStart",
  command: "/plan",
  user_message: "create plan",
  cwd: $cwd
}')

if echo "$INPUT" | bash plugin/hooks/dispatcher.sh > /dev/null 2>&1; then
  echo "✓ PASS: No error when context file doesn't exist"
else
  echo "✗ FAIL: Error when context file doesn't exist"
  exit 1
fi

# Test 5: Priority order (flat beats organized)
echo "Test 5: Discovery priority order"
echo "Flat version" > "$TEST_DIR/.claude/context/plan-start.md"
mkdir -p "$TEST_DIR/.claude/context/slash-command"
echo "Organized version" > "$TEST_DIR/.claude/context/slash-command/plan-start.md"

INPUT=$(jq -n --arg cwd "$TEST_DIR" '{
  hook_event_name: "SlashCommandStart",
  command: "/plan",
  user_message: "plan",
  cwd: $cwd
}')

RESULT=$(echo "$INPUT" | bash plugin/hooks/dispatcher.sh)

if echo "$RESULT" | jq -e '.additionalContext' | grep -q "Flat version"; then
  echo "✓ PASS: Flat structure has priority"
else
  echo "✗ FAIL: Wrong priority order"
  exit 1
fi

echo ""
echo "=== All Integration Tests Passed ==="
