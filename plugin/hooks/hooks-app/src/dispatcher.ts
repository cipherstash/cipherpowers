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
