# TypeScript Hooks System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use cipherpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace bash-based hooks system with single TypeScript application while preserving exact behavior

**Architecture:** Single stdin/stdout Node.js app that loads gates.json config, executes shell command gates or built-in TypeScript gates, handles actions (CONTINUE/BLOCK/STOP/chaining), and injects context files following existing conventions.

**Tech Stack:** TypeScript, Node.js, Jest for unit tests, bash for integration tests

## Acceptance Criteria

This implementation is considered complete when:

1. **Test Coverage:** All existing integration tests pass unchanged, demonstrating behavior preservation
2. **Config Priority:** Config loading follows exact priority order (.claude/gates.json → gates.json → plugin default)
3. **Context Injection:** Convention-based context file discovery matches bash system exactly
4. **Action Handling:** All action types (CONTINUE/BLOCK/STOP/chaining) work identically to bash
5. **Performance:** TypeScript system overhead is within 2x of bash baseline (measure stdin→stdout latency)
6. **Type Safety:** All code compiles with TypeScript strict mode enabled
7. **Documentation:** TYPESCRIPT.md provides complete architecture guide for future maintainers

---

## Task 1: Project Setup and Core Types

**Files:**
- Create: `plugin/hooks/hooks-app/package.json`
- Create: `plugin/hooks/hooks-app/tsconfig.json`
- Create: `plugin/hooks/hooks-app/jest.config.js`
- Create: `plugin/hooks/hooks-app/src/types.ts`
- Create: `plugin/hooks/hooks-app/src/utils.ts`

**Step 1: Write failing test for HookInput type**

```typescript
// plugin/hooks/hooks-app/__tests__/types.test.ts
import { HookInput, GateResult } from '../src/types';

describe('Types', () => {
  test('HookInput has required fields', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test/path'
    };
    expect(input.hook_event_name).toBe('PostToolUse');
    expect(input.cwd).toBe('/test/path');
  });

  test('HookInput accepts optional PostToolUse fields', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test/path',
      tool_name: 'Edit',
      file_path: '/test/file.ts'
    };
    expect(input.tool_name).toBe('Edit');
    expect(input.file_path).toBe('/test/file.ts');
  });

  test('GateResult can be empty object', () => {
    const result: GateResult = {};
    expect(result).toBeDefined();
  });

  test('GateResult can have additionalContext', () => {
    const result: GateResult = {
      additionalContext: 'Test context'
    };
    expect(result.additionalContext).toBe('Test context');
  });

  test('GateResult can have block decision', () => {
    const result: GateResult = {
      decision: 'block',
      reason: 'Test reason'
    };
    expect(result.decision).toBe('block');
    expect(result.reason).toBe('Test reason');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test`
Expected: FAIL with "Cannot find module '../src/types'"

**Step 3: Create package.json**

```json
{
  "name": "@cipherpowers/hooks-app",
  "version": "1.0.0",
  "description": "TypeScript hooks dispatcher for CipherPowers",
  "main": "dist/cli.js",
  "scripts": {
    "build": "tsc",
    "watch": "tsc --watch",
    "test": "jest",
    "lint": "eslint src/**/*.ts __tests__/**/*.ts",
    "lint:fix": "eslint src/**/*.ts __tests__/**/*.ts --fix",
    "format": "prettier --write \"src/**/*.ts\" \"__tests__/**/*.ts\"",
    "format:check": "prettier --check \"src/**/*.ts\" \"__tests__/**/*.ts\"",
    "clean": "rm -rf dist"
  },
  "devDependencies": {
    "@types/jest": "^29.5.0",
    "@types/node": "^20.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.0.0",
    "jest": "^29.5.0",
    "prettier": "^3.0.0",
    "ts-jest": "^29.1.0",
    "typescript": "^5.0.0"
  },
  "dependencies": {}
}
```

**Step 4: Create tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "declaration": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "__tests__"]
}
```

**Step 5: Create jest.config.js**

```javascript
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  roots: ['<rootDir>/__tests__'],
  testMatch: ['**/*.test.ts'],
  collectCoverageFrom: ['src/**/*.ts'],
  moduleFileExtensions: ['ts', 'js', 'json']
};
```

**Step 6: Create .eslintrc.js**

```javascript
module.exports = {
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
    project: './tsconfig.json'
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended'
  ],
  rules: {
    '@typescript-eslint/explicit-function-return-type': 'warn',
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }]
  }
};
```

**Step 7: Create .prettierrc**

```json
{
  "semi": true,
  "trailingComma": "none",
  "singleQuote": true,
  "printWidth": 100,
  "tabWidth": 2
}
```

**Step 8: Create types.ts**

```typescript
// plugin/hooks/hooks-app/src/types.ts

export interface HookInput {
  hook_event_name: string;
  cwd: string;

  // PostToolUse
  tool_name?: string;
  file_path?: string;

  // SubagentStop
  agent_name?: string;
  subagent_name?: string;
  output?: string;

  // UserPromptSubmit
  user_message?: string;

  // SlashCommand/Skill
  command?: string;
  skill?: string;
}

export interface GateResult {
  // Success - add context and continue
  additionalContext?: string;

  // Block agent from proceeding
  decision?: 'block';
  reason?: string;

  // Stop Claude entirely
  continue?: false;
  message?: string;
}

export type GateExecute = (input: HookInput) => Promise<GateResult>;

export interface GateConfig {
  command?: string;
  on_pass?: string;
  on_fail?: string;
}

export interface HookConfig {
  enabled_tools?: string[];
  enabled_agents?: string[];
  gates?: string[];
}

export interface GatesConfig {
  hooks: Record<string, HookConfig>;
  gates: Record<string, GateConfig>;
}
```

**Step 9: Run npm install**

Run: `cd plugin/hooks/hooks-app && npm install`
Expected: Dependencies installed successfully

**Step 10: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test`
Expected: PASS (all type tests pass)

