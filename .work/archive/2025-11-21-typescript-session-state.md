# TypeScript Session State Module Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use cipherpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement session state tracking for CipherPowers hook system using TypeScript with compiled JavaScript distribution

**Architecture:** Create hooklib TypeScript module with SessionState class for atomic state management. Ship compiled JavaScript for fast runtime execution. Provide CLI interface for bash interop and programmatic API for TypeScript gates.

**Tech Stack:** TypeScript 5.x, Node.js (bundled with Claude Code), Jest for testing

---

## Prerequisites

**Existing files:**
- `/Users/tobyhede/src/cipherpowers/plugin/hooks/dispatcher.sh` - Hook dispatcher
- `/Users/tobyhede/src/cipherpowers/plugin/hooks/shared-functions.sh` - Bash utilities
- `/Users/tobyhede/src/cipherpowers/plugin/hooks/gates/` - Gate scripts directory

**Design references:**
- `/Users/tobyhede/psrc/battlespace/.work/reviews/typescript-vs-python-for-hooks.md`
- `/Users/tobyhede/psrc/battlespace/.work/reviews/session-state-tracking-design.md` (Python design, adapt to TypeScript)

---

## Task 1: Project Setup

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/package.json`
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/tsconfig.json`
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/.gitignore`

**Step 1: Create package.json**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/package.json`:

```json
{
  "name": "@cipherpowers/hooklib",
  "version": "1.0.0",
  "description": "TypeScript utilities for CipherPowers hook system",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "watch": "tsc --watch",
    "test": "jest",
    "test:watch": "jest --watch",
    "clean": "rm -rf dist",
    "prepublish": "npm run clean && npm run build"
  },
  "keywords": ["hooks", "session-state", "cipherpowers"],
  "author": "CipherPowers",
  "license": "MIT",
  "devDependencies": {
    "@types/jest": "^29.5.0",
    "@types/node": "^20.0.0",
    "jest": "^29.7.0",
    "ts-jest": "^29.1.0",
    "typescript": "^5.3.0"
  }
}
```

**Step 2: Create tsconfig.json**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./hooklib/src",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "moduleResolution": "node"
  },
  "include": ["hooklib/src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}
```

**Step 3: Create .gitignore**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/.gitignore`:

```
node_modules/
dist/
*.log
.DS_Store
```

**Step 4: Install dependencies**

Run:
```bash
cd /Users/tobyhede/src/cipherpowers/plugin/hooks
npm install
```

Expected: Dependencies installed, `node_modules/` created, `package-lock.json` generated

**Step 5: Commit**

```bash
git add package.json tsconfig.json .gitignore
git commit -m "feat(hooks): add TypeScript project setup"
```

---

## Task 2: SessionState Types

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/types.ts`

**Step 1: Write test for SessionState interface**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/types.test.ts`:

```typescript
import { SessionState } from '../types';

describe('SessionState', () => {
  test('interface has required fields', () => {
    const state: SessionState = {
      session_id: 'test-123',
      started_at: '2025-11-21T00:00:00Z',
      active_command: null,
      active_skill: null,
      active_agent: null,
      edited_files: [],
      file_extensions: [],
      metadata: {}
    };

    expect(state.session_id).toBe('test-123');
    expect(state.active_command).toBeNull();
    expect(state.edited_files).toEqual([]);
  });
});
```

**Step 2: Run test to verify it fails**

Run:
```bash
cd /Users/tobyhede/src/cipherpowers/plugin/hooks
npm test
```

Expected: FAIL with "Cannot find module '../types'"

**Step 3: Create types file**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/types.ts`:

```typescript
/**
 * Session state interface for tracking hook execution context.
 */
export interface SessionState {
  /** Unique session identifier (timestamp-based) */
  session_id: string;

  /** ISO 8601 timestamp when session started */
  started_at: string;

  /** Currently active slash command (e.g., "/execute") */
  active_command: string | null;

  /** Currently active skill (e.g., "executing-plans") */
  active_skill: string | null;

  /** Currently active agent (e.g., "rust-engineer") */
  active_agent: string | null;

  /** Files edited during this session */
  edited_files: string[];

  /** File extensions edited during this session (deduplicated) */
  file_extensions: string[];

  /** Custom metadata for specific workflows */
  metadata: Record<string, any>;
}

/**
 * Hook input from Claude Code hook system.
 */
export interface HookInput {
  hook_event_name: string;
  cwd: string;
  tool_name?: string;
  file_path?: string;
  command?: string;
  skill?: string;
  agent_name?: string;
  subagent_name?: string;
  user_message?: string;
}

/**
 * Array field keys in SessionState (for type-safe append/contains operations).
 */
export type SessionStateArrayKey = 'edited_files' | 'file_extensions';

/**
 * Scalar field keys in SessionState.
 */
export type SessionStateScalarKey = Exclude<keyof SessionState, SessionStateArrayKey | 'metadata'>;
```

**Step 4: Run test to verify it passes**

Run:
```bash
npm test
```

Expected: PASS

**Step 5: Commit**

```bash
git add hooklib/src/types.ts hooklib/src/__tests__/types.test.ts
git commit -m "feat(hooks): add SessionState types"
```

---

## Task 3: SessionState Class - Core

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`

**Step 1: Write failing test for SessionState constructor**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`:

```typescript
import { Session } from '../session';
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

  test('constructor sets state file path', () => {
    const session = new Session(testDir);
    expect(session['stateFile']).toBe(join(testDir, '.claude', 'session', 'state.json'));
  });
});
```

**Step 2: Run test to verify it fails**

Run:
```bash
npm test
```

Expected: FAIL with "Cannot find module '../session'"

**Step 3: Create minimal Session class**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`:

