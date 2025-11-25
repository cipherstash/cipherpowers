"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const dispatcher_1 = require("./dispatcher");
const session_1 = require("./session");
const logger_1 = require("./logger");
async function main() {
    const args = process.argv.slice(2);
    // Check if first arg is "session" - session management mode
    if (args.length > 0 && args[0] === 'session') {
        await handleSessionCommand(args.slice(1));
        return;
    }
    // Check if first arg is "log-path" - return log file path for mise tasks
    if (args.length > 0 && args[0] === 'log-path') {
        console.log(logger_1.logger.getLogFilePath());
        return;
    }
    // Check if first arg is "log-dir" - return log directory for mise tasks
    if (args.length > 0 && args[0] === 'log-dir') {
        console.log(logger_1.logger.getLogDir());
        return;
    }
    // Otherwise, hook dispatch mode (existing behavior)
    await handleHookDispatch();
}
/**
 * Type guard for SessionState keys
 */
function isSessionStateKey(key) {
    const validKeys = [
        'session_id',
        'started_at',
        'active_command',
        'active_skill',
        'edited_files',
        'file_extensions',
        'metadata'
    ];
    return validKeys.includes(key);
}
/**
 * Type guard for array keys
 */
function isArrayKey(key) {
    return key === 'edited_files' || key === 'file_extensions';
}
/**
 * Handle session management commands with proper type safety
 */
async function handleSessionCommand(args) {
    if (args.length < 1) {
        console.error('Usage: hooks-app session [get|set|append|contains|clear] ...');
        process.exit(1);
    }
    const [command, ...params] = args;
    const cwd = params[params.length - 1] || '.';
    const session = new session_1.Session(cwd);
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
                }
                else if (key === 'metadata') {
                    await session.set(key, JSON.parse(value));
                }
                else {
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
    }
    catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        await logger_1.logger.error('Session command failed', { command, error: errorMessage });
        console.error(`Session error: ${errorMessage}`);
        process.exit(1);
    }
}
/**
 * Handle hook dispatch (existing behavior)
 */
async function handleHookDispatch() {
    try {
        // Read stdin
        const chunks = [];
        for await (const chunk of process.stdin) {
            chunks.push(chunk);
        }
        const inputStr = Buffer.concat(chunks).toString('utf-8');
        // ALWAYS log hook invocation (unconditional - for debugging)
        await logger_1.logger.always('HOOK_INVOKED', {
            input_length: inputStr.length,
            input_preview: inputStr.substring(0, 200)
        });
        // Log raw input at CLI entry point
        await logger_1.logger.debug('CLI received hook input', {
            input_length: inputStr.length,
            input_preview: inputStr.substring(0, 200)
        });
        // Parse input
        let input;
        try {
            input = JSON.parse(inputStr);
        }
        catch (error) {
            await logger_1.logger.error('CLI failed to parse JSON input', {
                input_preview: inputStr.substring(0, 200),
                error: error instanceof Error ? error.message : String(error)
            });
            console.error(JSON.stringify({
                continue: false,
                message: 'Invalid JSON input'
            }));
            process.exit(1);
        }
        // Log parsed hook event
        await logger_1.logger.info('CLI dispatching hook', {
            event: input.hook_event_name,
            cwd: input.cwd,
            tool: input.tool_name,
            agent: input.agent_name,
            command: input.command,
            skill: input.skill
        });
        // Validate required fields
        if (!input.hook_event_name || !input.cwd) {
            await logger_1.logger.warn('CLI missing required fields, exiting', {
                has_event: !!input.hook_event_name,
                has_cwd: !!input.cwd
            });
            return;
        }
        // Dispatch
        const result = await (0, dispatcher_1.dispatch)(input);
        // Build output
        const output = {};
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
        // Log result
        await logger_1.logger.info('CLI hook completed', {
            event: input.hook_event_name,
            has_context: !!result.context,
            has_block: !!result.blockReason,
            has_stop: !!result.stopMessage,
            output_keys: Object.keys(output)
        });
        // Write output
        if (Object.keys(output).length > 0) {
            console.log(JSON.stringify(output));
        }
    }
    catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        await logger_1.logger.error('Hook dispatch failed', { error: errorMessage });
        console.error(JSON.stringify({
            continue: false,
            message: `Unexpected error: ${error}`
        }));
        process.exit(1);
    }
}
main();
