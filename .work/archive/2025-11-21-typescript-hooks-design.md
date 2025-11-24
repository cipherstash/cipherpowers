# TypeScript Hooks System Design

**Date:** 2025-11-21
**Goal:** Replace bash-based hooks system with single TypeScript application

## Overview

Replace the entire bash hooks system (`dispatcher.sh`, `shared-functions.sh`, gate scripts) with a single TypeScript Node.js application that preserves exact behavior while providing type safety, testability, and maintainability.

**Key Principle:** Convention over configuration - follow existing patterns for discovery and integration.

## Architecture

### Single TypeScript Application: `hooks-app`

```
stdin (JSON) → hooks-app → stdout (JSON)
```

**Core Components:**

1. **cli.ts** - Entry point that reads stdin, parses hook input, delegates to dispatcher
2. **dispatcher.ts** - Loads config, filters events, injects context, runs gates
3. **gate-loader.ts** - Executes shell commands or loads built-in TypeScript gates
4. **action-handler.ts** - Processes gate results (CONTINUE, BLOCK, STOP, chaining)
5. **config.ts** - Config file discovery with priority
6. **context.ts** - Convention-based context file injection
7. **types.ts** - Shared TypeScript interfaces

## Gate Types

### 1. Shell Command Gates (Project-Defined)

Defined entirely in `gates.json`:

```json
{
  "gates": {
    "format": {
      "command": "mise run format",
      "on_pass": "check",
      "on_fail": "BLOCK"
    },
    "test": {
      "command": "npm test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  }
}
```

App executes shell command, uses exit code (0 = pass, non-zero = fail).

### 2. Built-in TypeScript Gates (Plugin Only)

No `command` field in config:

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

App loads from `${CLAUDE_PLUGIN_ROOT}/hooks/gates/plan-compliance.ts`.

**Rationale:** Projects customize via shell commands in config. TypeScript gates only needed for built-in logic (plan compliance, command injection, session-aware gates). No project-level TypeScript gate loading - adds no value.

## Gate Resolution

```typescript
async function executeGate(gateName: string, config: GatesConfig, input: HookInput) {
  const gateConfig = config.gates[gateName];

  if (gateConfig.command) {
    // Shell command gate
    return executeShellCommand(gateConfig.command, input.cwd);
  } else {
    // Built-in TypeScript gate
    const module = await import(`${PLUGIN_ROOT}/gates/${gateName}`);
    return module.execute(input);
  }
}
```

## TypeScript Gate Interface

```typescript
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
```

**Example Built-in Gate:**

```typescript
// gates/plan-compliance.ts
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

  return { additionalContext: 'Task completed successfully (STATUS: OK)' };
}
```

## Dispatcher Flow

```typescript
export async function dispatch(input: HookInput): Promise<void> {
  const hookEvent = input.hook_event_name;
  const cwd = input.cwd;

  // 1. Load gates.json config (with priority)
  const config = await loadConfig(cwd);
  if (!config) return;

  // 2. Check if this hook event is configured
  const hookConfig = config.hooks[hookEvent];
  if (!hookConfig) return;

  // 3. Filter by enabled lists (PostToolUse/SubagentStop only)
  if (hookEvent === 'PostToolUse') {
    if (!hookConfig.enabled_tools?.includes(input.tool_name)) return;
  } else if (hookEvent === 'SubagentStop') {
    const agentName = input.agent_name || input.subagent_name;
    if (!hookConfig.enabled_agents?.includes(agentName)) return;
  }

  // 4. Convention-based context injection
  await injectContext(hookEvent, input, cwd);

  // 5. Run built-in gates first
  if (hookEvent === 'UserPromptSubmit') {
    await runBuiltInGate('commands', input);
  } else if (hookEvent === 'SubagentStop') {
    const shouldContinue = await runBuiltInGate('plan-compliance', input);
    if (!shouldContinue) return;
  }

  // 6. Run configured gates in sequence
  const gates = hookConfig.gates || [];
  for (const gateName of gates) {
    const shouldContinue = await runGateWithActions(gateName, config, input);
    if (!shouldContinue) break;
  }
}
```

**Key Behaviors:**
- Clean exits when not configured (no config, hook not configured, not enabled)
- Built-in gates always run first
- Gate sequence with early exit on BLOCK/STOP
- No "enabled" flag - to disable, remove from config

