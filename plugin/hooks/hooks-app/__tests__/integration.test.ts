import { exec } from 'child_process';
import { promisify } from 'util';
import { join, dirname } from 'path';
import { promises as fs } from 'fs';
import { tmpdir } from 'os';

const execAsync = promisify(exec);

describe('Integration Tests', () => {
  let testDir: string;
  let cliPath: string;

  beforeEach(async () => {
    testDir = join(tmpdir(), `integration-test-${Date.now()}`);
    await fs.mkdir(testDir, { recursive: true });
    cliPath = join(__dirname, '../dist/cli.js');
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe('Session Management', () => {
    test('set and get command', async () => {
      await execAsync(`node ${cliPath} session set active_command /execute ${testDir}`);
      const { stdout } = await execAsync(`node ${cliPath} session get active_command ${testDir}`);
      expect(stdout.trim()).toBe('/execute');
    });

    test('append and check contains', async () => {
      await execAsync(`node ${cliPath} session append file_extensions ts ${testDir}`);

      const result = await execAsync(`node ${cliPath} session contains file_extensions ts ${testDir}`)
        .then(() => true)
        .catch(() => false);

      expect(result).toBe(true);
    });

    test('clear removes state', async () => {
      await execAsync(`node ${cliPath} session set active_command /plan ${testDir}`);
      await execAsync(`node ${cliPath} session clear ${testDir}`);

      const { stdout } = await execAsync(`node ${cliPath} session get active_command ${testDir}`);
      expect(stdout.trim()).toBe('');
    });
  });

  describe('Hook Dispatch with Session Tracking', () => {
    test('PostToolUse updates session', async () => {
      const hookInput = JSON.stringify({
        hook_event_name: 'PostToolUse',
        tool_name: 'Edit',
        file_path: 'main.ts',
        cwd: testDir
      });

      await execAsync(`echo '${hookInput}' | node ${cliPath}`);

      const { stdout: files } = await execAsync(
        `node ${cliPath} session get edited_files ${testDir}`
      );
      expect(files).toContain('main.ts');

      const containsTs = await execAsync(
        `node ${cliPath} session contains file_extensions ts ${testDir}`
      ).then(() => true).catch(() => false);
      expect(containsTs).toBe(true);
    });

    test('SlashCommandStart/End updates session', async () => {
      // Start command
      const startInput = JSON.stringify({
        hook_event_name: 'SlashCommandStart',
        command: '/execute',
        cwd: testDir
      });
      await execAsync(`echo '${startInput}' | node ${cliPath}`);

      const { stdout: activeCmd } = await execAsync(
        `node ${cliPath} session get active_command ${testDir}`
      );
      expect(activeCmd.trim()).toBe('/execute');

      // End command
      const endInput = JSON.stringify({
        hook_event_name: 'SlashCommandEnd',
        command: '/execute',
        cwd: testDir
      });
      await execAsync(`echo '${endInput}' | node ${cliPath}`);

      const { stdout: cleared } = await execAsync(
        `node ${cliPath} session get active_command ${testDir}`
      );
      expect(cleared.trim()).toBe('');
    });

    test('SkillStart/End updates session', async () => {
      // Start skill
      const startInput = JSON.stringify({
        hook_event_name: 'SkillStart',
        skill: 'executing-plans',
        cwd: testDir
      });
      await execAsync(`echo '${startInput}' | node ${cliPath}`);

      const { stdout: activeSkill } = await execAsync(
        `node ${cliPath} session get active_skill ${testDir}`
      );
      expect(activeSkill.trim()).toBe('executing-plans');

      // End skill
      const endInput = JSON.stringify({
        hook_event_name: 'SkillEnd',
        skill: 'executing-plans',
        cwd: testDir
      });
      await execAsync(`echo '${endInput}' | node ${cliPath}`);

      const { stdout: cleared } = await execAsync(
        `node ${cliPath} session get active_skill ${testDir}`
      );
      expect(cleared.trim()).toBe('');
    });
  });

  describe('Error Handling', () => {
    test('handles corrupted state file gracefully', async () => {
      const stateFile = join(testDir, '.claude', 'session', 'state.json');
      await fs.mkdir(dirname(stateFile), { recursive: true });
      await fs.writeFile(stateFile, '{invalid json', 'utf-8');

      // Should reinitialize and work
      await execAsync(`node ${cliPath} session set active_command /plan ${testDir}`);
      const { stdout } = await execAsync(`node ${cliPath} session get active_command ${testDir}`);
      expect(stdout.trim()).toBe('/plan');
    });

    test('rejects invalid session keys', async () => {
      try {
        await execAsync(`node ${cliPath} session get invalid_key ${testDir}`);
        fail('Should have thrown error');
      } catch (error: any) {
        expect(error.stderr).toContain('Invalid session key');
      }
    });

    test('rejects invalid array keys for append', async () => {
      try {
        await execAsync(`node ${cliPath} session append invalid_key value ${testDir}`);
        fail('Should have thrown error');
      } catch (error: any) {
        expect(error.stderr).toContain('Invalid array key');
      }
    });
  });
});
