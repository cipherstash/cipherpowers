#!/usr/bin/env bash
source "$(dirname "$0")/../shared-functions.sh"

# Setup test directory and ensure cleanup
TEST_DIR="/tmp/test-inject"
trap "rm -rf $TEST_DIR" EXIT

# Test: inject_context_file outputs valid JSON
mkdir -p $TEST_DIR/.claude/context
echo "Security checklist content" > $TEST_DIR/.claude/context/test.md

result=$(inject_context_file "$TEST_DIR/.claude/context/test.md")

# Verify JSON structure
if echo "$result" | jq -e '.additionalContext' > /dev/null; then
  echo "PASS: Valid JSON with additionalContext"
else
  echo "FAIL: Invalid JSON structure"
  exit 1
fi

# Verify content
content=$(echo "$result" | jq -r '.additionalContext')
if [ "$content" = "Security checklist content" ]; then
  echo "PASS: Content matches"
else
  echo "FAIL: Expected 'Security checklist content', got '$content'"
  exit 1
fi

# Test: inject_context_file handles special characters
cat > $TEST_DIR/.claude/context/special.md << 'EOF'
Content with "quotes", 'single quotes',
newlines, and \backslashes\
Plus $variables and `backticks`
EOF

result=$(inject_context_file "$TEST_DIR/.claude/context/special.md")

# Verify special characters are preserved
content=$(echo "$result" | jq -r '.additionalContext')
expected='Content with "quotes", '"'"'single quotes'"'"',
newlines, and \backslashes\
Plus $variables and `backticks`'

if [ "$content" = "$expected" ]; then
  echo "PASS: Special characters preserved"
else
  echo "FAIL: Special characters not preserved correctly"
  echo "Expected: $expected"
  echo "Got: $content"
  exit 1
fi