**Step 11: Run linting and formatting checks**

Run: `cd plugin/hooks/hooks-app && npm run lint && npm run format:check`
Expected: No linting errors, formatting is correct

**Step 12: Create shared utils.ts for common helpers**

```typescript
// plugin/hooks/hooks-app/src/utils.ts
import * as fs from 'fs/promises';

/**
 * Check if a file exists at the given path.
 * Used by config and context modules to probe file system.
 */
export async function fileExists(filePath: string): Promise<boolean> {
  try {
    await fs.access(filePath);
    return true;
  } catch {
    return false;
  }
}
```

**Step 13: Commit**

```bash
git add plugin/hooks/hooks-app/
git commit -m "feat(hooks): add hooks-app project setup with linting and core types"
```

---

## Task 2: Config Loading

**Files:**
- Create: `plugin/hooks/hooks-app/src/config.ts`
- Test: `plugin/hooks/hooks-app/__tests__/config.test.ts`

**Step 1: Write failing test for config loading**

```typescript
// plugin/hooks/hooks-app/__tests__/config.test.ts
import { loadConfig } from '../src/config';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';

describe('Config Loading', () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'hooks-test-'));
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  test('returns null when no config exists', async () => {
    const config = await loadConfig(testDir);
    expect(config).toBeNull();
  });

  test('loads .claude/gates.json with highest priority', async () => {
    const claudeDir = path.join(testDir, '.claude');
    await fs.mkdir(claudeDir);

    const config1 = { hooks: {}, gates: { test: { command: 'claude-config' } } };
    const config2 = { hooks: {}, gates: { test: { command: 'root-config' } } };

    await fs.writeFile(path.join(claudeDir, 'gates.json'), JSON.stringify(config1));
    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(config2));

    const config = await loadConfig(testDir);
    expect(config?.gates.test.command).toBe('claude-config');
  });

  test('loads gates.json from root when .claude does not exist', async () => {
    const config1 = { hooks: {}, gates: { test: { command: 'root-config' } } };
    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(config1));

    const config = await loadConfig(testDir);
    expect(config?.gates.test.command).toBe('root-config');
  });

  test('parses valid JSON config', async () => {
    const configObj = {
      hooks: {
        PostToolUse: {
          enabled_tools: ['Edit', 'Write'],
          gates: ['format', 'test']
        }
      },
      gates: {
        format: { command: 'npm run format', on_pass: 'CONTINUE' },
        test: { command: 'npm test', on_pass: 'CONTINUE' }
      }
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    const config = await loadConfig(testDir);
    expect(config?.hooks.PostToolUse.enabled_tools).toEqual(['Edit', 'Write']);
    expect(config?.gates.format.command).toBe('npm run format');
  });

  test('rejects unknown hook event', async () => {
    const configObj = {
      hooks: {
        UnknownEvent: { gates: [] }
      },
      gates: {}
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    await expect(loadConfig(testDir)).rejects.toThrow('Unknown hook event');
  });

  test('rejects undefined gate reference', async () => {
    const configObj = {
      hooks: {
        PostToolUse: { gates: ['nonexistent'] }
      },
      gates: {}
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    await expect(loadConfig(testDir)).rejects.toThrow('references undefined gate');
  });

  test('rejects invalid action', async () => {
    const configObj = {
      hooks: {
        PostToolUse: { gates: ['test'] }
      },
      gates: {
        test: { command: 'echo test', on_pass: 'INVALID' }
      }
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    await expect(loadConfig(testDir)).rejects.toThrow('is not CONTINUE/BLOCK/STOP or valid gate name');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test config.test.ts`
Expected: FAIL with "Cannot find module '../src/config'"

**Step 3: Implement config loading**

```typescript
// plugin/hooks/hooks-app/src/config.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { GatesConfig } from './types';
import { fileExists } from './utils';

const KNOWN_HOOK_EVENTS = [
  'PostToolUse',
  'SubagentStop',
  'UserPromptSubmit',
  'SlashCommandStart',
  'SlashCommandEnd',
  'SkillStart',
  'SkillEnd'
];

const KNOWN_ACTIONS = ['CONTINUE', 'BLOCK', 'STOP'];

/**
 * Validate config invariants to catch configuration errors early.
 * Throws descriptive errors when invariants are violated.
 */
export function validateConfig(config: GatesConfig): void {
  // Invariant: Hook event names must be known types
  for (const hookName of Object.keys(config.hooks)) {
    if (!KNOWN_HOOK_EVENTS.includes(hookName)) {
      throw new Error(`Unknown hook event: ${hookName}. Must be one of: ${KNOWN_HOOK_EVENTS.join(', ')}`);
    }
  }

  // Invariant: Gates referenced in hooks must exist in gates config
  for (const [hookName, hookConfig] of Object.entries(config.hooks)) {
    if (hookConfig.gates) {
      for (const gateName of hookConfig.gates) {
        if (!config.gates[gateName]) {
          throw new Error(`Hook '${hookName}' references undefined gate '${gateName}'`);
        }
      }
    }
  }

  // Invariant: Gate actions must be CONTINUE/BLOCK/STOP or reference existing gates
  for (const [gateName, gateConfig] of Object.entries(config.gates)) {
    for (const action of [gateConfig.on_pass, gateConfig.on_fail]) {
      if (action && !KNOWN_ACTIONS.includes(action) && !config.gates[action]) {
        throw new Error(`Gate '${gateName}' action '${action}' is not CONTINUE/BLOCK/STOP or valid gate name`);
      }
    }
  }
}

export async function loadConfig(cwd: string): Promise<GatesConfig | null> {
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || '';

  const paths = [
    path.join(cwd, '.claude', 'gates.json'),
    path.join(cwd, 'gates.json'),
    path.join(pluginRoot, 'hooks', 'gates.json')
  ];

  for (const configPath of paths) {
    if (await fileExists(configPath)) {
      const content = await fs.readFile(configPath, 'utf-8');
      const config = JSON.parse(content);
      validateConfig(config);
      return config;
    }
  }

  return null;
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test config.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/hooks-app/src/config.ts plugin/hooks/hooks-app/__tests__/config.test.ts
git commit -m "feat(hooks): add config loading with priority"
```