## Gate Execution with Actions

```typescript
async function runGateWithActions(
  gateName: string,
  config: GatesConfig,
  input: HookInput
): Promise<boolean> {

  const gateConfig = config.gates[gateName];
  if (!gateConfig) {
    outputError(`Gate '${gateName}' not defined in gates.json`);
    return false;
  }

  // Execute the gate
  let result: GateResult;
  let passed: boolean;

  if (gateConfig.command) {
    const shellResult = await executeShellCommand(gateConfig.command, input.cwd);
    passed = shellResult.exitCode === 0;
    result = shellResult.output;
  } else {
    const module = await import(`${PLUGIN_ROOT}/gates/${gateName}`);
    result = await module.execute(input);
    passed = !result.decision && result.continue !== false;
  }

  // Determine action
  const action = passed ?
    (gateConfig.on_pass || 'CONTINUE') :
    (gateConfig.on_fail || 'BLOCK');

  // Handle action
  return handleAction(action, result, config, input);
}
```

## Action Handling

```typescript
async function handleAction(
  action: string,
  gateResult: GateResult,
  config: GatesConfig,
  input: HookInput
): Promise<boolean> {

  switch (action) {
    case 'CONTINUE':
      if (gateResult.additionalContext) {
        outputContext(gateResult.additionalContext);
      }
      return true; // Continue to next gate

    case 'BLOCK':
      outputBlock(gateResult.reason || 'Gate failed');
      return false; // Stop execution

    case 'STOP':
      outputStop(gateResult.message || 'Gate stopped execution');
      return false; // Stop execution

    default:
      // Gate chaining - action is another gate name
      return runGateWithActions(action, config, input);
  }
}
```

**Actions:**
- `CONTINUE` - Continue to next gate (default on pass)
- `BLOCK` - Prevent agent from proceeding (default on fail)
- `STOP` - Stop Claude entirely
- `{gate_name}` - Chain to another gate (recursive call)

## Config Loading

**Priority order:**

```typescript
async function loadConfig(cwd: string): Promise<GatesConfig | null> {
  const paths = [
    `${cwd}/.claude/gates.json`,     // Project-specific (recommended)
    `${cwd}/gates.json`,             // Project root
    `${PLUGIN_ROOT}/hooks/gates.json` // Plugin default fallback
  ];

  for (const path of paths) {
    if (await fileExists(path)) {
      const content = await fs.readFile(path, 'utf-8');
      return JSON.parse(content);
    }
  }

  return null;
}
```

## Convention-Based Context Injection

```typescript
async function injectContext(
  hookEvent: string,
  input: HookInput,
  cwd: string
): Promise<void> {

  let name: string | undefined;
  let stage: string | undefined;

  if (hookEvent === 'SlashCommandStart' || hookEvent === 'SlashCommandEnd') {
    name = input.command?.replace(/^\//, '');
    stage = hookEvent === 'SlashCommandStart' ? 'start' : 'end';
  } else if (hookEvent === 'SkillStart' || hookEvent === 'SkillEnd') {
    name = input.skill;
    stage = hookEvent === 'SkillStart' ? 'start' : 'end';
  }

  if (!name || !stage) return;

  const contextFile = await discoverContextFile(cwd, name, stage);

  if (contextFile) {
    const content = await fs.readFile(contextFile, 'utf-8');
    outputContext(content);
  }
}

async function discoverContextFile(
  cwd: string,
  name: string,
  stage: string
): Promise<string | null> {

  const paths = [
    `${cwd}/.claude/context/${name}-${stage}.md`,
    `${cwd}/.claude/context/slash-command/${name}-${stage}.md`,
    `${cwd}/.claude/context/slash-command/${name}/${stage}.md`,
    `${cwd}/.claude/context/skill/${name}-${stage}.md`,
    `${cwd}/.claude/context/skill/${name}/${stage}.md`
  ];

  for (const path of paths) {
    if (await fileExists(path)) {
      return path;
    }
  }

  return null;
}
```

**Preserves existing bash discovery order exactly.**

## Session State Integration

The hooks-app integrates with the hooklib session state module:

```typescript
import { Session } from '../hooklib/dist';

async function updateSessionState(session: Session, input: HookInput): Promise<void> {
  const event = input.hook_event_name;

  switch (event) {
    case 'SlashCommandStart':
      await session.set('active_command', input.command);
      break;

    case 'SlashCommandEnd':
      await session.set('active_command', null);
      break;

    case 'SkillStart':
      await session.set('active_skill', input.skill);
      break;

    case 'SkillEnd':
      await session.set('active_skill', null);
      break;

    case 'PostToolUse':
      if (input.file_path) {
        await session.append('edited_files', input.file_path);
        const ext = input.file_path.split('.').pop();
        if (ext) {
          await session.append('file_extensions', ext);
        }
      }
      break;
  }
}
```

**Built-in gates can access session state:**

```typescript
// gates/dispatch-rust-engineer.ts
export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);

  const activeCommand = await session.get('active_command');
  if (activeCommand !== '/execute') {
    return { additionalContext: '' };
  }

  const hasRustFiles = await session.contains('file_extensions', 'rs');
  if (!hasRustFiles) {
    return { additionalContext: '' };
  }

  return {
    decision: 'block',
    reason: 'Rust files edited. Must use rust-engineer agent.'
  };
}
```

## Testing

### Unit Tests (Jest)

```typescript
// __tests__/gate-loader.test.ts
describe('Gate Loader', () => {
  test('executes shell command gate', async () => {
    const gateConfig = {
      command: 'echo "success"',
      on_pass: 'CONTINUE'
    };
    const result = await executeGate('test', gateConfig, mockInput);
    expect(result.passed).toBe(true);
  });

  test('loads built-in TypeScript gate', async () => {
    const gateConfig = { on_pass: 'CONTINUE' };
    const result = await executeGate('plan-compliance', gateConfig, mockInput);
    expect(result).toBeDefined();
  });
});
```

### Integration Tests (Bash)

Keep existing integration tests in `plugin/hooks/tests/`, update to use compiled app:

```bash
echo '{"hook_event_name":"PostToolUse","tool_name":"Edit","cwd":"'$TEST_DIR'"}' | \
  node ${PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js
```

## Project Structure

```
plugin/hooks/
├── hooks-app/              # Main TypeScript application
│   ├── src/
│   │   ├── cli.ts         # Entry point (reads stdin)
│   │   ├── dispatcher.ts  # Event routing & orchestration
│   │   ├── gate-loader.ts # Gate execution
│   │   ├── action-handler.ts  # Action processing
│   │   ├── config.ts      # Config loading
│   │   ├── context.ts     # Context injection
│   │   ├── types.ts       # Shared types
│   │   └── index.ts       # Exports
│   ├── __tests__/         # Unit tests
│   ├── dist/              # Compiled JavaScript
│   ├── package.json
│   ├── tsconfig.json
│   └── jest.config.js
├── hooklib/               # Session state module
│   ├── src/
│   ├── dist/
│   └── package.json
├── gates/                 # Built-in TypeScript gates
│   ├── commands.ts
│   ├── plan-compliance.ts
│   └── dispatch-rust-engineer.ts
├── tests/                 # Integration tests (bash)
├── examples/              # Example configs
└── hooks.json             # Hook registration
```

## Hook Registration

Update `plugin/hooks/hooks.json`:

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
    }]
  }
}
```

## Build Process

```json
// hooks-app/package.json
{
  "name": "@cipherpowers/hooks-app",
  "version": "1.0.0",
  "main": "dist/cli.js",
  "scripts": {
    "build": "tsc",
    "watch": "tsc --watch",
    "test": "jest",
    "clean": "rm -rf dist"
  }
}
```

## Benefits

1. **Type Safety** - TypeScript validates gate implementations and config
2. **Testability** - Pure async functions, easy to unit test
3. **Maintainability** - Single codebase, clear module boundaries
4. **Performance** - No process spawning for built-in gates
5. **Session State** - Native integration with hooklib module
6. **Convention Over Configuration** - Follows existing patterns exactly
7. **Simplicity** - Projects customize via shell commands in JSON config

## Implementation Notes

- Build hooks-app: `cd plugin/hooks/hooks-app && npm install && npm run build`
- Old bash system stays in git history
- Preserves exact behavior of current system
- No migration needed - fresh implementation