```typescript
import { promises as fs } from 'fs';
import { dirname, join } from 'path';
import { SessionState } from './types';

/**
 * Manages session state with atomic file updates.
 */
export class Session {
  private stateFile: string;

  constructor(cwd: string = '.') {
    this.stateFile = join(cwd, '.claude', 'session', 'state.json');
  }
}
```

**Step 4: Run test to verify it passes**

Run:
```bash
npm test
```

Expected: PASS

**Step 5: Commit**

```bash
git add hooklib/src/session.ts hooklib/src/__tests__/session.test.ts
git commit -m "feat(hooks): add Session class constructor"
```

---

## Task 4: SessionState Class - Load/Save

**Files:**
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`

**Step 1: Write test for load() method**

Add to `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`:

```typescript
test('load() returns existing state', async () => {
  const session = new Session(testDir);
  const stateFile = join(testDir, '.claude', 'session', 'state.json');

  await fs.mkdir(dirname(stateFile), { recursive: true });
  await fs.writeFile(stateFile, JSON.stringify({
    session_id: 'test-123',
    started_at: '2025-11-21T00:00:00Z',
    active_command: '/execute',
    active_skill: null,
    active_agent: null,
    edited_files: [],
    file_extensions: [],
    metadata: {}
  }));

  const state = await session['load']();
  expect(state.session_id).toBe('test-123');
  expect(state.active_command).toBe('/execute');
});

test('load() initializes new state when file missing', async () => {
  const session = new Session(testDir);
  const state = await session['load']();

  expect(state.session_id).toBeTruthy();
  expect(state.started_at).toBeTruthy();
  expect(state.active_command).toBeNull();
  expect(state.edited_files).toEqual([]);
});
```

**Step 2: Run test to verify it fails**

Run:
```bash
npm test
```

Expected: FAIL with "session['load'] is not a function"

**Step 3: Implement load() method**

Add to `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`:

```typescript
private async load(): Promise<SessionState> {
  try {
    const content = await fs.readFile(this.stateFile, 'utf-8');
    return JSON.parse(content);
  } catch (error) {
    // File doesn't exist or is corrupt, initialize new state
    return this.initState();
  }
}