---

## Task 3: Context Injection

**Files:**
- Create: `plugin/hooks/hooks-app/src/context.ts`
- Test: `plugin/hooks/hooks-app/__tests__/context.test.ts`

**Step 1: Write failing test for context discovery**

```typescript
// plugin/hooks/hooks-app/__tests__/context.test.ts
import { discoverContextFile } from '../src/context';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';

describe('Context Injection', () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'hooks-test-'));
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  test('returns null when no context file exists', async () => {
    const result = await discoverContextFile(testDir, 'test-command', 'start');
    expect(result).toBeNull();
  });

  test('discovers flat context file', async () => {
    const contextDir = path.join(testDir, '.claude', 'context');
    await fs.mkdir(contextDir, { recursive: true });
    await fs.writeFile(path.join(contextDir, 'test-command-start.md'), 'content');

    const result = await discoverContextFile(testDir, 'test-command', 'start');
    expect(result).toBe(path.join(contextDir, 'test-command-start.md'));
  });

  test('discovers slash-command subdirectory', async () => {
    const contextDir = path.join(testDir, '.claude', 'context', 'slash-command');
    await fs.mkdir(contextDir, { recursive: true });
    await fs.writeFile(path.join(contextDir, 'test-command-start.md'), 'content');

    const result = await discoverContextFile(testDir, 'test-command', 'start');
    expect(result).toBe(path.join(contextDir, 'test-command-start.md'));
  });

  test('discovers nested slash-command directory', async () => {
    const contextDir = path.join(testDir, '.claude', 'context', 'slash-command', 'test-command');
    await fs.mkdir(contextDir, { recursive: true });
    await fs.writeFile(path.join(contextDir, 'start.md'), 'content');

    const result = await discoverContextFile(testDir, 'test-command', 'start');
    expect(result).toBe(path.join(contextDir, 'start.md'));
  });

  test('discovers skill context', async () => {
    const contextDir = path.join(testDir, '.claude', 'context', 'skill');
    await fs.mkdir(contextDir, { recursive: true });
    await fs.writeFile(path.join(contextDir, 'test-skill-start.md'), 'content');

    const result = await discoverContextFile(testDir, 'test-skill', 'start');
    expect(result).toBe(path.join(contextDir, 'test-skill-start.md'));
  });

  test('follows priority order - flat wins', async () => {
    const contextBase = path.join(testDir, '.claude', 'context');
    await fs.mkdir(path.join(contextBase, 'slash-command'), { recursive: true });

    await fs.writeFile(path.join(contextBase, 'test-command-start.md'), 'flat');
    await fs.writeFile(path.join(contextBase, 'slash-command', 'test-command-start.md'), 'subdir');

    const result = await discoverContextFile(testDir, 'test-command', 'start');
    expect(result).toBe(path.join(contextBase, 'test-command-start.md'));
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test context.test.ts`
Expected: FAIL with "Cannot find module '../src/context'"

**Step 3: Implement context discovery**

```typescript
// plugin/hooks/hooks-app/src/context.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { HookInput } from './types';
import { fileExists } from './utils';

/**
 * Discover context file following priority order.
 * Priority: flat > slash-command subdir > slash-command nested > skill subdir > skill nested
 */
export async function discoverContextFile(
  cwd: string,
  name: string,
  stage: string
): Promise<string | null> {
  const paths = [
    path.join(cwd, '.claude', 'context', `${name}-${stage}.md`),
    path.join(cwd, '.claude', 'context', 'slash-command', `${name}-${stage}.md`),
    path.join(cwd, '.claude', 'context', 'slash-command', name, `${stage}.md`),
    path.join(cwd, '.claude', 'context', 'skill', `${name}-${stage}.md`),
    path.join(cwd, '.claude', 'context', 'skill', name, `${stage}.md`)
  ];

  for (const filePath of paths) {
    if (await fileExists(filePath)) {
      return filePath;
    }
  }

  return null;
}

export async function injectContext(
  hookEvent: string,
  input: HookInput
): Promise<string | null> {
  let name: string | undefined;
  let stage: string | undefined;

  if (hookEvent === 'SlashCommandStart' || hookEvent === 'SlashCommandEnd') {
    name = input.command?.replace(/^\//, '');
    stage = hookEvent === 'SlashCommandStart' ? 'start' : 'end';
  } else if (hookEvent === 'SkillStart' || hookEvent === 'SkillEnd') {
    name = input.skill;
    stage = hookEvent === 'SkillStart' ? 'start' : 'end';
  }

  if (!name || !stage) return null;

  const contextFile = await discoverContextFile(input.cwd, name, stage);

  if (contextFile) {
    const content = await fs.readFile(contextFile, 'utf-8');
    return content;
  }

  return null;
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test context.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/hooks-app/src/context.ts plugin/hooks/hooks-app/__tests__/context.test.ts
git commit -m "feat(hooks): add convention-based context file discovery"
```

---

## Task 4: Gate Loader (Shell Commands)

**Files:**
- Create: `plugin/hooks/hooks-app/src/gate-loader.ts`
- Test: `plugin/hooks/hooks-app/__tests__/gate-loader.test.ts`

**Step 1: Write failing test for shell command execution**

