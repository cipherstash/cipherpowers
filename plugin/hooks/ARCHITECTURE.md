# Hook System Architecture

The CipherPowers hook system is a **self-referential TypeScript application** that uses its own configuration format to define default behaviors. The plugin is built on itself.

## Core Concept

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Claude Code Hook Event                          │
│         (PostToolUse, SubagentStop, UserPromptSubmit, etc.)            │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          hooks.json Registration                         │
│              Routes ALL hook events to TypeScript CLI                    │
│         node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js          │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           TypeScript CLI                                 │
│                     plugin/hooks/hooks-app/src/cli.ts                   │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
┌───────────────────────────────┐   ┌───────────────────────────────────┐
│      Context Injection        │   │         Config Loading             │
│  (PRIMARY - always runs)      │   │   (loads + merges gates.json)     │
│                               │   │                                   │
│  1. Project .claude/context/  │   │  1. Plugin gates.json (defaults)  │
│  2. Plugin context/ (fallback)│   │  2. Project gates.json (override) │
└───────────────────────────────┘   └───────────────────────────────────┘
                                                    │
                                                    ▼
                                    ┌───────────────────────────────────┐
                                    │          Gate Execution            │
                                    │                                   │
                                    │  Shell command gates (command:)   │
                                    │  TypeScript gates (no command:)   │
                                    └───────────────────────────────────┘
                                                    │
                                                    ▼
                                    ┌───────────────────────────────────┐
                                    │          Action Handling           │
                                    │                                   │
                                    │  CONTINUE → proceed               │
                                    │  BLOCK → prevent agent action     │
                                    │  STOP → halt Claude entirely      │
                                    │  {gate} → chain to another gate   │
                                    └───────────────────────────────────┘
```

## Self-Referential Design

The hook system uses **its own gates.json** to configure default behaviors:

```
plugin/hooks/gates.json          ← Plugin defaults (TypeScript gates)
        ↓ merged with
.claude/gates.json               ← Project overrides (user configuration)
        ↓
Merged Configuration             ← Project takes precedence
```

### Plugin gates.json

```json
{
  "gates": {
    "commands": {
      "description": "Context-aware command injection",
      "on_pass": "CONTINUE",
      "on_fail": "CONTINUE"
    },
    "plan-compliance": { ... },
    "plugin-path": { ... }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["commands"]
    }
  }
}
```

**TypeScript gates** have no `command` field - they're implemented in `src/gates/`.

**Shell command gates** have a `command` field - they execute shell commands.

## Directory Structure

```
plugin/hooks/
├── hooks.json                  # Hook registration (routes to CLI)
├── gates.json                  # Plugin default gates configuration
├── ARCHITECTURE.md             # This file
├── CONVENTIONS.md              # Context file naming conventions
├── README.md                   # Quick start guide
├── SETUP.md                    # Detailed setup instructions
├── TYPESCRIPT.md               # TypeScript gate development
│
├── context/                    # Plugin-level context files (NEW)
│   └── session-start.md        # Injects on SessionStart
│
├── hooks-app/                  # TypeScript application
│   ├── src/
│   │   ├── cli.ts              # Entry point
│   │   ├── dispatcher.ts       # Main dispatch logic
│   │   ├── context.ts          # Context file discovery/injection
│   │   ├── config.ts           # Config loading/merging
│   │   ├── gate-loader.ts      # Gate execution
│   │   ├── action-handler.ts   # Action processing
│   │   ├── session.ts          # Session state management
│   │   ├── logger.ts           # Debug logging
│   │   ├── types.ts            # TypeScript interfaces
│   │   ├── utils.ts            # Utility functions
│   │   └── gates/              # Built-in TypeScript gates
│   │       ├── index.ts        # Gate registry
│   │       ├── commands.ts     # Command injection gate
│   │       ├── plan-compliance.ts
│   │       └── plugin-path.ts
│   └── dist/                   # Compiled JavaScript
│
└── examples/
    ├── context/                # Example context files
    ├── strict.json             # Example: strict mode
    ├── permissive.json         # Example: warn only
    └── pipeline.json           # Example: gate chaining
