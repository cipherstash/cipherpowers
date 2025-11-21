// plugin/hooks/hooks-app/__tests__/gate-loader.test.ts
import { executeShellCommand } from '../src/gate-loader';
import * as path from 'path';
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
});
