// plugin/hooks/hooks-app/__tests__/types.test.ts
import { HookInput, GateResult } from '../src/types';

describe('Types', () => {
  test('HookInput has required fields', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test/path'
    };
    expect(input.hook_event_name).toBe('PostToolUse');
    expect(input.cwd).toBe('/test/path');
  });

  test('HookInput accepts optional PostToolUse fields', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test/path',
      tool_name: 'Edit',
      file_path: '/test/file.ts'
    };
    expect(input.tool_name).toBe('Edit');
    expect(input.file_path).toBe('/test/file.ts');
  });

  test('GateResult can be empty object', () => {
    const result: GateResult = {};
    expect(result).toBeDefined();
  });

  test('GateResult can have additionalContext', () => {
    const result: GateResult = {
      additionalContext: 'Test context'
    };
    expect(result.additionalContext).toBe('Test context');
  });

  test('GateResult can have block decision', () => {
    const result: GateResult = {
      decision: 'block',
      reason: 'Test reason'
    };
    expect(result.decision).toBe('block');
    expect(result.reason).toBe('Test reason');
  });
});
