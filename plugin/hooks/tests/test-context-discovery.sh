#!/usr/bin/env bash
source "$(dirname "$0")/../shared-functions.sh"

# Test: discover_context_file finds flat structure
mkdir -p /tmp/test-context/.claude/context
echo "test content" > /tmp/test-context/.claude/context/code-review-start.md

result=$(discover_context_file "/tmp/test-context" "code-review" "start")
expected="/tmp/test-context/.claude/context/code-review-start.md"

if [ "$result" = "$expected" ]; then
  echo "PASS: Flat structure discovery"
else
  echo "FAIL: Expected $expected, got $result"
  exit 1
fi

rm -rf /tmp/test-context