```typescript
// plugin/hooks/hooks-app/__tests__/gate-loader.test.ts
import { executeShellCommand } from '../src/gate-loader';
import * as path from 'path';
import * as os from 'os';

describe('Gate Loader - Shell Commands', () => {
  test('executes shell command and returns exit code', async () => {
    const result = await executeShellCommand('echo "test"', process.cwd());
    expect(result.exitCode).toBe(0);
    expect(result.output).toContain('test');
  });

  test('captures non-zero exit code', async () => {
    const result = await executeShellCommand('exit 1', process.cwd());
    expect(result.exitCode).toBe(1);
  });

  test('captures stdout', async () => {
    const result = await executeShellCommand('echo "hello world"', process.cwd());
    expect(result.output).toContain('hello world');
  });

  test('captures stderr', async () => {
    const result = await executeShellCommand('echo "error" >&2', process.cwd());
    expect(result.output).toContain('error');
  });

  test('executes in specified directory', async () => {
    const tmpDir = os.tmpdir();
    const result = await executeShellCommand('pwd', tmpDir);
    expect(result.output.trim()).toBe(tmpDir);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test gate-loader.test.ts`
Expected: FAIL with "Cannot find module '../src/gate-loader'"

**Step 3: Implement shell command execution**

```typescript
// plugin/hooks/hooks-app/src/gate-loader.ts
import { exec } from 'child_process';
import { promisify } from 'util';
import { HookInput, GateResult, GateConfig } from './types';

const execAsync = promisify(exec);

export interface ShellResult {
  exitCode: number;
  output: string;
}

/**
 * Execute shell command from gate configuration with timeout.
 *
 * SECURITY MODEL: gates.json is trusted configuration (project-controlled, not user input).
 * Commands are executed without sanitization because:
 * 1. gates.json is committed to repository or managed by project admins
 * 2. Users cannot inject commands without write access to gates.json
 * 3. If gates.json is compromised, the project is already compromised
 *
 * This is equivalent to package.json scripts or Makefile targets - trusted project configuration.
 *
 * ERROR HANDLING: Commands timeout after 30 seconds to prevent hung gates.
 */
export async function executeShellCommand(
  command: string,
  cwd: string,
  timeoutMs: number = 30000
): Promise<ShellResult> {
  try {
    const { stdout, stderr } = await execAsync(command, { cwd, timeout: timeoutMs });
    return {
      exitCode: 0,
      output: stdout + stderr
    };
  } catch (error: any) {
    if (error.killed && error.signal === 'SIGTERM') {
      return {
        exitCode: 124, // Standard timeout exit code
        output: `Command timed out after ${timeoutMs}ms`
      };
    }
    return {
      exitCode: error.code || 1,
      output: (error.stdout || '') + (error.stderr || '')
    };
  }
}

export async function executeGate(
  gateName: string,
  gateConfig: GateConfig,
  input: HookInput
): Promise<{ passed: boolean; result: GateResult }> {
  if (gateConfig.command) {
    // Shell command gate
    const shellResult = await executeShellCommand(gateConfig.command, input.cwd);
    const passed = shellResult.exitCode === 0;

    return {
      passed,
      result: {
        additionalContext: shellResult.output
      }
    };
  } else {
    // Built-in TypeScript gate (to be implemented later)
    throw new Error(`Built-in gate ${gateName} not yet implemented`);
  }
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test gate-loader.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/hooks-app/src/gate-loader.ts plugin/hooks/hooks-app/__tests__/gate-loader.test.ts
git commit -m "feat(hooks): add shell command gate execution"
```

---

## Task 5: Action Handler

**Files:**
- Create: `plugin/hooks/hooks-app/src/action-handler.ts`
- Test: `plugin/hooks/hooks-app/__tests__/action-handler.test.ts`

**Step 1: Write failing test for action handling**

```typescript
// plugin/hooks/hooks-app/__tests__/action-handler.test.ts
import { handleAction, ActionResult } from '../src/action-handler';
import { GateResult, GatesConfig } from '../src/types';

const mockConfig: GatesConfig = {
  hooks: {},
  gates: {
    'next-gate': { command: 'echo "next"', on_pass: 'CONTINUE' }
  }
};

const mockInput = {
  hook_event_name: 'PostToolUse',
  cwd: '/test'
};

describe('Action Handler', () => {
  test('CONTINUE returns continue=true', async () => {
    const result: GateResult = {};
    const action = await handleAction('CONTINUE', result, mockConfig, mockInput);

    expect(action.continue).toBe(true);
    expect(action.context).toBeUndefined();
  });

  test('CONTINUE with context returns context', async () => {
    const result: GateResult = { additionalContext: 'test context' };
    const action = await handleAction('CONTINUE', result, mockConfig, mockInput);

    expect(action.continue).toBe(true);
    expect(action.context).toBe('test context');
  });

  test('BLOCK returns continue=false', async () => {
    const result: GateResult = { decision: 'block', reason: 'test reason' };
    const action = await handleAction('BLOCK', result, mockConfig, mockInput);

    expect(action.continue).toBe(false);
    expect(action.blockReason).toBe('test reason');
  });

  test('BLOCK with no reason uses default', async () => {
    const result: GateResult = {};
    const action = await handleAction('BLOCK', result, mockConfig, mockInput);

    expect(action.continue).toBe(false);
    expect(action.blockReason).toBe('Gate failed');
  });

  test('STOP returns continue=false with stop message', async () => {
    const result: GateResult = { message: 'stop message' };
    const action = await handleAction('STOP', result, mockConfig, mockInput);

    expect(action.continue).toBe(false);
    expect(action.stopMessage).toBe('stop message');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test action-handler.test.ts`
Expected: FAIL with "Cannot find module '../src/action-handler'"

**Step 3: Implement action handler**

