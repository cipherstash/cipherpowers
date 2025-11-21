#!/bin/bash
# plugin/hooks/hooks-app/test-cli.sh

# Test 1: Empty stdin
echo "Test 1: Empty stdin"
echo '{}' | node dist/cli.js

# Test 2: Valid PostToolUse input
echo "Test 2: PostToolUse"
echo '{"hook_event_name":"PostToolUse","cwd":"'$(pwd)'","tool_name":"Edit"}' | node dist/cli.js

# Test 3: Invalid JSON
echo "Test 3: Invalid JSON"
echo 'not json' | node dist/cli.js
