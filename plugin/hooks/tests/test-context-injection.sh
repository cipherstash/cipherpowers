#!/usr/bin/env bash
source "$(dirname "$0")/../shared-functions.sh"

# Test: inject_context_file outputs valid JSON
mkdir -p /tmp/test-inject/.claude/context
echo "Security checklist content" > /tmp/test-inject/.claude/context/test.md

result=$(inject_context_file "/tmp/test-inject/.claude/context/test.md")

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

rm -rf /tmp/test-inject