```typescript
// plugin/hooks/hooks-app/src/action-handler.ts
import { GateResult, GatesConfig, HookInput } from './types';

export interface ActionResult {
  continue: boolean;
  context?: string;
  blockReason?: string;
  stopMessage?: string;
  chainedGate?: string;
}

export async function handleAction(
  action: string,
  gateResult: GateResult,
  config: GatesConfig,
  input: HookInput
): Promise<ActionResult> {
  switch (action) {
    case 'CONTINUE':
      return {
        continue: true,
        context: gateResult.additionalContext
      };

    case 'BLOCK':
      return {
        continue: false,
        blockReason: gateResult.reason || 'Gate failed'
      };

    case 'STOP':
      return {
        continue: false,
        stopMessage: gateResult.message || 'Gate stopped execution'
      };

    default:
      // Gate chaining - action is another gate name
      return {
        continue: true,
        chainedGate: action
      };
  }
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test action-handler.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/hooks-app/src/action-handler.ts plugin/hooks/hooks-app/__tests__/action-handler.test.ts
git commit -m "feat(hooks): add action handler for CONTINUE/BLOCK/STOP"
```

---

## Task 6: Dispatcher Core Logic

**Files:**
- Create: `plugin/hooks/hooks-app/src/dispatcher.ts`
- Test: `plugin/hooks/hooks-app/__tests__/dispatcher.test.ts`

**Step 1: Write failing test for dispatcher**

```typescript
// plugin/hooks/hooks-app/__tests__/dispatcher.test.ts
import { shouldProcessHook } from '../src/dispatcher';
import { HookInput, HookConfig } from '../src/types';

describe('Dispatcher - Event Filtering', () => {
  test('PostToolUse with enabled tool returns true', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test',
      tool_name: 'Edit'
    };

    const hookConfig: HookConfig = {
      enabled_tools: ['Edit', 'Write']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('PostToolUse with disabled tool returns false', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test',
      tool_name: 'Read'
    };

    const hookConfig: HookConfig = {
      enabled_tools: ['Edit', 'Write']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(false);
  });

  test('SubagentStop with enabled agent returns true', () => {
    const input: HookInput = {
      hook_event_name: 'SubagentStop',
      cwd: '/test',
      agent_name: 'cipherpowers:coder'
    };

    const hookConfig: HookConfig = {
      enabled_agents: ['cipherpowers:coder']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('SubagentStop with disabled agent returns false', () => {
    const input: HookInput = {
      hook_event_name: 'SubagentStop',
      cwd: '/test',
      agent_name: 'other-agent'
    };

    const hookConfig: HookConfig = {
      enabled_agents: ['cipherpowers:coder']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(false);
  });

  test('SubagentStop checks subagent_name if agent_name missing', () => {
    const input: HookInput = {
      hook_event_name: 'SubagentStop',
      cwd: '/test',
      subagent_name: 'cipherpowers:coder'
    };

    const hookConfig: HookConfig = {
      enabled_agents: ['cipherpowers:coder']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('UserPromptSubmit always returns true', () => {
    const input: HookInput = {
      hook_event_name: 'UserPromptSubmit',
      cwd: '/test'
    };

    const hookConfig: HookConfig = {};

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('No filtering config returns true', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test',
      tool_name: 'Edit'
    };

    const hookConfig: HookConfig = {};

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test dispatcher.test.ts`
Expected: FAIL with "Cannot find module '../src/dispatcher'"

**Step 3: Implement dispatcher filtering logic**

```typescript
// plugin/hooks/hooks-app/src/dispatcher.ts
import { HookInput, HookConfig, GatesConfig } from './types';
import { loadConfig } from './config';
import { injectContext } from './context';
import { executeGate } from './gate-loader';
import { handleAction } from './action-handler';

export function shouldProcessHook(input: HookInput, hookConfig: HookConfig): boolean {
  const hookEvent = input.hook_event_name;

  // PostToolUse filtering
  if (hookEvent === 'PostToolUse') {
    if (hookConfig.enabled_tools && hookConfig.enabled_tools.length > 0) {
      return hookConfig.enabled_tools.includes(input.tool_name || '');
    }
  }

  // SubagentStop filtering
  if (hookEvent === 'SubagentStop') {
    if (hookConfig.enabled_agents && hookConfig.enabled_agents.length > 0) {
      const agentName = input.agent_name || input.subagent_name || '';
      return hookConfig.enabled_agents.includes(agentName);
    }
  }

  // No filtering or other events
  return true;
}

export interface DispatchResult {
  context?: string;
  blockReason?: string;
  stopMessage?: string;
}

/**
 * ERROR HANDLING: Circular gate chain prevention (max 10 gates per dispatch).
 * Prevents infinite loops from misconfigured gate chains.
 */
const MAX_GATES_PER_DISPATCH = 10;

export async function dispatch(input: HookInput): Promise<DispatchResult> {
  const hookEvent = input.hook_event_name;
  const cwd = input.cwd;

  // 1. Load config
  const config = await loadConfig(cwd);
  if (!config) {
    return {}; // Clean exit - graceful degradation when no config
  }

  // 2. Check if hook event is configured
  const hookConfig = config.hooks[hookEvent];
  if (!hookConfig) {
    return {}; // Clean exit - graceful degradation when hook not configured
  }

  // 3. Filter by enabled lists
  if (!shouldProcessHook(input, hookConfig)) {
    return {}; // Clean exit
  }

  // 4. Context injection
  const contextContent = await injectContext(hookEvent, input);
  let accumulatedContext = contextContent || '';

  // 5. Run gates in sequence with circular chain prevention
  const gates = hookConfig.gates || [];
  let gatesExecuted = 0;

  for (let i = 0; i < gates.length; i++) {
    const gateName = gates[i];

    // Circuit breaker: prevent infinite chains
    if (gatesExecuted >= MAX_GATES_PER_DISPATCH) {
      return {
        blockReason: `Exceeded max gate chain depth (${MAX_GATES_PER_DISPATCH}). Check for circular references.`
      };
    }

    const gateConfig = config.gates[gateName];
    if (!gateConfig) {
      // Graceful degradation: skip undefined gates with warning
      accumulatedContext += `\nWarning: Gate '${gateName}' not defined, skipping`;
      continue;
    }

    gatesExecuted++;

    // Execute gate
    const { passed, result } = await executeGate(gateName, gateConfig, input);

    // Determine action
    const action = passed ?
      (gateConfig.on_pass || 'CONTINUE') :
      (gateConfig.on_fail || 'BLOCK');

    // Handle action
    const actionResult = await handleAction(action, result, config, input);

    if (actionResult.context) {
      accumulatedContext += '\n' + actionResult.context;
    }

    if (!actionResult.continue) {
      return {
        context: accumulatedContext,
        blockReason: actionResult.blockReason,
        stopMessage: actionResult.stopMessage
      };
    }

    // Gate chaining
    if (actionResult.chainedGate) {
      gates.push(actionResult.chainedGate);
    }
  }

  return {
    context: accumulatedContext
  };
}
```