private initState(): SessionState {
  const now = new Date();
  return {
    session_id: now.toISOString().replace(/[:.]/g, '-').substring(0, 19),
    started_at: now.toISOString(),
    active_command: null,
    active_skill: null,
    active_agent: null,
    edited_files: [],
    file_extensions: [],
    metadata: {}
  };
}
```

**Step 4: Write test for save() method**

Add to test file:

```typescript
test('save() writes state atomically', async () => {
  const session = new Session(testDir);
  const state: SessionState = {
    session_id: 'atomic-123',
    started_at: '2025-11-21T00:00:00Z',
    active_command: '/plan',
    active_skill: null,
    active_agent: null,
    edited_files: ['main.ts'],
    file_extensions: ['ts'],
    metadata: {}
  };

  await session['save'](state);

  const stateFile = join(testDir, '.claude', 'session', 'state.json');
  const content = await fs.readFile(stateFile, 'utf-8');
  const loaded = JSON.parse(content);

  expect(loaded.session_id).toBe('atomic-123');
  expect(loaded.active_command).toBe('/plan');
  expect(loaded.edited_files).toEqual(['main.ts']);
});
```

**Step 5: Implement save() method**

Add to Session class:

```typescript
private async save(state: SessionState): Promise<void> {
  await fs.mkdir(dirname(this.stateFile), { recursive: true });
  const temp = this.stateFile + '.tmp';

  // Write to temp file
  await fs.writeFile(temp, JSON.stringify(state, null, 2), 'utf-8');

  // Atomic rename
  await fs.rename(temp, this.stateFile);
}
```

**Step 6: Run tests**

Run:
```bash
npm test
```

Expected: ALL PASS

**Step 7: Commit**

```bash
git add hooklib/src/session.ts hooklib/src/__tests__/session.test.ts
git commit -m "feat(hooks): add Session load/save with atomic writes"
```

---

## Task 5: SessionState Class - Public API

**Files:**
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`

**Step 1: Write test for get() method**

Add to test file:

```typescript
test('get() returns scalar value', async () => {
  const session = new Session(testDir);
  await session.set('active_command', '/execute');

  const value = await session.get('active_command');
  expect(value).toBe('/execute');
});

test('get() returns null for unset values', async () => {
  const session = new Session(testDir);
  const value = await session.get('active_skill');
  expect(value).toBeNull();
});
```

**Step 2: Run test**

Run: `npm test`

Expected: FAIL with "session.get is not a function"

**Step 3: Implement get() method**

Add to Session class:

```typescript
async get<K extends keyof SessionState>(key: K): Promise<SessionState[K]> {
  const state = await this.load();
  return state[key];
}
```

**Step 4: Write test for set() method**

Add to test file:

```typescript
test('set() updates scalar value', async () => {
  const session = new Session(testDir);
  await session.set('active_command', '/code-review');

  const value = await session.get('active_command');
  expect(value).toBe('/code-review');
});

test('set() updates multiple values independently', async () => {
  const session = new Session(testDir);
  await session.set('active_command', '/execute');
  await session.set('active_skill', 'executing-plans');

  expect(await session.get('active_command')).toBe('/execute');
  expect(await session.get('active_skill')).toBe('executing-plans');
});
```

**Step 5: Implement set() method**

Add to Session class:

```typescript
async set<K extends keyof SessionState>(
  key: K,
  value: SessionState[K]
): Promise<void> {
  const state = await this.load();
  state[key] = value;
  await this.save(state);
}
```

**Step 6: Run tests**

Run: `npm test`

Expected: ALL PASS

**Step 7: Commit**

```bash
git add hooklib/src/session.ts hooklib/src/__tests__/session.test.ts
git commit -m "feat(hooks): add Session get/set methods"
```

---

## Task 6: SessionState Class - Array Operations

**Files:**
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`

**Step 1: Write tests for append() method**

Add to test file:

```typescript
test('append() adds value to array', async () => {
  const session = new Session(testDir);
  await session.append('edited_files', 'main.ts');
  await session.append('edited_files', 'lib.ts');

  const files = await session.get('edited_files');
  expect(files).toEqual(['main.ts', 'lib.ts']);
});

test('append() deduplicates values', async () => {
  const session = new Session(testDir);
  await session.append('edited_files', 'main.ts');
  await session.append('edited_files', 'lib.ts');
  await session.append('edited_files', 'main.ts'); // Duplicate

  const files = await session.get('edited_files');
  expect(files).toEqual(['main.ts', 'lib.ts']);
});
```

**Step 2: Run test**

Run: `npm test`

Expected: FAIL

**Step 3: Implement append() method**

Add to Session class:

```typescript
import { SessionStateArrayKey } from './types';

