#!/usr/bin/env bash
set -euo pipefail

# Plugin Path Injection Gate
# Injects CLAUDE_PLUGIN_ROOT as context for agents to resolve file references

# This gate runs on SubagentStop to inject the plugin root path into agent context
# so agents can properly resolve @${CLAUDE_PLUGIN_ROOT}/... file references

#Use CLAUDE_PLUGIN_ROOT if set otherwise compute from script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"

# Output context injection as JSON
jq -n \
  --arg plugin_root "$PLUGIN_ROOT" \
  '{
    additionalContext: "## Plugin Path Context\n\nFor this session:\n```\nCLAUDE_PLUGIN_ROOT=\($plugin_root)\n```\n\nWhen you see file references like `@${CLAUDE_PLUGIN_ROOT}/skills/...`, resolve them using the path above."
  }'