**Step 4: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test dispatcher.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add plugin/hooks/hooks-app/src/dispatcher.ts plugin/hooks/hooks-app/__tests__/dispatcher.test.ts
git commit -m "feat(hooks): add dispatcher with event filtering and gate execution"
```

---

## Task 7: CLI Entry Point

**Files:**
- Create: `plugin/hooks/hooks-app/src/cli.ts`
- Create: `plugin/hooks/hooks-app/src/index.ts`

**Step 1: Write manual test script**

```bash
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
```

**Step 2: Verify test fails**

Run: `cd plugin/hooks/hooks-app && chmod +x test-cli.sh && ./test-cli.sh`
Expected: FAIL with "Cannot find module './dist/cli.js'"

**Step 3: Implement CLI entry point**

```typescript
// plugin/hooks/hooks-app/src/cli.ts
import { HookInput } from './types';
import { dispatch } from './dispatcher';

interface OutputMessage {
  additionalContext?: string;
  decision?: string;
  reason?: string;
  continue?: boolean;
  message?: string;
}

async function main() {
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
      console.error(JSON.stringify({
        continue: false,
        message: 'Invalid JSON input'
      }));
      process.exit(1);
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
    console.error(JSON.stringify({
      continue: false,
      message: `Unexpected error: ${error}`
    }));
    process.exit(1);
  }
}

main();
```

**Step 4: Create index.ts for exports**

```typescript
// plugin/hooks/hooks-app/src/index.ts
export * from './types';
export * from './config';
export * from './context';
export * from './gate-loader';
export * from './action-handler';
export * from './dispatcher';
```

**Step 5: Build the project**

Run: `cd plugin/hooks/hooks-app && npm run build`
Expected: Compilation successful, dist/ directory created

**Step 6: Run manual test**

Run: `cd plugin/hooks/hooks-app && ./test-cli.sh`
Expected: Tests run without errors

**Step 7: Commit**

```bash
git add plugin/hooks/hooks-app/src/cli.ts plugin/hooks/hooks-app/src/index.ts plugin/hooks/hooks-app/test-cli.sh
git commit -m "feat(hooks): add CLI entry point with stdin/stdout"
```

---

## Task 8: Built-in TypeScript Gates

**Files:**
- Create: `plugin/hooks/gates/plan-compliance.ts`
- Test: `plugin/hooks/hooks-app/__tests__/builtin-gates.test.ts`

**Step 1: Write failing test for plan-compliance gate**

```typescript
// plugin/hooks/hooks-app/__tests__/builtin-gates.test.ts
import { executeBuiltinGate } from '../src/gate-loader';
import { HookInput } from '../src/types';

