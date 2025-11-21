// plugin/hooks/hooks-app/__tests__/builtin-gates.test.ts
import { executeBuiltinGate } from '../src/gate-loader';
import { HookInput } from '../src/types';
import * as path from 'path';

// Set CLAUDE_PLUGIN_ROOT for tests to point to plugin directory
process.env.CLAUDE_PLUGIN_ROOT = path.resolve(__dirname, '../../..');

describe('Built-in Gates', () => {
  describe('plan-compliance', () => {
    test('blocks when no STATUS in output', async () => {
      const input: HookInput = {
        hook_event_name: 'SubagentStop',
        cwd: '/test',
        output: 'Task completed'
      };

      const result = await executeBuiltinGate('plan-compliance', input);
      expect(result.decision).toBe('block');
      expect(result.reason).toContain('STATUS');
    });

    test('blocks when STATUS: BLOCKED', async () => {
      const input: HookInput = {
        hook_event_name: 'SubagentStop',
        cwd: '/test',
        output: 'STATUS: BLOCKED\nSome reason'
      };

      const result = await executeBuiltinGate('plan-compliance', input);
      expect(result.decision).toBe('block');
      expect(result.reason).toContain('BLOCKED');
    });

    test('continues when STATUS: OK', async () => {
      const input: HookInput = {
        hook_event_name: 'SubagentStop',
        cwd: '/test',
        output: 'STATUS: OK\nTask completed successfully'
      };

      const result = await executeBuiltinGate('plan-compliance', input);
      expect(result.decision).toBeUndefined();
      expect(result.additionalContext).toContain('successfully');
    });
  });
});