```

## Execution Flow

### 1. Hook Event Received

Claude Code fires a hook event (e.g., `UserPromptSubmit`). The `hooks.json` routes it to the TypeScript CLI:

```json
{
  "hooks": {
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

### 2. Context Injection (Primary Behavior)

**Always runs first.** Discovers and injects markdown content from:

1. **Project context** (highest priority):
   - `.claude/context/{name}-{stage}.md`
   - `.claude/context/slash-command/{name}-{stage}.md`
   - `.claude/context/skill/{name}-{stage}.md`

2. **Plugin context** (fallback):
   - `${CLAUDE_PLUGIN_ROOT}/context/{name}-{stage}.md`
   - (same variations as project)

### 3. Config Loading and Merging

Loads both configs and merges them:

```typescript
// Load plugin defaults first
const pluginConfig = await loadConfigFile(`${CLAUDE_PLUGIN_ROOT}/hooks/gates.json`);

// Load project overrides
const projectConfig = await loadConfigFile('.claude/gates.json');

// Merge: project overrides plugin
const mergedConfig = {
  hooks: { ...pluginConfig.hooks, ...projectConfig.hooks },
  gates: { ...pluginConfig.gates, ...projectConfig.gates }
};
```

### 4. Gate Execution

For each gate in the hook's `gates` array:

**TypeScript Gate** (no `command` field):
```typescript
// Gate name maps to module: "commands" → gates/commands.ts
const gates = await import('./gates');
const result = await gates.commands.execute(input);
```

**Shell Command Gate** (has `command` field):
```typescript
const { stdout, stderr } = await exec(gateConfig.command, { cwd });
```

### 5. Action Handling

Based on gate result and `on_pass`/`on_fail` configuration:

- **CONTINUE**: Proceed to next gate or complete
- **BLOCK**: Return block decision to Claude Code
- **STOP**: Return stop signal to halt Claude
- **{gate_name}**: Chain to another gate

## Supported Hook Events

All 12 Claude Code hook types are supported:

| Event | Context Pattern | Description |
|-------|----------------|-------------|
| `SessionStart` | `session-start.md` | Beginning of Claude session |
| `SessionEnd` | `session-end.md` | End of Claude session |
| `UserPromptSubmit` | `prompt-submit.md` | User submits prompt |
| `SlashCommandStart` | `{command}-start.md` | Command begins |
| `SlashCommandEnd` | `{command}-end.md` | Command completes |
| `SkillStart` | `{skill}-start.md` | Skill loads |
| `SkillEnd` | `{skill}-end.md` | Skill completes |
| `SubagentStop` | `{agent}-end.md` | Agent completes |
| `PreToolUse` | `{tool}-pre.md` | Before tool executes |
| `PostToolUse` | `{tool}-post.md` | After tool executes |
| `Stop` | `agent-stop.md` | Agent stops |
| `Notification` | `notification-receive.md` | Notification received |

## TypeScript Gates

Gates without a `command` field are TypeScript gates. They're implemented in `src/gates/` and export an `execute` function:

```typescript
// src/gates/commands.ts
import { HookInput, GateResult } from '../types';

export async function execute(input: HookInput): Promise<GateResult> {
  // Gate logic here
  return {
    additionalContext: '...'  // Inject content
    // or
    decision: 'block',
    reason: '...'             // Block execution
    // or
    continue: false,
    message: '...'            // Stop Claude
  };
}
```

Register in `src/gates/index.ts`:
```typescript
export * as commands from './commands';
export * as planCompliance from './plan-compliance';
```

Gate name maps to export: `"commands"` → `gates.commands.execute()`

## Configuration Merging

**Project configuration overrides plugin configuration at the key level:**

```json
// Plugin gates.json (defaults)
{
  "hooks": {
    "UserPromptSubmit": { "gates": ["commands"] },
    "PostToolUse": { "gates": ["check"] }
  },
  "gates": {
    "commands": { "on_pass": "CONTINUE" },
    "check": { "command": "echo placeholder" }
  }
}

// Project .claude/gates.json (overrides)
{
  "hooks": {
    "PostToolUse": { "gates": ["lint", "test"] }  // Replaces plugin's PostToolUse
  },
  "gates": {
    "check": { "command": "npm run lint" },       // Replaces plugin's check
    "lint": { "command": "eslint ." },            // New gate
    "test": { "command": "npm test" }             // New gate
  }
}

// Merged result
{
  "hooks": {
    "UserPromptSubmit": { "gates": ["commands"] }, // From plugin
    "PostToolUse": { "gates": ["lint", "test"] }   // From project (replaced)
  },
  "gates": {
    "commands": { "on_pass": "CONTINUE" },         // From plugin
    "check": { "command": "npm run lint" },        // From project (replaced)
    "lint": { "command": "eslint ." },             // From project (new)
    "test": { "command": "npm test" }              // From project (new)
  }
}
```

## Session State

The hook system maintains session state for cross-hook coordination:

```typescript
interface SessionState {
  session_id: string;           // Unique session ID
  started_at: string;           // ISO timestamp
  active_command: string | null; // Current slash command
  active_skill: string | null;   // Current skill
  edited_files: string[];        // Files modified this session
  file_extensions: string[];     // Extensions edited
  metadata: Record<string, any>; // Custom data
}
```

State persists in `$TMPDIR/cipherpowers/session-{cwd-hash}.json`.

## Logging

All hook invocations are logged to `$TMPDIR/cipherpowers/hooks-YYYY-MM-DD.log`:

```bash
# View logs
tail -f $(node plugin/hooks/hooks-app/dist/cli.js log-path)

# Or use mise task
mise run logs
```

Log entries include:
- Hook event type
- Config file paths loaded
- Gate execution results
- Action handling decisions
- Timing information

## Benefits of Self-Referential Design

1. **Dogfooding**: Plugin uses its own infrastructure
2. **Consistent Patterns**: Same config format for plugin and projects
3. **Testable**: TypeScript gates are unit-testable
4. **Debuggable**: Standard TypeScript tooling works
5. **Extensible**: Add new TypeScript gates, projects can override
6. **Type-Safe**: Full TypeScript type checking
