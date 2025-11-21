// plugin/hooks/hooks-app/__tests__/action-handler.test.ts
import { handleAction, ActionResult } from '../src/action-handler';
import { GateResult, GatesConfig } from '../src/types';

const mockConfig: GatesConfig = {
  hooks: {},
  gates: {
    'next-gate': { command: 'echo "next"', on_pass: 'CONTINUE' }
  }
};

const mockInput = {
  hook_event_name: 'PostToolUse',
  cwd: '/test'
};

describe('Action Handler', () => {
  test('CONTINUE returns continue=true', async () => {
    const result: GateResult = {};
    const action = await handleAction('CONTINUE', result, mockConfig, mockInput);

    expect(action.continue).toBe(true);
    expect(action.context).toBeUndefined();
  });

  test('CONTINUE with context returns context', async () => {
    const result: GateResult = { additionalContext: 'test context' };
    const action = await handleAction('CONTINUE', result, mockConfig, mockInput);

    expect(action.continue).toBe(true);
    expect(action.context).toBe('test context');
  });

  test('BLOCK returns continue=false', async () => {
    const result: GateResult = { decision: 'block', reason: 'test reason' };
    const action = await handleAction('BLOCK', result, mockConfig, mockInput);

    expect(action.continue).toBe(false);
    expect(action.blockReason).toBe('test reason');
  });

  test('BLOCK with no reason uses default', async () => {
    const result: GateResult = {};
    const action = await handleAction('BLOCK', result, mockConfig, mockInput);

    expect(action.continue).toBe(false);
    expect(action.blockReason).toBe('Gate failed');
  });

  test('STOP returns continue=false with stop message', async () => {
    const result: GateResult = { message: 'stop message' };
    const action = await handleAction('STOP', result, mockConfig, mockInput);

    expect(action.continue).toBe(false);
    expect(action.stopMessage).toBe('stop message');
  });
});
