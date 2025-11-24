# Hooks App

TypeScript-based hook dispatcher for Claude Code quality enforcement.

## Session State Management

The hooks-app includes built-in session state tracking. See [SESSION.md](./SESSION.md) for detailed documentation.

**Quick Start:**

```bash
# Get session value
node dist/cli.js session get active_command /path/to/project

# Set session value
node dist/cli.js session set active_command /execute /path/to/project

# Append to array
node dist/cli.js session append edited_files main.ts /path/to/project

# Check contains (exit code 0 if true, 1 if false)
node dist/cli.js session contains file_extensions rs /path/to/project
```

**Automatic Tracking:**

The dispatcher automatically tracks:
- Active command/skill
- Edited files and extensions
- Session timestamps

This enables session-aware gates that can make decisions based on workflow context.

**Note:** Agent tracking is not included due to Claude Code limitations ([Issue #7881](https://github.com/anthropics/claude-code/issues/7881)).
