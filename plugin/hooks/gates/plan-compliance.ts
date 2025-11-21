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
