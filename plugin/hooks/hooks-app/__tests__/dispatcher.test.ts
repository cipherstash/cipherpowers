// plugin/hooks/hooks-app/__tests__/dispatcher.test.ts
import { shouldProcessHook } from '../src/dispatcher';
import { HookInput, HookConfig } from '../src/types';

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
