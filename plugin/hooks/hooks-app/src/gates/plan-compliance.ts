// plugin/hooks/hooks-app/src/gates/plan-compliance.ts
import { HookInput, GateResult } from '../types';

/**
 * Plan Compliance Gate
 *
 * Checks that agents provide STATUS and handles BLOCKED reports.
 * Invoked automatically on all SubagentStop events to ensure:
 * 1. Agent provided STATUS (OK or BLOCKED)
 * 2. If BLOCKED, block execution for dispatcher to handle
 * 3. If OK, allow execution to continue
 *
 * This is a built-in gate that always runs automatically on SubagentStop.
 * No configuration needed.
 */

export async function execute(input: HookInput): Promise<GateResult> {
  const agentOutput = input.output || '';

  // If no output, just continue (agent may not be doing plan execution)
  if (!agentOutput) {
    return {
      additionalContext: ''
    };
  }

  // Check if STATUS is present
  if (!agentOutput.includes('STATUS:')) {
    // No STATUS found - block execution
    return {
      decision: 'block',
      reason:
        'Agent must provide STATUS in completion report. Use STATUS: OK when task complete or STATUS: BLOCKED when plan cannot be followed.',
      continue: false
    };
  }

  // Check if STATUS: BLOCKED
  if (agentOutput.includes('STATUS: BLOCKED')) {
    // Extract BLOCKED reason if possible
    const reasonMatch = agentOutput.match(/REASON:\s*(.+)/);
    const reason = reasonMatch ? reasonMatch[1].trim() : 'No reason provided';

    // Block execution - dispatcher will handle escalation to user
    return {
      decision: 'block',
      reason: `Agent reported BLOCKED. Review required before proceeding.\n\n${reason}`,
      continue: false
    };
  }

  // STATUS: OK - continue execution
  return {
    additionalContext: 'Task completed successfully (STATUS: OK)'
  };
}