async append(key: SessionStateArrayKey, value: string): Promise<void> {
  const state = await this.load();
  const array = state[key];

  if (!array.includes(value)) {
    array.push(value);
    await this.save(state);
  }
}
```

**Step 4: Write tests for contains() method**

Add to test file:

```typescript
test('contains() returns true for existing value', async () => {
  const session = new Session(testDir);
  await session.append('file_extensions', 'ts');
  await session.append('file_extensions', 'js');

  expect(await session.contains('file_extensions', 'ts')).toBe(true);
  expect(await session.contains('file_extensions', 'js')).toBe(true);
});

test('contains() returns false for missing value', async () => {
  const session = new Session(testDir);
  await session.append('file_extensions', 'ts');

  expect(await session.contains('file_extensions', 'rs')).toBe(false);
});
```

**Step 5: Implement contains() method**

Add to Session class:

```typescript
async contains(key: SessionStateArrayKey, value: string): Promise<boolean> {
  const state = await this.load();
  return state[key].includes(value);
}
```

**Step 6: Run tests**

Run: `npm test`

Expected: ALL PASS

**Step 7: Commit**

```bash
git add hooklib/src/session.ts hooklib/src/__tests__/session.test.ts
git commit -m "feat(hooks): add Session append/contains for arrays"
```

---

## Task 7: SessionState Class - Clear

**Files:**
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/session.ts`
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/session.test.ts`

**Step 1: Write test for clear() method**

Add to test file:

```typescript
test('clear() removes state file', async () => {
  const session = new Session(testDir);
  await session.set('active_command', '/execute');

  const stateFile = join(testDir, '.claude', 'session', 'state.json');
  expect(await fs.access(stateFile).then(() => true).catch(() => false)).toBe(true);

  await session.clear();

  expect(await fs.access(stateFile).then(() => true).catch(() => false)).toBe(false);
});

test('clear() is safe when file does not exist', async () => {
  const session = new Session(testDir);
  await expect(session.clear()).resolves.not.toThrow();
});
```

**Step 2: Run test**

Run: `npm test`

Expected: FAIL

**Step 3: Implement clear() method**

Add to Session class:

```typescript
async clear(): Promise<void> {
  try {
    await fs.unlink(this.stateFile);
  } catch (error) {
    // File doesn't exist, that's fine
  }
}
```

**Step 4: Run tests**

Run: `npm test`

Expected: ALL PASS

**Step 5: Commit**

```bash
git add hooklib/src/session.ts hooklib/src/__tests__/session.test.ts
git commit -m "feat(hooks): add Session clear method"
```

---

## Task 8: CLI Interface for Bash Interop

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/cli.ts`
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/cli.test.ts`

**Step 1: Write test for CLI get command**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/cli.test.ts`:

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';
import { join } from 'path';
import { promises as fs } from 'fs';
import { tmpdir } from 'os';

const execAsync = promisify(exec);

