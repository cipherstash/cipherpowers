// plugin/hooks/hooks-app/src/dispatcher.ts
import { HookInput, HookConfig } from './types';
import { loadConfig } from './config';
import { injectContext } from './context';
import { executeGate } from './gate-loader';
import { handleAction } from './action-handler';
import { Session } from './session';

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
          // Edge case: ext !== input.file_path prevents tracking entire filename
          // as extension when file has no dot (e.g., "README")
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

export async function dispatch(input: HookInput): Promise<DispatchResult> {
  const hookEvent = input.hook_event_name;
  const cwd = input.cwd;

  // Update session state (best-effort)
  await updateSessionState(input);

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
    const action = passed ? gateConfig.on_pass || 'CONTINUE' : gateConfig.on_fail || 'BLOCK';

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
