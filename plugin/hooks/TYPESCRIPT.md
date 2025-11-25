# TypeScript Gates

Guide to creating and working with TypeScript gates in the CipherPowers hook system.

## Overview

TypeScript gates are gates defined **without a `command` field** in `gates.json`. They're implemented as TypeScript modules in `hooks-app/src/gates/`.

```json
{
  "gates": {
    "commands": {
      "description": "Context-aware command injection",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    }
  }
}
```

When this gate runs, the system loads `src/gates/commands.ts` and calls its `execute()` function.

## Built-in Gates

The plugin includes these TypeScript gates:

| Gate | Purpose | Default Hook |
|------|---------|--------------|
| `commands` | Inject project commands from CLAUDE.md when user references them | UserPromptSubmit |
| `plan-compliance` | Validate agent work follows active plan | (manual) |
| `plugin-path` | Verify plugin path resolution in subagents | (manual) |

## Creating a TypeScript Gate

### 1. Create the Gate Module

Create `hooks-app/src/gates/my-gate.ts`:

```typescript
import { HookInput, GateResult } from '../types';

/**
 * My custom gate
 *
 * Describe what this gate does and when it should be used.
 */
export async function execute(input: HookInput): Promise<GateResult> {
  // Access hook input data
  const { cwd, hook_event_name, tool_name, user_message } = input;

  // Your gate logic here
  const shouldPass = true;

  if (shouldPass) {
    return {
      additionalContext: 'Gate passed - injecting this context'
    };
  } else {
    return {
      decision: 'block',
      reason: 'Gate failed because...'
    };
  }
}
```

### 2. Register in Index

Add to `hooks-app/src/gates/index.ts`:

```typescript
export * as commands from './commands';
export * as planCompliance from './plan-compliance';
export * as pluginPath from './plugin-path';
export * as myGate from './my-gate';  // Add this line
```

**Note:** Gate name in `gates.json` uses kebab-case (`my-gate`), which maps to camelCase export (`myGate`).

### 3. Add to gates.json

Add to `plugin/hooks/gates.json` (for plugin default) or project `.claude/gates.json`:

```json
{
  "gates": {
    "my-gate": {
      "description": "My custom gate",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["my-gate"]
    }
  }
}
```

### 4. Build

```bash
cd plugin/hooks/hooks-app
npm run build
```

## HookInput Interface

```typescript
interface HookInput {
  hook_event_name: string;  // "PostToolUse", "UserPromptSubmit", etc.
  cwd: string;              // Current working directory

  // PostToolUse
  tool_name?: string;       // "Edit", "Write", etc.
  file_path?: string;       // File being edited

  // SubagentStop
  agent_name?: string;      // "rust-agent", "code-review-agent", etc.
  subagent_name?: string;   // Alternative agent name field
  output?: string;          // Agent output

  // UserPromptSubmit
  user_message?: string;    // User's prompt text

  // SlashCommand/Skill
  command?: string;         // "/code-review", etc.
  skill?: string;           // "executing-plans", etc.
}
```

## GateResult Interface

```typescript
interface GateResult {
  // Success - add context and continue
  additionalContext?: string;

  // Block agent from proceeding
  decision?: 'block';
  reason?: string;

  // Stop Claude entirely
  continue?: false;
  message?: string;
}
```

### Return Values

**Pass with context injection:**
```typescript
return {
  additionalContext: 'This content is injected into the conversation'
};
```

**Pass silently:**
```typescript
return {};
```

**Block execution:**
```typescript
return {
  decision: 'block',
  reason: 'Cannot proceed because...'
};
```

**Stop Claude entirely:**
```typescript
return {
  continue: false,
  message: 'Stopping because...'
};
```

## Accessing Session State

Gates can read/write session state for cross-hook coordination:

```typescript
import { HookInput, GateResult } from '../types';
import { Session } from '../session';

export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);

  // Read state
  const activeCommand = await session.get('active_command');
  const editedFiles = await session.get('edited_files');

  // Write state
  await session.set('active_command', '/my-command');
  await session.append('edited_files', '/path/to/file.ts');

  // Check array membership
  const hasRustFiles = await session.contains('file_extensions', 'rs');

  return {};
}
```

## Using the Logger

```typescript
import { logger } from '../logger';

export async function execute(input: HookInput): Promise<GateResult> {
  await logger.debug('Gate starting', { input });
  await logger.info('Processing', { file: input.file_path });
  await logger.warn('Potential issue', { reason: '...' });
  await logger.error('Gate failed', { error: '...' });

  return {};
}
```

Logs go to `$TMPDIR/cipherpowers/hooks-YYYY-MM-DD.log`.

## Example: Commands Gate

The built-in `commands` gate shows a complete implementation:

```typescript
// src/gates/commands.ts
import { HookInput, GateResult } from '../types';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as yaml from 'js-yaml';

interface ClaudeMdFrontmatter {
  commands?: Record<string, string>;
}

async function parseClaudeMd(cwd: string): Promise<Record<string, string>> {
  const claudeMdPath = path.join(cwd, 'CLAUDE.md');
  try {
    const content = await fs.readFile(claudeMdPath, 'utf-8');
    const frontmatterMatch = content.match(/^---\s*\n([\s\S]*?)\n---/);
    if (!frontmatterMatch) return {};
    const frontmatter = yaml.load(frontmatterMatch[1]) as ClaudeMdFrontmatter;
    return frontmatter.commands || {};
  } catch {
    return {};
  }
}

function detectNeededCommands(userMessage: string): string[] {
  const needed: string[] = [];
  const lower = userMessage.toLowerCase();
  if (lower.includes('run project test command')) needed.push('test');
  if (lower.includes('run project check command')) needed.push('check');
  if (lower.includes('run project build command')) needed.push('build');
  return [...new Set(needed)];
}

export async function execute(input: HookInput): Promise<GateResult> {
  const commands = await parseClaudeMd(input.cwd);
  const needed = detectNeededCommands(input.user_message || '');

  if (needed.length === 0) return {};

  const lines = ['<project_commands>'];
  for (const cmd of needed) {
    if (commands[cmd]) {
      lines.push(`  <${cmd}>${commands[cmd]}</${cmd}>`);
    }
  }
  lines.push('</project_commands>');

  return { additionalContext: lines.join('\n') };
}
```

## Development Workflow

### Build

```bash
cd plugin/hooks/hooks-app
npm run build
```

### Test Manually

```bash
# Test a hook event
echo '{"hook_event_name": "UserPromptSubmit", "cwd": "/path/to/project", "user_message": "Run project test command"}' | \
  CLAUDE_PLUGIN_ROOT=/path/to/plugin \
  node dist/cli.js
```

### Run Tests

```bash
npm test
```

### Watch Mode

```bash
npm run build -- --watch
```

## Naming Conventions

| gates.json | TypeScript Export | File |
|------------|-------------------|------|
| `commands` | `commands` | `commands.ts` |
| `plan-compliance` | `planCompliance` | `plan-compliance.ts` |
| `plugin-path` | `pluginPath` | `plugin-path.ts` |
| `my-custom-gate` | `myCustomGate` | `my-custom-gate.ts` |

The gate loader converts kebab-case to camelCase automatically.

## Best Practices

1. **Single responsibility**: Each gate does one thing well
2. **Fast execution**: Gates run synchronously in hook flow
3. **Graceful failures**: Return empty result `{}` on non-critical errors
4. **Logging**: Use logger for debugging, not console
5. **Type safety**: Leverage TypeScript interfaces
6. **Documentation**: Add JSDoc comments explaining gate purpose