describe('CLI', () => {
  let testDir: string;
  let cliPath: string;

  beforeEach(async () => {
    testDir = join(tmpdir(), `cli-test-${Date.now()}`);
    await fs.mkdir(testDir, { recursive: true });

    // Will be compiled to dist/cli.js
    cliPath = join(__dirname, '../../dist/cli.js');
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  test('get command returns value', async () => {
    // Set up state
    const stateFile = join(testDir, '.claude', 'session', 'state.json');
    await fs.mkdir(join(testDir, '.claude', 'session'), { recursive: true });
    await fs.writeFile(stateFile, JSON.stringify({
      session_id: 'test-123',
      started_at: '2025-11-21T00:00:00Z',
      active_command: '/execute',
      active_skill: null,
      active_agent: null,
      edited_files: [],
      file_extensions: [],
      metadata: {}
    }));

    const { stdout } = await execAsync(`node ${cliPath} get active_command ${testDir}`);
    expect(stdout.trim()).toBe('/execute');
  });
});
```

**Step 2: Run test**

Run: `npm test`

Expected: FAIL (cli.ts doesn't exist)

**Step 3: Create CLI implementation**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/cli.ts`:

```typescript
#!/usr/bin/env node

import { Session } from './session';

async function main() {
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.error('Usage: session [get|set|append|contains|clear] ...');
    process.exit(1);
  }

  const [command, ...params] = args;
  const cwd = params[params.length - 1] || '.';
  const session = new Session(cwd);

  try {
    switch (command) {
      case 'get': {
        const [key] = params;
        const value = await session.get(key as any);
        console.log(value ?? '');
        break;
      }

      case 'set': {
        const [key, value] = params;
        await session.set(key as any, value);
        break;
      }

      case 'append': {
        const [key, value] = params;
        await session.append(key as any, value);
        break;
      }

      case 'contains': {
        const [key, value] = params;
        const result = await session.contains(key as any, value);
        process.exit(result ? 0 : 1);
        break;
      }

      case 'clear': {
        await session.clear();
        break;
      }

      default:
        console.error(`Unknown command: ${command}`);
        process.exit(1);
    }
  } catch (error) {
    console.error(`Error: ${error}`);
    process.exit(1);
  }
}

main();
```

**Step 4: Build and run test**

Run:
```bash
npm run build
npm test
```

Expected: PASS

**Step 5: Add shebang and make executable**

The CLI is already set up with `#!/usr/bin/env node` shebang.

After build, make executable:
```bash
chmod +x dist/cli.js
```

**Step 6: Test from bash**

Run:
```bash
cd /Users/tobyhede/src/cipherpowers/plugin/hooks
node dist/cli.js set active_command /execute /tmp/test-session
node dist/cli.js get active_command /tmp/test-session
```

Expected: Output `/execute`

**Step 7: Commit**

```bash
git add hooklib/src/cli.ts hooklib/src/__tests__/cli.test.ts
git commit -m "feat(hooks): add CLI interface for bash interop"
```

---

## Task 9: Module Exports

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/index.ts`
- Modify: `/Users/tobyhede/src/cipherpowers/plugin/hooks/package.json`

**Step 1: Create index.ts**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/index.ts`:

```typescript
/**
 * CipherPowers Hook System - TypeScript Library
 *
 * Provides session state management and utilities for hook scripts.
 */

export { Session } from './session';
export type { SessionState, HookInput, SessionStateArrayKey, SessionStateScalarKey } from './types';
```

**Step 2: Build**

Run:
```bash
npm run build
```

Expected: Compiled files in `dist/`

**Step 3: Verify exports**

Create test `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/src/__tests__/index.test.ts`:

```typescript
import * as hooklib from '../index';

test('exports Session class', () => {
  expect(hooklib.Session).toBeDefined();
  expect(typeof hooklib.Session).toBe('function');
});
```

Run: `npm test`

Expected: PASS

**Step 4: Commit**

```bash
git add hooklib/src/index.ts hooklib/src/__tests__/index.test.ts
git commit -m "feat(hooks): add module exports"
```

---

## Task 10: Jest Configuration

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/jest.config.js`

**Step 1: Create Jest config**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/jest.config.js`:

```javascript
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  roots: ['<rootDir>/hooklib/src'],
  testMatch: ['**/__tests__/**/*.test.ts'],
  collectCoverageFrom: [
    'hooklib/src/**/*.ts',
    '!hooklib/src/**/*.test.ts',
    '!hooklib/src/__tests__/**'
  ],
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80
    }
  }
};
```

**Step 2: Run tests with coverage**

Run:
```bash
npm test -- --coverage
```

Expected: All tests pass with >80% coverage

**Step 3: Commit**

```bash
git add jest.config.js
git commit -m "feat(hooks): add Jest configuration"
```

---

## Task 11: Documentation

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/README.md`

**Step 1: Create README**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/README.md`:

```markdown
# CipherPowers Hook System - TypeScript Library

Session state management for CipherPowers hook system.

## Installation

```bash
cd plugin/hooks
npm install
npm run build
```

## Usage

### From TypeScript

```typescript
import { Session } from './hooklib/dist';

const session = new Session('/path/to/project');

// Set values
await session.set('active_command', '/execute');
await session.set('active_skill', 'executing-plans');

// Get values
const command = await session.get('active_command');

// Array operations
await session.append('edited_files', 'main.ts');
await session.append('file_extensions', 'ts');

const hasRust = await session.contains('file_extensions', 'rs');

// Clear session
await session.clear();
```

### From Bash

```bash
# Set value
node hooklib/dist/cli.js set active_command /execute /path/to/project

# Get value
COMMAND=$(node hooklib/dist/cli.js get active_command /path/to/project)

# Append to array
node hooklib/dist/cli.js append edited_files main.ts /path/to/project

# Check contains
if node hooklib/dist/cli.js contains file_extensions rs /path/to/project; then
  echo "Rust files edited"
fi

# Clear session
node hooklib/dist/cli.js clear /path/to/project
```

## API

### Session Class

**Constructor:**
- `new Session(cwd?: string)` - Create session (default: current directory)

**Methods:**
- `async get<K>(key: K): Promise<SessionState[K]>` - Get state value
- `async set<K>(key: K, value: SessionState[K]): Promise<void>` - Set state value
- `async append(key: ArrayKey, value: string): Promise<void>` - Append to array (deduplicated)
- `async contains(key: ArrayKey, value: string): Promise<boolean>` - Check if array contains value
- `async clear(): Promise<void>` - Clear session state

### Types

```typescript
interface SessionState {
  session_id: string;
  started_at: string;
  active_command: string | null;
  active_skill: string | null;
  active_agent: string | null;
  edited_files: string[];
  file_extensions: string[];
  metadata: Record<string, any>;
}
```

## Testing

```bash
npm test              # Run tests
npm test -- --watch   # Watch mode
npm test -- --coverage # With coverage
```

## Building

```bash
npm run build   # Compile TypeScript
npm run watch   # Watch mode
npm run clean   # Remove dist/
```
```

**Step 2: Commit**

```bash
git add hooklib/README.md
git commit -m "docs(hooks): add hooklib README"
```

---

## Task 12: Example TypeScript Gate

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/gates/dispatch-rust-engineer.ts`

**Step 1: Create example gate**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/gates/dispatch-rust-engineer.ts`:

```typescript
#!/usr/bin/env node

/**
 * dispatch-rust-engineer gate
 *
 * Enforces use of rust-engineer agent when editing Rust files during /execute command.
 */

import { Session } from '../hooklib/dist';

interface GateOutput {
  continue?: boolean;
  decision?: 'block';
  reason?: string;
  additionalContext?: string;
}

async function main() {
  try {
    // Read hook input from stdin
    const input = await readStdin();
    const hookInput = JSON.parse(input);

    const session = new Session(hookInput.cwd || '.');

    // Only enforce during /execute
    const activeCommand = await session.get('active_command');
    if (activeCommand !== '/execute') {
      output({ continue: true });
      return;
    }

    // Check if Rust files edited
    const hasRustFiles = await session.contains('file_extensions', 'rs');
    if (!hasRustFiles) {
      output({ continue: true });
      return;
    }

    // Check if rust-engineer already active
    const activeAgent = await session.get('active_agent');
    if (activeAgent === 'rust-engineer') {
      output({ continue: true });
      return;
    }

    // Block and suggest dispatch
    output({
      decision: 'block',
      reason: 'Rust files edited during plan execution. Must use rust-engineer agent.',
      additionalContext: `
# Rust Engineering Required

You edited Rust files during plan execution. Must use rust-engineer agent for:
- Test-driven development (TDD)
- Code review before completion
- Prime Directive (type system compliance)

Dispatch the rust-engineer agent:

\`\`\`
Task(
  subagent_type="cipherpowers:rust-engineer",
  description="Implement Rust changes with TDD",
  prompt="Implement the following Rust changes: [describe work]"
)
\`\`\`
`.trim()
    });
  } catch (error) {
    console.error(JSON.stringify({
      continue: false,
      message: `Gate error: ${error}`
    }));
    process.exit(1);
  }
}

