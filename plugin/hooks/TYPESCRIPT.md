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
