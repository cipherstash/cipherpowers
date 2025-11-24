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
  const validKeys = [
    'session_id',
    'started_at',
    'active_command',
    'active_skill',
    'edited_files',
    'file_extensions',
    'metadata'
  ] as const;
  return (validKeys as readonly string[]).includes(key);
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
