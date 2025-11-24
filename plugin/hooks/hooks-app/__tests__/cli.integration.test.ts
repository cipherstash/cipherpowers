// __tests__/cli.integration.test.ts
import { spawn } from 'child_process';
import { promises as fs } from 'fs';
import * as path from 'path';
import * as os from 'os';

describe('CLI Integration', () => {
  let testDir: string;

  beforeEach(async () => {
    // Create temp directory for each test
    testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'cli-test-'));
  });

  afterEach(async () => {
    // Clean up
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe('Session Management Mode', () => {
    const runCLI = (
      args: string[]
    ): Promise<{ stdout: string; stderr: string; exitCode: number }> => {
      return new Promise((resolve, reject) => {
        const proc = spawn('node', ['dist/cli.js', ...args], {
          cwd: path.resolve(__dirname, '..')
        });

        let stdout = '';
        let stderr = '';

        proc.stdout.on('data', (data) => {
          stdout += data.toString();
        });

        proc.stderr.on('data', (data) => {
          stderr += data.toString();
        });

        proc.on('close', (code) => {
          resolve({ stdout, stderr, exitCode: code ?? 0 });
        });

        proc.on('error', reject);
      });
    };

    test('should set and get active_command', async () => {
      // Set
      const setResult = await runCLI(['session', 'set', 'active_command', '/execute', testDir]);
      expect(setResult.exitCode).toBe(0);

      // Get
      const getResult = await runCLI(['session', 'get', 'active_command', testDir]);
      expect(getResult.exitCode).toBe(0);
      expect(getResult.stdout.trim()).toBe('/execute');
    });

    test('should set and get active_skill', async () => {
      // Set
      const setResult = await runCLI(['session', 'set', 'active_skill', 'brainstorming', testDir]);
      expect(setResult.exitCode).toBe(0);

      // Get
      const getResult = await runCLI(['session', 'get', 'active_skill', testDir]);
      expect(getResult.exitCode).toBe(0);
      expect(getResult.stdout.trim()).toBe('brainstorming');
    });

    test('should append to edited_files', async () => {
      // Append
      const append1 = await runCLI(['session', 'append', 'edited_files', 'file1.ts', testDir]);
      expect(append1.exitCode).toBe(0);

      const append2 = await runCLI(['session', 'append', 'edited_files', 'file2.ts', testDir]);
      expect(append2.exitCode).toBe(0);

      // Check contains
      const contains1 = await runCLI(['session', 'contains', 'edited_files', 'file1.ts', testDir]);
      expect(contains1.exitCode).toBe(0);

      const contains2 = await runCLI(['session', 'contains', 'edited_files', 'file2.ts', testDir]);
      expect(contains2.exitCode).toBe(0);

      const notContains = await runCLI([
        'session',
        'contains',
        'edited_files',
        'file3.ts',
        testDir
      ]);
      expect(notContains.exitCode).toBe(1);
    });

    test('should append to file_extensions', async () => {
      // Append
      const append1 = await runCLI(['session', 'append', 'file_extensions', 'ts', testDir]);
      expect(append1.exitCode).toBe(0);

      const append2 = await runCLI(['session', 'append', 'file_extensions', 'js', testDir]);
      expect(append2.exitCode).toBe(0);

      // Check contains
      const contains1 = await runCLI(['session', 'contains', 'file_extensions', 'ts', testDir]);
      expect(contains1.exitCode).toBe(0);

      const notContains = await runCLI(['session', 'contains', 'file_extensions', 'py', testDir]);
      expect(notContains.exitCode).toBe(1);
    });

    test('should clear session', async () => {
      // Set some data
      await runCLI(['session', 'set', 'active_command', '/execute', testDir]);
      await runCLI(['session', 'append', 'edited_files', 'file1.ts', testDir]);

      // Clear
      const clearResult = await runCLI(['session', 'clear', testDir]);
      expect(clearResult.exitCode).toBe(0);

      // Verify cleared
      const getResult = await runCLI(['session', 'get', 'active_command', testDir]);
      expect(getResult.stdout.trim()).toBe('');
    });

    test('should reject invalid session keys', async () => {
      const result = await runCLI(['session', 'get', 'invalid_key', testDir]);
      expect(result.exitCode).toBe(1);
      expect(result.stderr).toContain('Invalid session key: invalid_key');
    });

    test('should reject invalid array keys for append', async () => {
      const result = await runCLI(['session', 'append', 'session_id', 'value', testDir]);
      expect(result.exitCode).toBe(1);
      expect(result.stderr).toContain('Invalid array key');
    });

    test('should reject setting non-settable keys', async () => {
      const result = await runCLI(['session', 'set', 'session_id', 'value', testDir]);
      expect(result.exitCode).toBe(1);
      expect(result.stderr).toContain('Cannot set session_id');
    });

    test('should show usage for missing arguments', async () => {
      const result = await runCLI(['session']);
      expect(result.exitCode).toBe(1);
      expect(result.stderr).toContain('Usage:');
    });

    test('should show error for unknown session command', async () => {
      const result = await runCLI(['session', 'unknown', testDir]);
      expect(result.exitCode).toBe(1);
      expect(result.stderr).toContain('Unknown session command');
    });
  });

  describe('Hook Dispatch Mode', () => {
    test('should handle hook dispatch with valid JSON input', (done) => {
      const proc = spawn('node', ['dist/cli.js'], {
        cwd: path.resolve(__dirname, '..')
      });

      const input = JSON.stringify({
        hook_event_name: 'PostToolUse',
        cwd: testDir,
        tool_name: 'Edit',
        tool_input: {}
      });

      let stdout = '';
      proc.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      proc.on('close', (code) => {
        expect(code).toBe(0);
        // Should produce empty output or valid JSON
        if (stdout.trim()) {
          expect(() => JSON.parse(stdout)).not.toThrow();
        }
        done();
      });

      proc.stdin.write(input);
      proc.stdin.end();
    });

    test('should handle graceful exit on missing required fields', (done) => {
      const proc = spawn('node', ['dist/cli.js'], {
        cwd: path.resolve(__dirname, '..')
      });

      const input = JSON.stringify({
        // Missing hook_event_name and cwd
        tool_name: 'Edit'
      });

      proc.on('close', (code) => {
        expect(code).toBe(0); // Graceful exit
        done();
      });

      proc.stdin.write(input);
      proc.stdin.end();
    });

    test('should handle invalid JSON input', (done) => {
      const proc = spawn('node', ['dist/cli.js'], {
        cwd: path.resolve(__dirname, '..')
      });

      let stderr = '';
      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      proc.on('close', (code) => {
        expect(code).toBe(1);
        expect(stderr).toContain('Invalid JSON input');
        done();
      });

      proc.stdin.write('not valid json');
      proc.stdin.end();
    });
  });
});
