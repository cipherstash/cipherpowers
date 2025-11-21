// plugin/hooks/hooks-app/__tests__/dispatcher.test.ts
import { shouldProcessHook, dispatch } from '../src/dispatcher';
import { HookInput, HookConfig } from '../src/types';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';

describe('Dispatcher - Event Filtering', () => {
  test('PostToolUse with enabled tool returns true', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test',
      tool_name: 'Edit'
    };

    const hookConfig: HookConfig = {
      enabled_tools: ['Edit', 'Write']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('PostToolUse with disabled tool returns false', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test',
      tool_name: 'Read'
    };

    const hookConfig: HookConfig = {
      enabled_tools: ['Edit', 'Write']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(false);
  });

  test('SubagentStop with enabled agent returns true', () => {
    const input: HookInput = {
      hook_event_name: 'SubagentStop',
      cwd: '/test',
      agent_name: 'cipherpowers:coder'
    };

    const hookConfig: HookConfig = {
      enabled_agents: ['cipherpowers:coder']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('SubagentStop with disabled agent returns false', () => {
    const input: HookInput = {
      hook_event_name: 'SubagentStop',
      cwd: '/test',
      agent_name: 'other-agent'
    };

    const hookConfig: HookConfig = {
      enabled_agents: ['cipherpowers:coder']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(false);
  });

  test('SubagentStop checks subagent_name if agent_name missing', () => {
    const input: HookInput = {
      hook_event_name: 'SubagentStop',
      cwd: '/test',
      subagent_name: 'cipherpowers:coder'
    };

    const hookConfig: HookConfig = {
      enabled_agents: ['cipherpowers:coder']
    };

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('UserPromptSubmit always returns true', () => {
    const input: HookInput = {
      hook_event_name: 'UserPromptSubmit',
      cwd: '/test'
    };

    const hookConfig: HookConfig = {};

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });

  test('No filtering config returns true', () => {
    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: '/test',
      tool_name: 'Edit'
    };

    const hookConfig: HookConfig = {};

    expect(shouldProcessHook(input, hookConfig)).toBe(true);
  });
});

describe('Dispatcher - Gate Chaining', () => {
  let testDir: string;

  beforeEach(async () => {
    // Create temporary directory for test config
    testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'gates-test-'));
  });

  afterEach(async () => {
    // Clean up
    await fs.rm(testDir, { recursive: true, force: true });
  });

  test('gate chaining works - gate-a chains to gate-b on pass', async () => {
    // Create gates.json with chaining config
    const gatesConfig = {
      hooks: {
        PostToolUse: {
          gates: ['gate-a']
        }
      },
      gates: {
        'gate-a': {
          command: 'echo "gate-a passed"',
          on_pass: 'gate-b' // Chain to gate-b on pass
        },
        'gate-b': {
          command: 'echo "gate-b passed"',
          on_pass: 'CONTINUE'
        }
      }
    };

    await fs.writeFile(
      path.join(testDir, 'gates.json'),
      JSON.stringify(gatesConfig, null, 2)
    );

    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: testDir,
      tool_name: 'Edit'
    };

    const result = await dispatch(input);

    // Should contain output from both gates
    expect(result.context).toContain('gate-a passed');
    expect(result.context).toContain('gate-b passed');
    expect(result.blockReason).toBeUndefined();
  });

  test('circular chain prevention - exceeds max gate depth', async () => {
    // Create gates.json with circular chain
    const gatesConfig = {
      hooks: {
        PostToolUse: {
          gates: ['gate-a']
        }
      },
      gates: {
        'gate-a': {
          command: 'echo "gate-a"',
          on_pass: 'gate-b'
        },
        'gate-b': {
          command: 'echo "gate-b"',
          on_pass: 'gate-a' // Circular chain back to gate-a
        }
      }
    };

    await fs.writeFile(
      path.join(testDir, 'gates.json'),
      JSON.stringify(gatesConfig, null, 2)
    );

    const input: HookInput = {
      hook_event_name: 'PostToolUse',
      cwd: testDir,
      tool_name: 'Edit'
    };

    const result = await dispatch(input);

    // Should hit circuit breaker
    expect(result.blockReason).toContain('Exceeded max gate chain depth');
    expect(result.blockReason).toContain('circular');
  });
});
