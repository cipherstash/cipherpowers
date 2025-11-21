#!/usr/bin/env bash
source "$(dirname "$0")/../shared-functions.sh"

# Setup test directory and ensure cleanup
TEST_DIR="/tmp/test-context"
trap "rm -rf $TEST_DIR" EXIT

# Test: discover_context_file finds flat structure
mkdir -p $TEST_DIR/.claude/context
echo "test content" > $TEST_DIR/.claude/context/code-review-start.md

result=$(discover_context_file "$TEST_DIR" "code-review" "start")
expected="$TEST_DIR/.claude/context/code-review-start.md"

if [ "$result" = "$expected" ]; then
  echo "PASS: Flat structure discovery"
else
  echo "FAIL: Expected $expected, got $result"
  exit 1
fi
