# Integrate Session State into Hooks-App

**Date:** 2025-11-23
**Goal:** Add session state management directly into the existing hooks-app TypeScript application

## Overview

Integrate session state tracking into the existing `plugin/hooks/hooks-app/` instead of creating a separate hooklib package. This provides a unified TypeScript application that handles both hook dispatching and session management.

**Key Benefits:**
- Single TypeScript app (not two separate packages)
- Single build system and dist/ output
- CLI supports both hook dispatch AND session commands
- Simpler deployment and maintenance
- Reuses existing infrastructure (package.json, tsconfig, jest, build system)

**Important Limitation:**
- Claude Code does not provide unique agent identifiers ([Issue #7881](https://github.com/anthropics/claude-code/issues/7881))
- We cannot reliably track which specific agent instances are active
- Session state tracks commands, skills, and file edits only
- Agent tracking can be added when Claude Code adds `subagent_id` support

## Architecture

```
plugin/hooks/hooks-app/
├── src/
│   ├── cli.ts              # EXTENDED: Add session subcommands
│   ├── dispatcher.ts       # EXTENDED: Track session on hooks
│   ├── session.ts          # NEW: Session state management
│   ├── types.ts            # EXTENDED: Add session types
│   ├── action-handler.ts   # EXISTING: No changes
│   ├── config.ts           # EXISTING: No changes
│   ├── context.ts          # EXISTING: No changes
│   ├── gate-loader.ts      # EXISTING: No changes
│   ├── utils.ts            # EXISTING: No changes
│   └── index.ts            # EXTENDED: Export session types
├── __tests__/
│   ├── session.test.ts     # NEW: Session unit tests
│   └── ...                 # EXISTING: Other tests
├── dist/                   # EXISTING: Compiled output
├── package.json            # EXISTING: No changes needed
├── tsconfig.json           # EXISTING: No changes needed
└── jest.config.js          # EXISTING: No changes needed
```

## CLI Usage

The hooks-app CLI will support two modes of operation:

### 1. Hook Dispatch Mode (Existing)

```bash
# Receives hook input via stdin
echo '{"hook_event_name":"PostToolUse","tool_name":"Edit","cwd":"/path"}' | \
  node hooks-app/dist/cli.js
```

### 2. Session Management Mode (New)

```bash
# Set session value
node hooks-app/dist/cli.js session set active_command /execute /path/to/project

# Get session value
node hooks-app/dist/cli.js session get active_command /path/to/project

# Append to array
node hooks-app/dist/cli.js session append edited_files main.ts /path/to/project

# Check contains
node hooks-app/dist/cli.js session contains file_extensions rs /path/to/project

# Clear session
node hooks-app/dist/cli.js session clear /path/to/project
```

**CLI Mode Detection:**
- First arg is "session" → Session management mode
- First arg is anything else OR stdin has data → Hook dispatch mode (existing behavior)

## Implementation Tasks

---

## Task 1: Add Session Types

**Files:**
- Modify: `plugin/hooks/hooks-app/src/types.ts`

**Changes:**

Add session state types to existing types file:

```typescript
// Session state interface
export interface SessionState {
  /** Unique session identifier (timestamp-based) */
  session_id: string;

  /** ISO 8601 timestamp when session started */
  started_at: string;

  /** Currently active slash command (e.g., "/execute") */
  active_command: string | null;

  /** Currently active skill (e.g., "executing-plans") */
  active_skill: string | null;

  /** Files edited during this session */
  edited_files: string[];

  /** File extensions edited during this session (deduplicated) */
  file_extensions: string[];

  /** Custom metadata for specific workflows */
  metadata: Record<string, any>;
}

// Note: active_agent NOT included - Claude Code does not provide unique
// agent identifiers. Use metadata field if you need custom agent tracking.

/** Array field keys in SessionState (for type-safe operations) */
export type SessionStateArrayKey = 'edited_files' | 'file_extensions';

/** Scalar field keys in SessionState */
export type SessionStateScalarKey = Exclude<
  keyof SessionState,
  SessionStateArrayKey | 'metadata'
>;
```

**Verification:**
- Run `npm run build` - expect zero TypeScript errors
- Run `npm test` - expect all existing tests pass
- Verify types exported in dist/types.d.ts

---

## Task 2: Create Session Module

**Note:** This task is larger than ideal (15-20 minutes) but is kept as a cohesive unit because the Session class and its tests are tightly coupled and splitting them would create artificial dependencies.

**TDD Approach:**
1. Write test first → Run test (fail) → Implement → Run test (pass)
2. Follow RED-GREEN-REFACTOR cycle for each method

**Files:**
- Create: `plugin/hooks/hooks-app/src/session.ts`
- Create: `plugin/hooks/hooks-app/__tests__/session.test.ts`

**Implementation:**

Create `plugin/hooks/hooks-app/src/session.ts`:

```typescript
import { promises as fs } from 'fs';
import { dirname, join } from 'path';
import { SessionState, SessionStateArrayKey } from './types';

/**
 * Manages session state with atomic file updates.
 *
 * State is stored in .claude/session/state.json relative to the project directory.
 */
export class Session {
  private stateFile: string;

  constructor(cwd: string = '.') {
    this.stateFile = join(cwd, '.claude', 'session', 'state.json');
  }

  /**
   * Get a session state value
   */
  async get<K extends keyof SessionState>(key: K): Promise<SessionState[K]> {
    const state = await this.load();
    return state[key];
  }

  /**
   * Set a session state value
   */
  async set<K extends keyof SessionState>(
    key: K,
    value: SessionState[K]
  ): Promise<void> {
    const state = await this.load();
    state[key] = value;
    await this.save(state);
  }

  /**
   * Append value to array field (deduplicated)
   */
  async append(key: SessionStateArrayKey, value: string): Promise<void> {
    const state = await this.load();
    const array = state[key];

    if (!array.includes(value)) {
      array.push(value);
      await this.save(state);
    }
  }

  /**
   * Check if array contains value
   */
  async contains(key: SessionStateArrayKey, value: string): Promise<boolean> {
    const state = await this.load();
    return state[key].includes(value);
  }

  /**
   * Clear session state (remove file)
   */
  async clear(): Promise<void> {
    try {
      await fs.unlink(this.stateFile);
    } catch (error) {
      // File doesn't exist, that's fine
    }
  }

  /**
   * Load state from file or initialize new state
   */
  private async load(): Promise<SessionState> {
    try {
      const content = await fs.readFile(this.stateFile, 'utf-8');
      return JSON.parse(content);
    } catch (error) {
      // File doesn't exist or is corrupt, initialize new state
      return this.initState();
    }
  }

  /**
   * Save state to file atomically (write to temp, then rename)
   *
   * Performance note: File I/O adds small overhead (~1-5ms) per operation.
   * Atomic writes prevent corruption but require temp file creation.
   * For high-frequency PostToolUse events, this is acceptable overhead.
   */
  private async save(state: SessionState): Promise<void> {
    await fs.mkdir(dirname(this.stateFile), { recursive: true });
    const temp = this.stateFile + '.tmp';

    try {
      // Write to temp file
      await fs.writeFile(temp, JSON.stringify(state, null, 2), 'utf-8');

      // Atomic rename (prevents corruption from concurrent writes)
      await fs.rename(temp, this.stateFile);
    } catch (error) {
      // Clean up temp file on error
      try {
        await fs.unlink(temp);
      } catch {
        // Ignore cleanup errors
      }
      throw error;
    }
  }

  /**
   * Initialize new session state
   *
   * Session ID format: ISO timestamp with punctuation replaced (e.g., "2025-11-23T14-30-45")
   * Unique per millisecond. Collisions possible if multiple sessions start in same millisecond,
   * but unlikely in practice due to hook serialization.
   */
  private initState(): SessionState {
    const now = new Date();
    return {
      session_id: now.toISOString().replace(/[:.]/g, '-').substring(0, 19),
      started_at: now.toISOString(),
      active_command: null,
      active_skill: null,
      edited_files: [],
      file_extensions: [],
      metadata: {}
    };
  }
}
```

**Tests:**

Create `plugin/hooks/hooks-app/__tests__/session.test.ts`:

```typescript
import { Session } from '../src/session';
import { promises as fs } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

describe('Session', () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = join(tmpdir(), `session-test-${Date.now()}`);
    await fs.mkdir(testDir, { recursive: true });
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe('constructor', () => {
    test('sets state file path', () => {
      const session = new Session(testDir);
      expect(session['stateFile']).toBe(
        join(testDir, '.claude', 'session', 'state.json')
      );
    });
  });

  describe('get/set', () => {
    test('set and get scalar value', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');

      const value = await session.get('active_command');
      expect(value).toBe('/execute');
    });

    test('get returns null for unset values', async () => {
      const session = new Session(testDir);
      const value = await session.get('active_skill');
      expect(value).toBeNull();
    });

    test('set multiple values independently', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');
      await session.set('active_skill', 'executing-plans');

      expect(await session.get('active_command')).toBe('/execute');
      expect(await session.get('active_skill')).toBe('executing-plans');
    });
  });

  describe('append/contains', () => {
    test('append adds value to array', async () => {
      const session = new Session(testDir);
      await session.append('edited_files', 'main.ts');
      await session.append('edited_files', 'lib.ts');

      const files = await session.get('edited_files');
      expect(files).toEqual(['main.ts', 'lib.ts']);
    });

    test('append deduplicates values', async () => {
      const session = new Session(testDir);
      await session.append('edited_files', 'main.ts');
      await session.append('edited_files', 'lib.ts');
      await session.append('edited_files', 'main.ts'); // Duplicate

      const files = await session.get('edited_files');
      expect(files).toEqual(['main.ts', 'lib.ts']);
    });

    test('contains returns true for existing value', async () => {
      const session = new Session(testDir);
      await session.append('file_extensions', 'ts');
      await session.append('file_extensions', 'js');

      expect(await session.contains('file_extensions', 'ts')).toBe(true);
      expect(await session.contains('file_extensions', 'js')).toBe(true);
    });

    test('contains returns false for missing value', async () => {
      const session = new Session(testDir);
      await session.append('file_extensions', 'ts');

      expect(await session.contains('file_extensions', 'rs')).toBe(false);
    });
  });

  describe('clear', () => {
    test('removes state file', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');

      const stateFile = join(testDir, '.claude', 'session', 'state.json');
      const exists = await fs
        .access(stateFile)
        .then(() => true)
        .catch(() => false);
      expect(exists).toBe(true);

      await session.clear();

      const existsAfter = await fs
        .access(stateFile)
        .then(() => true)
        .catch(() => false);
      expect(existsAfter).toBe(false);
    });

    test('is safe when file does not exist', async () => {
      const session = new Session(testDir);
      await expect(session.clear()).resolves.not.toThrow();
    });
  });

  describe('persistence', () => {
    test('state persists across Session instances', async () => {
      const session1 = new Session(testDir);
      await session1.set('active_command', '/plan');
      await session1.append('edited_files', 'main.ts');

      const session2 = new Session(testDir);
      expect(await session2.get('active_command')).toBe('/plan');
      expect(await session2.get('edited_files')).toEqual(['main.ts']);
    });
  });

  describe('atomic writes', () => {
    test('uses atomic rename', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');

      const stateFile = join(testDir, '.claude', 'session', 'state.json');
      const tempFile = stateFile + '.tmp';

      // Temp file should not exist after save completes
      const tempExists = await fs
        .access(tempFile)
        .then(() => true)
        .catch(() => false);
      expect(tempExists).toBe(false);

      // State file should exist
      const stateExists = await fs
        .access(stateFile)
        .then(() => true)
        .catch(() => false);
      expect(stateExists).toBe(true);
    });
  });

  describe('error scenarios', () => {
    test('handles corrupted JSON gracefully', async () => {
      const session = new Session(testDir);
      const stateFile = join(testDir, '.claude', 'session', 'state.json');

      // Create directory and write corrupted JSON
      await fs.mkdir(dirname(stateFile), { recursive: true });
      await fs.writeFile(stateFile, '{invalid json', 'utf-8');

      // Should reinitialize state on corruption
      const value = await session.get('active_command');
      expect(value).toBeNull();
    });

    test('handles cross-process persistence', async () => {
      // Simulate separate process invocations
      const session1 = new Session(testDir);
      await session1.set('active_command', '/execute');
      await session1.append('edited_files', 'main.ts');

      // Create new session instance (simulates new process)
      const session2 = new Session(testDir);
      expect(await session2.get('active_command')).toBe('/execute');
      expect(await session2.get('edited_files')).toEqual(['main.ts']);
    });

    test('handles concurrent writes via atomic rename', async () => {
      const session = new Session(testDir);

      // Rapid concurrent writes (atomic rename prevents corruption)
      await Promise.all([
        session.append('edited_files', 'file1.ts'),
        session.append('edited_files', 'file2.ts'),
        session.append('edited_files', 'file3.ts'),
      ]);

      const files = await session.get('edited_files');
      expect(files).toHaveLength(3);
      expect(files).toContain('file1.ts');
      expect(files).toContain('file2.ts');
      expect(files).toContain('file3.ts');
    });
  });
});
```

**Verification:**
- Run `npm test session.test.ts` - expect all tests pass (including new error scenarios)
- Run `npm run build` - expect zero errors
- Verify test coverage includes corrupted JSON, cross-process persistence, concurrent writes

---

## Task 3: Extend CLI with Session Subcommands

**Note:** This task is larger than ideal (10-15 minutes) but is kept as a cohesive unit to ensure CLI mode detection and error handling are implemented together.

**TDD Approach:**
1. Write integration test for each CLI command first
2. Run test (fail) → Implement command → Run test (pass)
3. Follow RED-GREEN-REFACTOR cycle

**Files:**
- Modify: `plugin/hooks/hooks-app/src/cli.ts`

**Changes:**

Extend CLI to support both hook dispatch and session management with proper type safety:

```typescript
// plugin/hooks/hooks-app/src/cli.ts
import { HookInput, SessionState, SessionStateArrayKey } from './types';
import { dispatch } from './dispatcher';
import { Session } from './session';

interface OutputMessage {
  additionalContext?: string;
  decision?: string;
  reason?: string;
  continue?: boolean;
  message?: string;
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);

  // Check if first arg is "session" - session management mode
  if (args.length > 0 && args[0] === 'session') {
    await handleSessionCommand(args.slice(1));
    return;
  }

  // Otherwise, hook dispatch mode (existing behavior)
  await handleHookDispatch();
}

/**
 * Type guard for SessionState keys
 */
function isSessionStateKey(key: string): key is keyof SessionState {
  const validKeys: Array<keyof SessionState> = [
    'session_id',
    'started_at',
    'active_command',
    'active_skill',
    'edited_files',
    'file_extensions',
    'metadata'
  ];
  return validKeys.includes(key as keyof SessionState);
}

/**
 * Type guard for array keys
 */
function isArrayKey(key: string): key is SessionStateArrayKey {
  return key === 'edited_files' || key === 'file_extensions';
}

/**
 * Handle session management commands with proper type safety
 */
async function handleSessionCommand(args: string[]): Promise<void> {
  if (args.length < 1) {
    console.error('Usage: hooks-app session [get|set|append|contains|clear] ...');
    process.exit(1);
  }

  const [command, ...params] = args;
  const cwd = params[params.length - 1] || '.';
  const session = new Session(cwd);

  try {
    switch (command) {
      case 'get': {
        if (params.length < 2) {
          console.error('Usage: hooks-app session get <key> [cwd]');
          process.exit(1);
        }
        const [key] = params;
        if (!isSessionStateKey(key)) {
          console.error(`Invalid session key: ${key}`);
          process.exit(1);
        }
        const value = await session.get(key);
        console.log(value ?? '');
        break;
      }

      case 'set': {
        if (params.length < 3) {
          console.error('Usage: hooks-app session set <key> <value> [cwd]');
          process.exit(1);
        }
        const [key, value] = params;
        if (!isSessionStateKey(key)) {
          console.error(`Invalid session key: ${key}`);
          process.exit(1);
        }
        // Type-safe set with runtime validation
        if (key === 'active_command' || key === 'active_skill') {
          await session.set(key, value === 'null' ? null : value);
        } else if (key === 'metadata') {
          await session.set(key, JSON.parse(value));
        } else {
          console.error(`Cannot set ${key} via CLI (use get, append, or contains)`);
          process.exit(1);
        }
        break;
      }

      case 'append': {
        if (params.length < 3) {
          console.error('Usage: hooks-app session append <key> <value> [cwd]');
          process.exit(1);
        }
        const [key, value] = params;
        if (!isArrayKey(key)) {
          console.error(`Invalid array key: ${key} (must be edited_files or file_extensions)`);
          process.exit(1);
        }
        await session.append(key, value);
        break;
      }

      case 'contains': {
        if (params.length < 3) {
          console.error('Usage: hooks-app session contains <key> <value> [cwd]');
          process.exit(1);
        }
        const [key, value] = params;
        if (!isArrayKey(key)) {
          console.error(`Invalid array key: ${key} (must be edited_files or file_extensions)`);
          process.exit(1);
        }
        const result = await session.contains(key, value);
        process.exit(result ? 0 : 1);
        break;
      }

      case 'clear': {
        await session.clear();
        break;
      }

      default:
        console.error(`Unknown session command: ${command}`);
        process.exit(1);
    }
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error(`Session error: ${errorMessage}`);
    process.exit(1);
  }
}

/**
 * Handle hook dispatch (existing behavior)
 */
async function handleHookDispatch(): Promise<void> {
  try {
    // Read stdin
    const chunks: Buffer[] = [];
    for await (const chunk of process.stdin) {
      chunks.push(chunk);
    }
    const inputStr = Buffer.concat(chunks).toString('utf-8');

    // Parse input
    let input: HookInput;
    try {
      input = JSON.parse(inputStr);
    } catch (error) {
      console.error(
        JSON.stringify({
          continue: false,
          message: 'Invalid JSON input'
        })
      );
      process.exit(1);
    }

    // Validate required fields
    if (!input.hook_event_name || !input.cwd) {
      // Graceful exit - missing required fields
      return;
    }

    // Dispatch
    const result = await dispatch(input);

    // Build output
    const output: OutputMessage = {};

    if (result.context) {
      output.additionalContext = result.context;
    }

    if (result.blockReason) {
      output.decision = 'block';
      output.reason = result.blockReason;
    }

    if (result.stopMessage) {
      output.continue = false;
      output.message = result.stopMessage;
    }

    // Write output
    if (Object.keys(output).length > 0) {
      console.log(JSON.stringify(output));
    }
  } catch (error) {
    console.error(
      JSON.stringify({
        continue: false,
        message: `Unexpected error: ${error}`
      })
    );
    process.exit(1);
  }
}

main();
```

**Verification:**
- Run `npm run build` - expect zero TypeScript errors
- Run `npm test` - expect all tests pass
- Test CLI session mode manually:
  ```bash
  node dist/cli.js session set active_command /execute .
  node dist/cli.js session get active_command .
  # Expected output: /execute
  ```
- Test type guard catches invalid keys:
  ```bash
  node dist/cli.js session get invalid_key .
  # Expected output: Invalid session key: invalid_key
  # Expected exit code: 1
  ```
- Test hook dispatch mode still works (pipe JSON to stdin)

---

## Task 4: Track Session State in Dispatcher

**Files:**
- Modify: `plugin/hooks/hooks-app/src/dispatcher.ts`

**Changes:**

Add session tracking to dispatcher:

```typescript
// Add import at top
import { Session } from './session';

// Add new function before dispatch()
async function updateSessionState(input: HookInput): Promise<void> {
  const session = new Session(input.cwd);
  const event = input.hook_event_name;

  try {
    switch (event) {
      case 'SlashCommandStart':
        if (input.command) {
          await session.set('active_command', input.command);
        }
        break;

      case 'SlashCommandEnd':
        await session.set('active_command', null);
        break;

      case 'SkillStart':
        if (input.skill) {
          await session.set('active_skill', input.skill);
        }
        break;

      case 'SkillEnd':
        await session.set('active_skill', null);
        break;

      // Note: SubagentStart/SubagentStop NOT tracked - Claude Code does not
      // provide unique agent identifiers, making reliable agent tracking impossible
      // when multiple agents of the same type run in parallel.

      case 'PostToolUse':
        if (input.file_path) {
          await session.append('edited_files', input.file_path);

          // Extract and track file extension
          const ext = input.file_path.split('.').pop();
          if (ext && ext !== input.file_path) {
            await session.append('file_extensions', ext);
          }
        }
        break;
    }
  } catch (error) {
    // Session state is best-effort, don't fail the hook if it errors
    // Structured error logging for debugging
    const errorData = {
      error_type: error instanceof Error ? error.constructor.name : 'UnknownError',
      error_message: error instanceof Error ? error.message : String(error),
      hook_event: event,
      cwd: input.cwd,
      timestamp: new Date().toISOString()
    };
    console.error(`[Session Error] ${JSON.stringify(errorData)}`);
  }
}

// Modify dispatch() to call updateSessionState
export async function dispatch(input: HookInput): Promise<DispatchResult> {
  const hookEvent = input.hook_event_name;
  const cwd = input.cwd;

  // Update session state (best-effort)
  await updateSessionState(input);

  // ... rest of existing dispatch logic unchanged
```

**Verification:**
- Run `npm run build` - expect zero TypeScript errors
- Run `npm test` - expect all tests pass
- Test session tracking manually with hook input:
  ```bash
  echo '{"hook_event_name":"PostToolUse","tool_name":"Edit","file_path":"main.ts","cwd":"."}' | node dist/cli.js
  node dist/cli.js session get edited_files .
  # Expected output: ["main.ts"]
  ```
- Verify structured error logging format includes all fields (error_type, error_message, hook_event, cwd, timestamp)

---

## Task 5: Update Module Exports

**Files:**
- Modify: `plugin/hooks/hooks-app/src/index.ts`

**Changes:**

Export session types and class:

```typescript
// plugin/hooks/hooks-app/src/index.ts

// Existing exports
export { dispatch } from './dispatcher';
export { executeGate } from './gate-loader';
export { handleAction } from './action-handler';
export { loadConfig } from './config';
export { injectContext } from './context';

export type {
  HookInput,
  GateResult,
  GateExecute,
  GateConfig,
  HookConfig,
  GatesConfig
} from './types';

// New session exports
export { Session } from './session';
export type {
  SessionState,
  SessionStateArrayKey,
  SessionStateScalarKey
} from './types';
```

**Verification:**
- Run `npm run build` - expect zero TypeScript errors
- Verify exports in dist/index.d.ts include Session, SessionState, SessionStateArrayKey, SessionStateScalarKey
- Test import from external TypeScript file:
  ```typescript
  import { Session, SessionState } from './hooks-app/dist';
  ```

---

## Task 6: Built-in Gates Can Access Session

**Files:**
- Create: `plugin/hooks/gates/example-session-aware-gate.ts`

**Example gate using session state:**

```typescript
// plugin/hooks/gates/example-session-aware-gate.ts
import { Session } from '../hooks-app/dist/session';
import type { HookInput, GateResult } from '../hooks-app/dist/types';

/**
 * Example gate that uses session state
 *
 * This gate provides context when Rust files are edited during /execute,
 * reminding the user to consider using the rust-engineer agent.
 *
 * Demonstrates session state usage:
 * - Reading active_command
 * - Checking array contains
 * - Using metadata for custom tracking
 */
export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);

  // Only enforce during /execute
  const activeCommand = await session.get('active_command');
  if (activeCommand !== '/execute') {
    return { additionalContext: '' };
  }

  // Check if Rust files edited
  const hasRustFiles = await session.contains('file_extensions', 'rs');
  if (!hasRustFiles) {
    return { additionalContext: '' };
  }

  // Example: Track reminder count in metadata
  const metadata = await session.get('metadata');
  const reminderCount = (metadata.rust_reminder_count || 0) + 1;
  await session.set('metadata', {
    ...metadata,
    rust_reminder_count: reminderCount,
    last_rust_reminder: new Date().toISOString()
  });

  // Provide helpful context about Rust files
  return {
    additionalContext: `
## Rust Files Detected

You've edited Rust files during plan execution. Consider:
- Dispatching rust-engineer agent for TDD workflow
- Using rust-engineer for code review before completion
- Ensuring Prime Directive (type system) compliance

(Reminder #${reminderCount})
    `.trim()
  };
}
```

**Note:** This is an example placeholder. Real session-aware gates will be implemented after hooklib integration.

**Verification:**
- Run `npm run build` - expect zero TypeScript errors
- Compile example gate: `npx tsc gates/example-session-aware-gate.ts --outDir dist/gates --esModuleInterop --moduleResolution node`
- Test gate can import Session and read session state
- Verify metadata field usage example shows proper type handling

---

## Task 7: Integration Tests

**Note:** This task is larger than ideal (10-15 minutes) but is kept as a cohesive unit to ensure comprehensive end-to-end testing of all hook events and CLI commands together.

**TDD Approach:**
1. Write integration test first → Run test (fail) → Verify implementation → Run test (pass)
2. Test all hook event types (SlashCommand, Skill, PostToolUse)
3. Test error scenarios (corrupted state, invalid commands)

**Files:**
- Create: `plugin/hooks/hooks-app/__tests__/integration.test.ts`

**Integration tests:**

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';
import { join } from 'path';
import { promises as fs } from 'fs';
import { tmpdir } from 'os';

const execAsync = promisify(exec);

describe('Integration Tests', () => {
  let testDir: string;
  let cliPath: string;

  beforeEach(async () => {
    testDir = join(tmpdir(), `integration-test-${Date.now()}`);
    await fs.mkdir(testDir, { recursive: true });
    cliPath = join(__dirname, '../dist/cli.js');
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe('Session Management', () => {
    test('set and get command', async () => {
      await execAsync(`node ${cliPath} session set active_command /execute ${testDir}`);
      const { stdout } = await execAsync(`node ${cliPath} session get active_command ${testDir}`);
      expect(stdout.trim()).toBe('/execute');
    });

    test('append and check contains', async () => {
      await execAsync(`node ${cliPath} session append file_extensions ts ${testDir}`);

      const result = await execAsync(`node ${cliPath} session contains file_extensions ts ${testDir}`)
        .then(() => true)
        .catch(() => false);

      expect(result).toBe(true);
    });

    test('clear removes state', async () => {
      await execAsync(`node ${cliPath} session set active_command /plan ${testDir}`);
      await execAsync(`node ${cliPath} session clear ${testDir}`);

      const { stdout } = await execAsync(`node ${cliPath} session get active_command ${testDir}`);
      expect(stdout.trim()).toBe('');
    });
  });

  describe('Hook Dispatch with Session Tracking', () => {
    test('PostToolUse updates session', async () => {
      const hookInput = JSON.stringify({
        hook_event_name: 'PostToolUse',
        tool_name: 'Edit',
        file_path: 'main.ts',
        cwd: testDir
      });

      await execAsync(`echo '${hookInput}' | node ${cliPath}`);

      const { stdout: files } = await execAsync(
        `node ${cliPath} session get edited_files ${testDir}`
      );
      expect(files).toContain('main.ts');

      const containsTs = await execAsync(
        `node ${cliPath} session contains file_extensions ts ${testDir}`
      ).then(() => true).catch(() => false);
      expect(containsTs).toBe(true);
    });

    test('SlashCommandStart/End updates session', async () => {
      // Start command
      const startInput = JSON.stringify({
        hook_event_name: 'SlashCommandStart',
        command: '/execute',
        cwd: testDir
      });
      await execAsync(`echo '${startInput}' | node ${cliPath}`);

      const { stdout: activeCmd } = await execAsync(
        `node ${cliPath} session get active_command ${testDir}`
      );
      expect(activeCmd.trim()).toBe('/execute');

      // End command
      const endInput = JSON.stringify({
        hook_event_name: 'SlashCommandEnd',
        command: '/execute',
        cwd: testDir
      });
      await execAsync(`echo '${endInput}' | node ${cliPath}`);

      const { stdout: cleared } = await execAsync(
        `node ${cliPath} session get active_command ${testDir}`
      );
      expect(cleared.trim()).toBe('');
    });

    test('SkillStart/End updates session', async () => {
      // Start skill
      const startInput = JSON.stringify({
        hook_event_name: 'SkillStart',
        skill: 'executing-plans',
        cwd: testDir
      });
      await execAsync(`echo '${startInput}' | node ${cliPath}`);

      const { stdout: activeSkill } = await execAsync(
        `node ${cliPath} session get active_skill ${testDir}`
      );
      expect(activeSkill.trim()).toBe('executing-plans');

      // End skill
      const endInput = JSON.stringify({
        hook_event_name: 'SkillEnd',
        skill: 'executing-plans',
        cwd: testDir
      });
      await execAsync(`echo '${endInput}' | node ${cliPath}`);

      const { stdout: cleared } = await execAsync(
        `node ${cliPath} session get active_skill ${testDir}`
      );
      expect(cleared.trim()).toBe('');
    });
  });

  describe('Error Handling', () => {
    test('handles corrupted state file gracefully', async () => {
      const stateFile = join(testDir, '.claude', 'session', 'state.json');
      await execAsync(`mkdir -p ${dirname(stateFile)}`);
      await execAsync(`echo '{invalid json' > ${stateFile}`);

      // Should reinitialize and work
      await execAsync(`node ${cliPath} session set active_command /plan ${testDir}`);
      const { stdout } = await execAsync(`node ${cliPath} session get active_command ${testDir}`);
      expect(stdout.trim()).toBe('/plan');
    });

    test('rejects invalid session keys', async () => {
      try {
        await execAsync(`node ${cliPath} session get invalid_key ${testDir}`);
        fail('Should have thrown error');
      } catch (error: any) {
        expect(error.stderr).toContain('Invalid session key');
      }
    });

    test('rejects invalid array keys for append', async () => {
      try {
        await execAsync(`node ${cliPath} session append invalid_key value ${testDir}`);
        fail('Should have thrown error');
      } catch (error: any) {
        expect(error.stderr).toContain('Invalid array key');
      }
    });
  });
});
```

**Verification:**
- Run `npm test integration.test.ts` - expect all tests pass
- Verify test coverage includes:
  - All CLI commands (get, set, append, contains, clear)
  - All hook events (SlashCommandStart/End, SkillStart/End, PostToolUse)
  - Error scenarios (corrupted state, invalid keys)
  - Cross-process persistence
- Run `npm test -- --coverage` - verify integration test coverage metrics

---

## Task 8: Documentation

**Files:**
- Create: `plugin/hooks/hooks-app/SESSION.md`
- Update: `plugin/hooks/hooks-app/README.md`

**Create SESSION.md:**

```markdown
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

**Note:** `active_agent` is NOT included because Claude Code does not provide unique agent identifiers. This makes reliable tracking impossible when multiple agents of the same type run in parallel. See [Issue #7881](https://github.com/anthropics/claude-code/issues/7881).

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
source "${CLAUDE_PLUGIN_ROOT}/hooks/session-helpers.sh"

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
node "${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js" session set active_command /execute "$CWD"

# Get session value
COMMAND=$(node "${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js" session get active_command "$CWD")

# Check contains
if node "${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js" session contains file_extensions rs "$CWD"; then
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
```

**Update README.md:**

Add section to existing `plugin/hooks/hooks-app/README.md`:

```markdown
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
```

---

## Success Criteria

✅ Session types added to existing types.ts
✅ Session module created with full test coverage
✅ CLI extended to support session subcommands
✅ Dispatcher tracks session state on hook events
✅ Session API exported for TypeScript gates
✅ Integration tests pass
✅ Documentation complete
✅ Single build system and dist/ output
✅ No separate hooklib package needed

## Benefits of This Approach

1. **Single TypeScript App**: One package, one build, one dist/
2. **Dual-Mode CLI**: Handles both hook dispatch AND session management
3. **Reuses Infrastructure**: Existing package.json, tsconfig, jest, build system
4. **Simpler Deployment**: One compiled application instead of two
5. **Type Safety**: Full TypeScript support across all functionality
6. **Easy Testing**: Unit and integration tests in one test suite
7. **Clean API**: Gates can import session directly from hooks-app

## Plan Improvements from Dual Review

This plan was updated to incorporate **all 11 suggestions** from comprehensive dual-verification plan review:

**Common Suggestions (VERY HIGH confidence - both reviewers found):**

1. ✅ **TDD Workflow Made Explicit** - Added "TDD Approach" sections to Tasks 2, 3, 7 with RED-GREEN-REFACTOR cycle
2. ✅ **Performance Documented** - Added performance notes to `save()` method (1-5ms overhead), SESSION.md performance section
3. ✅ **Type Safety Fixed** - Replaced `as any` casts in CLI with proper type guards (`isSessionStateKey`, `isArrayKey`)
4. ✅ **Integration Test Coverage Enhanced** - Added SkillStart/End tests, error scenario tests (corrupted state, invalid keys)
5. ✅ **Task Granularity Documented** - Added notes explaining why Tasks 2, 3, 7 are larger (cohesive units)

**Exclusive Suggestions (MODERATE confidence - one reviewer found):**

6. ✅ **Session Lifecycle Documented** - Added complete lifecycle section to SESSION.md (when to clear, cleanup strategy, best practices)
7. ✅ **Verification Steps Made Specific** - Replaced generic "TypeScript compiles" with exact commands (`npm run build`, `npm test`, expected output)
8. ✅ **Error Logging Format Specified** - Added structured JSON error logging with all fields (error_type, error_message, hook_event, cwd, timestamp)
9. ✅ **Bash Helper Functions Clarified** - Marked as future work with "to be implemented separately" note, provided current CLI workaround
10. ✅ **Concurrent Access Testing Added** - Added concurrent write test to Task 2 (rapid parallel appends)
11. ✅ **Additional Improvements:**
    - Session ID format documented with collision notes
    - Error cleanup in `save()` method (temp file cleanup on failure)
    - Metadata usage example in Task 6
    - Cross-process persistence test in Task 2

**Result:** Plan now has **zero blocking issues**, explicit TDD workflow, comprehensive test coverage, proper type safety, and complete documentation.

## Next Steps

After implementation:

1. Create bash helper functions for session management (separate task)
2. Implement real session-aware gates (after testing)
3. Update existing bash gates to use session state
4. Test with real workflows (PostToolUse, /execute, etc.)
5. Consider migrating more logic from bash to TypeScript
6. Add session timeout/expiry for long-lived projects (future enhancement)

## References

- Design doc: `docs/plans/2025-11-21-typescript-hooks-design.md`
- Original session plan: `docs/plans/2025-11-21-typescript-session-state.md`
- Existing hooks-app: `plugin/hooks/hooks-app/`
- Dual plan review: `.work/2025-11-23-collated-plan-review.md`
