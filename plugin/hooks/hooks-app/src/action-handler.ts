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
