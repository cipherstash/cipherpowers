# Session State Management

The hooks-app includes session state tracking for hook workflows.

## Session State Structure

```typescript
interface SessionState {
  session_id: string;           // Unique session ID
  started_at: string;           // ISO 8601 timestamp
  active_command: string | null; // Current slash command (e.g., "/execute")
  active_skill: string | null;   // Current skill
  edited_files: string[];        // Files edited in session
  file_extensions: string[];     // File extensions edited (deduplicated)
  metadata: Record<string, any>; // Custom metadata
}
```

**Note:** Agent tracking is not implemented because Claude Code does not provide unique agent identifiers. This makes reliable tracking impossible when multiple agents of the same type run in parallel. See [Issue #7881](https://github.com/anthropics/claude-code/issues/7881).

## CLI Usage

### Get Value

```bash
node hooks-app/dist/cli.js session get active_command /path/to/project
```

### Set Value

```bash
node hooks-app/dist/cli.js session set active_command /execute /path/to/project
```

### Append to Array

```bash
node hooks-app/dist/cli.js session append edited_files main.ts /path/to/project
```

### Check Contains

```bash
node hooks-app/dist/cli.js session contains file_extensions rs /path/to/project
echo $?  # 0 if true, 1 if false
```

### Clear Session

```bash
node hooks-app/dist/cli.js session clear /path/to/project
```

## Programmatic Usage

### From TypeScript Gates

```typescript
import { Session } from '../hooks-app/dist/session';
import type { HookInput, GateResult } from '../hooks-app/dist/types';

export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);

  const activeCommand = await session.get('active_command');
  const hasRustFiles = await session.contains('file_extensions', 'rs');

  if (activeCommand === '/execute' && hasRustFiles) {
    return {
      decision: 'block',
      reason: 'Rust files edited during /execute'
    };
  }

  return { additionalContext: '' };
}
```

### From Bash Scripts

**Note:** Bash helper functions are NOT implemented in this task. They will be created in future work after session state integration is complete and tested.

**Future bash helper interface** (to be implemented separately):

```bash
# Source helpers (to be created in future task)
source "${CLAUDE_PLUGIN_ROOT}hooks/session-helpers.sh"

# Use session functions
session_set "active_command" "/execute" "$CWD"
COMMAND=$(session_get "active_command" "$CWD")

session_append "edited_files" "main.rs" "$CWD"

if session_contains "file_extensions" "rs" "$CWD"; then
  echo "Rust files edited"
fi
```

**Current workaround** (use CLI directly from bash):

```bash
# Set session value
node "${CLAUDE_PLUGIN_ROOT}hooks/hooks-app/dist/cli.js" session set active_command /execute "$CWD"

# Get session value
COMMAND=$(node "${CLAUDE_PLUGIN_ROOT}hooks/hooks-app/dist/cli.js" session get active_command "$CWD")

# Check contains
if node "${CLAUDE_PLUGIN_ROOT}hooks/hooks-app/dist/cli.js" session contains file_extensions rs "$CWD"; then
  echo "Rust files edited"
fi
```

## Automatic Session Tracking

The dispatcher automatically tracks session state on hook events:

- **SlashCommandStart**: Sets `active_command`
- **SlashCommandEnd**: Clears `active_command`
- **SkillStart**: Sets `active_skill`
- **SkillEnd**: Clears `active_skill`
- **PostToolUse**: Appends to `edited_files` and `file_extensions`

**Note:** SubagentStart/SubagentStop are NOT tracked because Claude Code does not provide unique agent identifiers ([Issue #7881](https://github.com/anthropics/claude-code/issues/7881)). This makes reliable agent instance tracking impossible when multiple agents of the same type run in parallel.

## Session Lifecycle

**When to Clear Session State:**

Session state should be manually cleared when:
1. **Switching workflows:** Moving from `/execute` to `/plan` or other major command changes
2. **Starting fresh:** Beginning a new feature or task that doesn't depend on previous session
3. **On explicit user request:** When user wants to reset session tracking
4. **After project switch:** When activating a different project (different CWD)

**Cleanup Strategy:**

```bash
# Clear session when workflow completes
node hooks-app/dist/cli.js session clear /path/to/project

# Or clear on SlashCommandEnd if needed (add to dispatcher logic)
```

**Important:** Session state persists across hook invocations by design. The hooks do NOT auto-clear session state. Cleanup is user-managed or can be automated via custom gates.

**Best Practices:**
- Session state is per-project (stored in .claude/session/ relative to CWD)
- Each CWD has independent session state (enables monorepo isolation - different projects don't interfere)
- Long-running workflows (like `/execute`) accumulate state throughout execution
- Clearing too frequently loses valuable context; clearing too rarely causes stale data
- Consider session timeout/expiry for long-lived projects (future enhancement)

## Storage Location

Session state is stored in:
```
.claude/session/state.json
```

Relative to the project directory (CWD from hook input).

## Atomic Updates

Session state uses atomic file writes (write to temp, then rename) to prevent corruption from concurrent updates or interrupted writes.

**Performance Characteristics:**
- File I/O overhead: ~1-5ms per operation
- Acceptable for hook events (PostToolUse, SlashCommand, Skill)
- Atomic writes prevent corruption but require temp file creation
- State file grows with edited_files array (typically <100KB)

## Error Handling

Session state updates are best-effort. If session updates fail, the hook continues execution.

**Structured Error Logging:**

Errors are logged to stderr in JSON format:
```json
{
  "error_type": "ENOENT",
  "error_message": "Permission denied",
  "hook_event": "PostToolUse",
  "cwd": "/path/to/project",
  "timestamp": "2025-11-23T14:30:45.123Z"
}
```

This structured format enables:
- Easier debugging with log aggregation tools
- Filtering by error_type or hook_event
- Correlation with timestamps for incident analysis
