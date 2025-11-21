#!/bin/bash
# plugin/hooks/tests/test-typescript-app.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
HOOKS_APP="$PLUGIN_ROOT/hooks/hooks-app"

echo "Building hooks-app..."
cd "$HOOKS_APP" && npm install && npm run build

# Create temp directory for tests
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

echo "Test 1: No config - should exit cleanly"
INPUT='{"hook_event_name":"PostToolUse","cwd":"'$TEST_DIR'","tool_name":"Edit"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
if [ -n "$OUTPUT" ]; then
  echo "FAIL: Expected empty output, got: $OUTPUT"
  exit 1
fi
echo "PASS"

echo "Test 2: Config with shell command gate"
mkdir -p "$TEST_DIR/.claude"
cat > "$TEST_DIR/.claude/gates.json" <<EOF
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["test-gate"]
    }
  },
  "gates": {
    "test-gate": {
      "command": "echo 'Gate passed'",
      "on_pass": "CONTINUE"
    }
  }
}
EOF

INPUT='{"hook_event_name":"PostToolUse","cwd":"'$TEST_DIR'","tool_name":"Edit"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
if ! echo "$OUTPUT" | grep -q "Gate passed"; then
  echo "FAIL: Expected 'Gate passed' in output"
  exit 1
fi
echo "PASS"

echo "Test 3: Gate failure with BLOCK action"
cat > "$TEST_DIR/.claude/gates.json" <<EOF
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["fail-gate"]
    }
  },
  "gates": {
    "fail-gate": {
      "command": "exit 1",
      "on_fail": "BLOCK"
    }
  }
}
EOF

INPUT='{"hook_event_name":"PostToolUse","cwd":"'$TEST_DIR'","tool_name":"Edit"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
if ! echo "$OUTPUT" | grep -q '"decision":"block"'; then
  echo "FAIL: Expected block decision"
  exit 1
fi
echo "PASS"

echo "Test 4: Context injection"
mkdir -p "$TEST_DIR/.claude/context"
echo "# Test Context" > "$TEST_DIR/.claude/context/test-command-start.md"
cat > "$TEST_DIR/.claude/gates.json" <<EOF
{
  "hooks": {
    "SlashCommandStart": {
      "gates": []
    }
  },
  "gates": {}
}
EOF

INPUT='{"hook_event_name":"SlashCommandStart","cwd":"'$TEST_DIR'","command":"/test-command"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
if ! echo "$OUTPUT" | grep -q "Test Context"; then
  echo "FAIL: Expected context injection"
  exit 1
fi
echo "PASS"

echo "Test 5: Empty stdin"
OUTPUT=$(echo "" | node "$HOOKS_APP/dist/cli.js" 2>&1 || true)
if ! echo "$OUTPUT" | grep -q "Invalid JSON"; then
  echo "FAIL: Expected invalid JSON error"
  exit 1
fi
echo "PASS"

echo "Test 6: Truncated JSON"
OUTPUT=$(echo '{"hook_event_name":"PostT' | node "$HOOKS_APP/dist/cli.js" 2>&1 || true)
if ! echo "$OUTPUT" | grep -q "Invalid JSON"; then
  echo "FAIL: Expected invalid JSON error"
  exit 1
fi
echo "PASS"

echo "Test 7: Large output handling (100KB+)"
cat > "$TEST_DIR/.claude/gates.json" <<EOF
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["large-output"]
    }
  },
  "gates": {
    "large-output": {
      "command": "head -c 102400 /dev/urandom | base64",
      "on_pass": "CONTINUE"
    }
  }
}
EOF

INPUT='{"hook_event_name":"PostToolUse","cwd":"'$TEST_DIR'","tool_name":"Edit"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
OUTPUT_SIZE=${#OUTPUT}
if [ "$OUTPUT_SIZE" -lt 100000 ]; then
  echo "FAIL: Expected large output (>100KB), got ${OUTPUT_SIZE} bytes"
  exit 1
fi
echo "PASS (output size: ${OUTPUT_SIZE} bytes)"

echo "Test 8: Command timeout (should complete in ~30s)"
cat > "$TEST_DIR/.claude/gates.json" <<EOF
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["timeout-gate"]
    }
  },
  "gates": {
    "timeout-gate": {
      "command": "sleep 35",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
EOF

START_TIME=$(date +%s)
INPUT='{"hook_event_name":"PostToolUse","cwd":"'$TEST_DIR'","tool_name":"Edit"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

if ! echo "$OUTPUT" | grep -q '"decision":"block"'; then
  echo "FAIL: Expected block decision (timeout should fail the gate)"
  exit 1
fi

if [ "$DURATION" -gt 35 ]; then
  echo "FAIL: Timeout took too long (${DURATION}s, expected ~30s)"
  exit 1
fi
echo "PASS (timed out after ${DURATION}s)"

echo "Test 9: Circular gate chain prevention"
cat > "$TEST_DIR/.claude/gates.json" <<EOF
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit"],
      "gates": ["gate-a"]
    }
  },
  "gates": {
    "gate-a": {
      "command": "echo a",
      "on_pass": "gate-b"
    },
    "gate-b": {
      "command": "echo b",
      "on_pass": "gate-a"
    }
  }
}
EOF

INPUT='{"hook_event_name":"PostToolUse","cwd":"'$TEST_DIR'","tool_name":"Edit"}'
OUTPUT=$(echo "$INPUT" | node "$HOOKS_APP/dist/cli.js")
if ! echo "$OUTPUT" | grep -q "max gate chain depth"; then
  echo "FAIL: Expected circular chain prevention"
  exit 1
fi
echo "PASS"

echo ""
echo "All integration tests passed!"