describe('Built-in Gates', () => {
  describe('plan-compliance', () => {
    test('blocks when no STATUS in output', async () => {
      const input: HookInput = {
        hook_event_name: 'SubagentStop',
        cwd: '/test',
        output: 'Task completed'
      };

      const result = await executeBuiltinGate('plan-compliance', input);
      expect(result.decision).toBe('block');
      expect(result.reason).toContain('STATUS');
    });

    test('blocks when STATUS: BLOCKED', async () => {
      const input: HookInput = {
        hook_event_name: 'SubagentStop',
        cwd: '/test',
        output: 'STATUS: BLOCKED\nSome reason'
      };

      const result = await executeBuiltinGate('plan-compliance', input);
      expect(result.decision).toBe('block');
      expect(result.reason).toContain('BLOCKED');
    });

    test('continues when STATUS: OK', async () => {
      const input: HookInput = {
        hook_event_name: 'SubagentStop',
        cwd: '/test',
        output: 'STATUS: OK\nTask completed successfully'
      };

      const result = await executeBuiltinGate('plan-compliance', input);
      expect(result.decision).toBeUndefined();
      expect(result.additionalContext).toContain('successfully');
    });
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd plugin/hooks/hooks-app && npm test builtin-gates.test.ts`
Expected: FAIL with "Cannot find module 'executeBuiltinGate'"

**Step 3: Update gate-loader.ts to support built-in gates**

```typescript
// plugin/hooks/hooks-app/src/gate-loader.ts (add to existing file)
import * as path from 'path';

export async function executeBuiltinGate(
  gateName: string,
  input: HookInput
): Promise<GateResult> {
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || '';
  const gatePath = path.join(pluginRoot, 'hooks', 'gates', gateName);

  try {
    const module = await import(gatePath);
    return await module.execute(input);
  } catch (error) {
    throw new Error(`Failed to load built-in gate ${gateName}: ${error}`);
  }
}

// Update executeGate function
export async function executeGate(
  gateName: string,
  gateConfig: GateConfig,
  input: HookInput
): Promise<{ passed: boolean; result: GateResult }> {
  if (gateConfig.command) {
    // Shell command gate
    const shellResult = await executeShellCommand(gateConfig.command, input.cwd);
    const passed = shellResult.exitCode === 0;

    return {
      passed,
      result: {
        additionalContext: shellResult.output
      }
    };
  } else {
    // Built-in TypeScript gate
    const result = await executeBuiltinGate(gateName, input);
    const passed = !result.decision && result.continue !== false;

    return {
      passed,
      result
    };
  }
}
```

**Step 4: Create plan-compliance gate**

```typescript
// plugin/hooks/gates/plan-compliance.ts
import { HookInput, GateResult } from '../hooks-app/src/types';

export async function execute(input: HookInput): Promise<GateResult> {
  const output = input.output || '';

  if (!output.includes('STATUS:')) {
    return {
      decision: 'block',
      reason: 'Agent must provide STATUS in completion report'
    };
  }

  if (output.includes('STATUS: BLOCKED')) {
    return {
      decision: 'block',
      reason: 'Agent reported BLOCKED. Review required.'
    };
  }

  return {
    additionalContext: 'Task completed successfully (STATUS: OK)'
  };
}
```

**Step 5: Run test to verify it passes**

Run: `cd plugin/hooks/hooks-app && npm test builtin-gates.test.ts`
Expected: PASS

**Step 6: Commit**

```bash
git add plugin/hooks/gates/plan-compliance.ts plugin/hooks/hooks-app/src/gate-loader.ts plugin/hooks/hooks-app/__tests__/builtin-gates.test.ts
git commit -m "feat(hooks): add built-in TypeScript gate support with plan-compliance"
```

---

## Task 9: Hook Registration

**Files:**
- Create: `plugin/hooks/hooks.json`

**Step 1: Write hooks.json**

```json
{
  "hooks": {
    "PostToolUse": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }],
    "SubagentStop": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }],
    "UserPromptSubmit": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }],
    "SlashCommandStart": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }],
    "SlashCommandEnd": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }],
    "SkillStart": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }],
    "SkillEnd": [{
      "matcher": ".*",
      "hooks": [{
        "type": "command",
        "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
      }]
    }]
  }
}
```

**Step 2: Commit**

```bash
git add plugin/hooks/hooks.json
git commit -m "feat(hooks): add hook registration for all hook events"
```

---

## Task 10: Example Configuration

**Files:**
- Create: `plugin/hooks/examples/typescript-gates.json`

**Step 1: Create example config showing TypeScript gates**

```json
{
  "hooks": {
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write", "NotebookEdit"],
      "gates": ["format", "check"]
    },
    "SubagentStop": {
      "enabled_agents": [
        "cipherpowers:coder",
        "cipherpowers:rust-engineer",
        "cipherpowers:code-reviewer"
      ],
      "gates": ["plan-compliance"]
    }
  },
  "gates": {
    "format": {
      "command": "mise run format",
      "on_pass": "check",
      "on_fail": "BLOCK"
    },
    "check": {
      "command": "mise run check",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "plan-compliance": {
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

**Step 2: Commit**

```bash
git add plugin/hooks/examples/typescript-gates.json
git commit -m "docs(hooks): add example config showing TypeScript gate usage"
```

---

## Task 11: Integration Tests

**Files:**
- Create: `plugin/hooks/tests/test-typescript-app.sh`

**Step 1: Write integration test script**

```bash
#!/bin/bash
# plugin/hooks/tests/test-typescript-app.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
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

if ! echo "$OUTPUT" | grep -q "timed out"; then
  echo "FAIL: Expected timeout message"
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
```

**Step 2: Make script executable and run**

Run: `chmod +x plugin/hooks/tests/test-typescript-app.sh && plugin/hooks/tests/test-typescript-app.sh`
Expected: All tests pass

**Step 3: Commit**

```bash
git add plugin/hooks/tests/test-typescript-app.sh
git commit -m "test(hooks): add integration tests for TypeScript hooks app"
```

---

## Task 12: Documentation

**Files:**
- Create: `plugin/hooks/TYPESCRIPT.md`
- Modify: `plugin/hooks/README.md`

**Step 1: Read existing README**

Run: Read `plugin/hooks/README.md` to understand current structure

**Step 2: Create TypeScript documentation**

```markdown
# TypeScript Hooks System

The CipherPowers hooks system is implemented as a single TypeScript Node.js application that replaces the previous bash-based system.

## Architecture

```
stdin (JSON) → hooks-app → stdout (JSON)
```

**Components:**
- `cli.ts` - Entry point, reads stdin, writes stdout
- `dispatcher.ts` - Loads config, filters events, runs gates
- `gate-loader.ts` - Executes shell or TypeScript gates
- `action-handler.ts` - Processes CONTINUE/BLOCK/STOP/chaining
- `config.ts` - Config file discovery with priority
- `context.ts` - Convention-based context injection

## Gate Types

### Shell Command Gates

Defined in `gates.json` with `command` field:

```json
{
  "gates": {
    "format": {
      "command": "npm run format",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

App executes shell command, exit code determines pass/fail.

### Built-in TypeScript Gates

No `command` field - loaded from `plugin/hooks/gates/`:

```json
{
  "gates": {
    "plan-compliance": {
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

**Available built-in gates:**
- `plan-compliance` - Validates agent completion reports

## Development

### Build

```bash
cd plugin/hooks/hooks-app
npm install
npm run build
```

### Test

```bash
# Unit tests
npm test

# Integration tests
../tests/test-typescript-app.sh
```

### Create New Built-in Gate

1. Create `plugin/hooks/gates/my-gate.ts`:

```typescript
import { HookInput, GateResult } from '../hooks-app/src/types';

export async function execute(input: HookInput): Promise<GateResult> {
  // Your logic here
  return {
    additionalContext: 'Gate passed'
  };
}
```

2. Add to `gates.json`:

```json
{
  "gates": {
    "my-gate": {
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

3. Write tests in `plugin/hooks/hooks-app/__tests__/`

## Session State Integration

Built-in gates can access session state via `hooklib`:

```typescript
import { Session } from '../hooklib/dist';

export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);
  const activeCommand = await session.get('active_command');

  // Your logic
}
```

## Migration from Bash

The TypeScript system preserves exact behavior:
- Same config file discovery order
- Same context injection convention
- Same action handling (CONTINUE/BLOCK/STOP/chaining)
- All integration tests pass unchanged

No migration needed for projects - just use updated hooks.json registration.
```

**Step 3: Update README to reference TypeScript docs**

Add to `plugin/hooks/README.md`:

```markdown
## Implementation

The hooks system is implemented in TypeScript for type safety and testability. See [TYPESCRIPT.md](TYPESCRIPT.md) for architecture details.

**Quick links:**
- [TypeScript Architecture](TYPESCRIPT.md)
- [Creating Built-in Gates](TYPESCRIPT.md#create-new-built-in-gate)
- [Development Guide](TYPESCRIPT.md#development)
```

**Step 4: Commit**

```bash
git add plugin/hooks/TYPESCRIPT.md plugin/hooks/README.md
git commit -m "docs(hooks): add TypeScript architecture documentation"
```

---

## Task 13: Session State Module Import

**Files:**
- Modify: `plugin/hooks/hooks-app/package.json`
- Create: `plugin/hooks/gates/example-session-gate.ts`

**Step 1: Update package.json to include hooklib dependency**

```json
{
  "name": "@cipherpowers/hooks-app",
  "version": "1.0.0",
  "description": "TypeScript hooks dispatcher for CipherPowers",
  "main": "dist/cli.js",
  "scripts": {
    "build": "tsc",
    "watch": "tsc --watch",
    "test": "jest",
    "clean": "rm -rf dist"
  },
  "devDependencies": {
    "@types/jest": "^29.5.0",
    "@types/node": "^20.0.0",
    "jest": "^29.5.0",
    "ts-jest": "^29.1.0",
    "typescript": "^5.0.0"
  },
  "dependencies": {
    "@cipherpowers/hooklib": "file:../hooklib"
  }
}
```

**Step 2: Create example session-aware gate**

```typescript
// plugin/hooks/gates/example-session-gate.ts
import { HookInput, GateResult } from '../hooks-app/src/types';
import { Session } from '../hooklib/dist';

/**
 * Example gate showing session state integration.
 * Not used in production - serves as reference.
 */
export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);

  const activeCommand = await session.get('active_command');
  const editedFiles = await session.get('edited_files');

  return {
    additionalContext: `Active: ${activeCommand}, Files: ${editedFiles?.length || 0}`
  };
}
```

**Step 3: Install hooklib dependency**

Run: `cd plugin/hooks/hooks-app && npm install`
Expected: hooklib linked successfully

**Step 4: Commit**

```bash
git add plugin/hooks/hooks-app/package.json plugin/hooks/gates/example-session-gate.ts
git commit -m "feat(hooks): add hooklib integration for session-aware gates"
```

---

## Task 14: Build and Verification

**Files:**
- Modify: `plugin/hooks/hooks-app/package.json` (add build script)

**Step 1: Add build script to root**

Run: `cd plugin/hooks/hooks-app && npm run build`
Expected: TypeScript compilation succeeds

**Step 2: Verify all tests pass**

Run: `cd plugin/hooks/hooks-app && npm test`
Expected: All unit tests pass

**Step 3: Verify integration tests pass**

Run: `plugin/hooks/tests/test-typescript-app.sh`
Expected: All integration tests pass

**Step 4: Verify hooks.json is valid**

Run: `cat plugin/hooks/hooks.json | jq .`
Expected: Valid JSON output

**Step 5: Test manual stdin/stdout**

```bash
echo '{"hook_event_name":"PostToolUse","cwd":"'$(pwd)'","tool_name":"Edit"}' | \
  node plugin/hooks/hooks-app/dist/cli.js
```
Expected: Clean exit (no config in current directory)

**Step 6: Performance benchmark (stdin→stdout latency)**

```bash
# Benchmark TypeScript hooks app (10 iterations)
echo "Benchmarking TypeScript hooks app..."
TOTAL_MS=0
for i in {1..10}; do
  START=$(date +%s%N)
  echo '{"hook_event_name":"PostToolUse","cwd":"'$(pwd)'","tool_name":"Edit"}' | \
    node plugin/hooks/hooks-app/dist/cli.js > /dev/null
  END=$(date +%s%N)
  DURATION_MS=$(( (END - START) / 1000000 ))
  TOTAL_MS=$((TOTAL_MS + DURATION_MS))
  echo "  Iteration $i: ${DURATION_MS}ms"
done
AVG_MS=$((TOTAL_MS / 10))
echo "Average latency: ${AVG_MS}ms"

# Acceptance: Should be within 2x of bash system (typically <100ms)
if [ "$AVG_MS" -gt 200 ]; then
  echo "WARNING: TypeScript system slower than expected (>${AVG_MS}ms)"
  echo "Consider: Node.js startup time, optimize module loading"
else
  echo "PASS: Performance acceptable (${AVG_MS}ms average)"
fi
```

Expected: Average latency <100ms (Node.js startup + execution)

**Step 7: Commit verification results**

```bash
git add -A
git commit -m "chore(hooks): verify TypeScript hooks system build and tests"
```

---

## Completion Checklist

**Before marking complete, verify:**

- [ ] All unit tests pass (`npm test`)
- [ ] All integration tests pass (`test-typescript-app.sh`)
- [ ] TypeScript compiles without errors (`npm run build`)
- [ ] hooks.json registration is valid
- [ ] Example configuration provided
- [ ] Documentation complete (TYPESCRIPT.md)
- [ ] Session state integration working
- [ ] Built-in gates loadable
- [ ] Shell command gates executable
- [ ] Context injection follows conventions
- [ ] Config loading priority correct

**Next Steps:**

1. Test in real Claude Code session
2. Migrate existing gate configs to new format
3. Add more built-in gates as needed (commands, dispatch-rust-engineer, etc.)
4. Monitor performance vs bash system
5. Update marketplace documentation

**References:**
- Design: `docs/plans/2025-11-21-typescript-hooks-design.md`
- Skill: `@cipherpowers:executing-plans`
- Standards: `@${CLAUDE_PLUGIN_ROOT}standards/code-review.md`
