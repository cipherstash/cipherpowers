// plugin/hooks/hooks-app/__tests__/dispatcher.test.ts
import { shouldProcessHook, dispatch, gateMatchesKeywords } from '../src/dispatcher';
import { HookInput, HookConfig, GateConfig } from '../src/types';
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

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(gatesConfig, null, 2));

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

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(gatesConfig, null, 2));

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

describe('Keyword Matching', () => {
  test('no keywords - gate always runs', () => {
    const gateConfig: GateConfig = {
      command: 'npm test'
    };

    expect(gateMatchesKeywords(gateConfig, 'hello world')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, undefined)).toBe(true);
    expect(gateMatchesKeywords(gateConfig, '')).toBe(true);
  });

  test('empty keywords array - gate always runs', () => {
    const gateConfig: GateConfig = {
      command: 'npm test',
      keywords: []
    };

    expect(gateMatchesKeywords(gateConfig, 'hello world')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, undefined)).toBe(true);
  });

  test('no user message with keywords - gate does not run', () => {
    const gateConfig: GateConfig = {
      command: 'npm test',
      keywords: ['test', 'testing']
    };

    expect(gateMatchesKeywords(gateConfig, undefined)).toBe(false);
    expect(gateMatchesKeywords(gateConfig, '')).toBe(false);
  });

  test('keyword match - case insensitive', () => {
    const gateConfig: GateConfig = {
      command: 'npm test',
      keywords: ['test']
    };

    expect(gateMatchesKeywords(gateConfig, 'run the TEST')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'RUN THE Test')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'test this')).toBe(true);
  });

  test('multiple keywords - any matches', () => {
    const gateConfig: GateConfig = {
      command: 'npm test',
      keywords: ['test', 'testing', 'spec', 'verify']
    };

    expect(gateMatchesKeywords(gateConfig, 'run the tests')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'verify this works')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'check the spec')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'we are testing')).toBe(true);
  });

  test('no keyword match - gate does not run', () => {
    const gateConfig: GateConfig = {
      command: 'npm test',
      keywords: ['test', 'testing']
    };

    expect(gateMatchesKeywords(gateConfig, 'hello world')).toBe(false);
    expect(gateMatchesKeywords(gateConfig, 'run the linter')).toBe(false);
  });

  test('substring matching - partial word matches', () => {
    const gateConfig: GateConfig = {
      command: 'npm test',
      keywords: ['test']
    };

    // Intentional substring matching (not word-boundary)
    expect(gateMatchesKeywords(gateConfig, 'latest version')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'contest results')).toBe(true);
    expect(gateMatchesKeywords(gateConfig, 'testing')).toBe(true);
  });
});