function readStdin(): Promise<string> {
  return new Promise((resolve) => {
    let data = '';
    process.stdin.on('data', (chunk) => { data += chunk; });
    process.stdin.on('end', () => { resolve(data); });
  });
}

function output(result: GateOutput) {
  console.log(JSON.stringify(result));
}

main();
```

**Step 2: Make executable**

Run:
```bash
chmod +x /Users/tobyhede/src/cipherpowers/plugin/hooks/gates/dispatch-rust-engineer.ts
```

**Step 3: Test manually**

Run:
```bash
echo '{"hook_event_name":"PostToolUse","cwd":"/tmp/test"}' | \
  npx ts-node /Users/tobyhede/src/cipherpowers/plugin/hooks/gates/dispatch-rust-engineer.ts
```

Expected: `{"continue":true}` (no active command)

**Step 4: Compile for production**

Add to package.json scripts:
```json
"build:gates": "tsc gates/*.ts --outDir dist/gates --module commonjs"
```

**Step 5: Commit**

```bash
git add gates/dispatch-rust-engineer.ts
git commit -m "feat(hooks): add dispatch-rust-engineer TypeScript gate example"
```

---

## Task 13: Integration with Dispatcher

**Files:**
- Create: `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/bash-integration.sh`
- Document: Update `/Users/tobyhede/src/cipherpowers/plugin/hooks/README.md`

**Step 1: Create bash helper functions**

Create `/Users/tobyhede/src/cipherpowers/plugin/hooks/hooklib/bash-integration.sh`:

```bash
#!/usr/bin/env bash
# Bash integration for TypeScript session state

