// plugin/hooks/hooks-app/__tests__/gate-loader.test.ts
import { executeShellCommand, executeGate } from '../src/gate-loader';
import { GateConfig, HookInput } from '../src/types';
import * as os from 'os';

describe('Gate Loader - Shell Commands', () => {
  test('executes shell command and returns exit code', async () => {
    const result = await executeShellCommand('echo "test"', process.cwd());
    expect(result.exitCode).toBe(0);
    expect(result.output).toContain('test');
  });

  test('captures non-zero exit code', async () => {
    const result = await executeShellCommand('exit 1', process.cwd());
    expect(result.exitCode).toBe(1);
  });

  test('captures stdout', async () => {
    const result = await executeShellCommand('echo "hello world"', process.cwd());
    expect(result.output).toContain('hello world');
  });

  test('captures stderr', async () => {
    const result = await executeShellCommand('echo "error" >&2', process.cwd());
    expect(result.output).toContain('error');
  });

  test('executes in specified directory', async () => {
    const tmpDir = os.tmpdir();
    const result = await executeShellCommand('pwd', tmpDir);
    // macOS may prepend /private to paths
    expect(result.output.trim()).toMatch(new RegExp(tmpDir.replace('/var/', '(/private)?/var/')));
  });

  test('timeout returns exit code 124 and timeout message', async () => {
    const result = await executeShellCommand('sleep 1', process.cwd(), 100);
    expect(result.exitCode).toBe(124);
    expect(result.output).toContain('timed out');
  });
});

describe('Gate Loader - executeGate', () => {
  const mockInput: HookInput = {
    hook_event_name: 'PostToolUse',
    cwd: process.cwd()
  };

  test('shell command gate with exit 0 returns passed=true', async () => {
    const gateConfig: GateConfig = {
      command: 'echo "success"'
    };

    const result = await executeGate('test-gate', gateConfig, mockInput);

    expect(result.passed).toBe(true);
    expect(result.result.additionalContext).toContain('success');
  });

  test('shell command gate with exit 1 returns passed=false', async () => {
    const gateConfig: GateConfig = {
      command: 'exit 1'
    };

    const result = await executeGate('test-gate', gateConfig, mockInput);

    expect(result.passed).toBe(false);
  });

  test('built-in gate throws error (not yet implemented)', async () => {
    const gateConfig: GateConfig = {
      // No command = built-in gate
    };

    await expect(executeGate('builtin-gate', gateConfig, mockInput))
      .rejects.toThrow('Built-in gate builtin-gate not yet implemented');
  });
});
