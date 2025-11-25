import { Session } from '../src/session';
import { promises as fs } from 'fs';
import { join, dirname } from 'path';
import { tmpdir } from 'os';

describe('Session', () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = join(tmpdir(), `session-test-${Date.now()}`);
    await fs.mkdir(testDir, { recursive: true });
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  describe('constructor', () => {
    test('sets state file path', () => {
      const session = new Session(testDir);
      expect(session['stateFile']).toBe(join(testDir, '.claude', 'session', 'state.json'));
    });
  });

  describe('get/set', () => {
    test('set and get scalar value', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');

      const value = await session.get('active_command');
      expect(value).toBe('/execute');
    });

    test('get returns null for unset values', async () => {
      const session = new Session(testDir);
      const value = await session.get('active_skill');
      expect(value).toBeNull();
    });

    test('set multiple values independently', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');
      await session.set('active_skill', 'executing-plans');

      expect(await session.get('active_command')).toBe('/execute');
      expect(await session.get('active_skill')).toBe('executing-plans');
    });
  });

  describe('append/contains', () => {
    test('append adds value to array', async () => {
      const session = new Session(testDir);
      await session.append('edited_files', 'main.ts');
      await session.append('edited_files', 'lib.ts');

      const files = await session.get('edited_files');
      expect(files).toEqual(['main.ts', 'lib.ts']);
    });

    test('append deduplicates values', async () => {
      const session = new Session(testDir);
      await session.append('edited_files', 'main.ts');
      await session.append('edited_files', 'lib.ts');
      await session.append('edited_files', 'main.ts'); // Duplicate

      const files = await session.get('edited_files');
      expect(files).toEqual(['main.ts', 'lib.ts']);
    });

    test('contains returns true for existing value', async () => {
      const session = new Session(testDir);
      await session.append('file_extensions', 'ts');
      await session.append('file_extensions', 'js');

      expect(await session.contains('file_extensions', 'ts')).toBe(true);
      expect(await session.contains('file_extensions', 'js')).toBe(true);
    });

    test('contains returns false for missing value', async () => {
      const session = new Session(testDir);
      await session.append('file_extensions', 'ts');

      expect(await session.contains('file_extensions', 'rs')).toBe(false);
    });
  });

  describe('clear', () => {
    test('removes state file', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');

      const stateFile = join(testDir, '.claude', 'session', 'state.json');
      const exists = await fs
        .access(stateFile)
        .then(() => true)
        .catch(() => false);
      expect(exists).toBe(true);

      await session.clear();

      const existsAfter = await fs
        .access(stateFile)
        .then(() => true)
        .catch(() => false);
      expect(existsAfter).toBe(false);
    });

    test('is safe when file does not exist', async () => {
      const session = new Session(testDir);
      await expect(session.clear()).resolves.not.toThrow();
    });
  });

  describe('persistence', () => {
    test('state persists across Session instances', async () => {
      const session1 = new Session(testDir);
      await session1.set('active_command', '/plan');
      await session1.append('edited_files', 'main.ts');

      const session2 = new Session(testDir);
      expect(await session2.get('active_command')).toBe('/plan');
      expect(await session2.get('edited_files')).toEqual(['main.ts']);
    });
  });

  describe('atomic writes', () => {
    test('uses atomic rename', async () => {
      const session = new Session(testDir);
      await session.set('active_command', '/execute');

      const stateFile = join(testDir, '.claude', 'session', 'state.json');
      const tempFile = stateFile + '.tmp';

      // Temp file should not exist after save completes
      const tempExists = await fs
        .access(tempFile)
        .then(() => true)
        .catch(() => false);
      expect(tempExists).toBe(false);

      // State file should exist
      const stateExists = await fs
        .access(stateFile)
        .then(() => true)
        .catch(() => false);
      expect(stateExists).toBe(true);
    });
  });

  describe('error scenarios', () => {
    test('handles corrupted JSON gracefully', async () => {
      const session = new Session(testDir);
      const stateFile = join(testDir, '.claude', 'session', 'state.json');

      // Create directory and write corrupted JSON
      await fs.mkdir(dirname(stateFile), { recursive: true });
      await fs.writeFile(stateFile, '{invalid json', 'utf-8');

      // Should reinitialize state on corruption
      const value = await session.get('active_command');
      expect(value).toBeNull();
    });

    test('handles cross-process persistence', async () => {
      // Simulate separate process invocations
      const session1 = new Session(testDir);
      await session1.set('active_command', '/execute');
      await session1.append('edited_files', 'main.ts');

      // Create new session instance (simulates new process)
      const session2 = new Session(testDir);
      expect(await session2.get('active_command')).toBe('/execute');
      expect(await session2.get('edited_files')).toEqual(['main.ts']);
    });

    test('handles concurrent writes via atomic rename', async () => {
      const session = new Session(testDir);

      // Rapid concurrent writes (atomic rename prevents corruption)
      // Note: Some writes may fail due to temp file conflicts, but state file
      // should never be corrupted (that's what atomic rename protects against)
      const results = await Promise.allSettled([
        session.append('edited_files', 'file1.ts'),
        session.append('edited_files', 'file2.ts'),
        session.append('edited_files', 'file3.ts')
      ]);

      // At least one operation should succeed
      const successCount = results.filter((r) => r.status === 'fulfilled').length;
      expect(successCount).toBeGreaterThan(0);

      // State file should be valid (not corrupted)
      const files = await session.get('edited_files');
      expect(Array.isArray(files)).toBe(true);
      expect(files.length).toBeGreaterThan(0);
    });
  });
});