# Get plugin root
HOOKLIB_ROOT="${CLAUDE_PLUGIN_ROOT}/hooks/hooklib"
HOOKLIB_CLI="${HOOKLIB_ROOT}/dist/cli.js"

# Session state helpers
session_get() {
  local key="$1"
  local cwd="${2:-.}"
  node "${HOOKLIB_CLI}" get "$key" "$cwd"
}

session_set() {
  local key="$1"
  local value="$2"
  local cwd="${3:-.}"
  node "${HOOKLIB_CLI}" set "$key" "$value" "$cwd"
}

session_append() {
  local key="$1"
  local value="$2"
  local cwd="${3:-.}"
  node "${HOOKLIB_CLI}" append "$key" "$value" "$cwd"
}

session_contains() {
  local key="$1"
  local value="$2"
  local cwd="${3:-.}"
  node "${HOOKLIB_CLI}" contains "$key" "$value" "$cwd"
}

session_clear() {
  local cwd="${1:-.}"
  node "${HOOKLIB_CLI}" clear "$cwd"
}

# Export for use in other scripts
export -f session_get
export -f session_set
export -f session_append
export -f session_contains
export -f session_clear
```

**Step 2: Document integration**

Add to `/Users/tobyhede/src/cipherpowers/plugin/hooks/README.md`:

```markdown
## TypeScript Session State Integration

### From Bash Scripts

Source the integration helpers:

```bash
source "${CLAUDE_PLUGIN_ROOT}/hooks/hooklib/bash-integration.sh"

# Use helper functions
session_set "active_command" "/execute"
COMMAND=$(session_get "active_command")

session_append "edited_files" "main.rs"

if session_contains "file_extensions" "rs"; then
  echo "Rust files edited"
fi

session_clear
```

### From dispatcher.sh

Update dispatcher to track session state:

```bash
case "$HOOK_EVENT" in
  SlashCommandStart)
    session_set "active_command" "$COMMAND" "$CWD"
    ;;
  PostToolUse)
    FILE_PATH=$(echo "$INPUT" | jq -r '.file_path // ""')
    if [ -n "$FILE_PATH" ]; then
      session_append "edited_files" "$FILE_PATH" "$CWD"
      EXT="${FILE_PATH##*.}"
      session_append "file_extensions" "$EXT" "$CWD"
    fi
    ;;
esac
```
```

**Step 3: Commit**

```bash
git add hooklib/bash-integration.sh
git commit -m "feat(hooks): add bash integration for session state"
```

---

## Success Criteria

- ✅ TypeScript compiles without errors
- ✅ All tests pass with >80% coverage
- ✅ CLI works from bash
- ✅ Session state persists across hook invocations
- ✅ Atomic file writes prevent corruption
- ✅ Example gate demonstrates TypeScript usage
- ✅ Documentation complete

## Next Steps

After implementation:

1. **Update dispatcher.sh** to track session state events
2. **Create additional TypeScript gates** as needed
3. **Test with real hook workflows** (PostToolUse, SlashCommandStart, etc.)
4. **Consider:** Migrate more gates from bash to TypeScript

## References

- Design doc: `/Users/tobyhede/psrc/battlespace/.work/reviews/typescript-vs-python-for-hooks.md`
- Session state spec: `/Users/tobyhede/psrc/battlespace/.work/reviews/session-state-tracking-design.md`
- CipherPowers hooks: `/Users/tobyhede/src/cipherpowers/plugin/hooks/`
