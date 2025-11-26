// plugin/hooks/hooks-app/__tests__/config.test.ts
import { loadConfig } from '../src/config';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';

describe('Config Loading', () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'hooks-test-'));
  });

  afterEach(async () => {
    await fs.rm(testDir, { recursive: true, force: true });
  });

  test('returns plugin defaults when no project config exists', async () => {
    // Config loader now returns plugin defaults when no project config exists
    // This provides fallback behavior without requiring every project to have gates.json
    const config = await loadConfig(testDir);
    expect(config).not.toBeNull();
    // Verify it's actually plugin defaults by checking for expected structure
    expect(config?.hooks).toBeDefined();
    expect(config?.gates).toBeDefined();
  });

  test('loads .claude/gates.json with highest priority', async () => {
    const claudeDir = path.join(testDir, '.claude');
    await fs.mkdir(claudeDir);

    const config1 = { hooks: {}, gates: { test: { command: 'claude-config' } } };
    const config2 = { hooks: {}, gates: { test: { command: 'root-config' } } };

    await fs.writeFile(path.join(claudeDir, 'gates.json'), JSON.stringify(config1));
    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(config2));

    const config = await loadConfig(testDir);
    expect(config?.gates.test.command).toBe('claude-config');
  });

  test('loads gates.json from root when .claude does not exist', async () => {
    const config1 = { hooks: {}, gates: { test: { command: 'root-config' } } };
    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(config1));

    const config = await loadConfig(testDir);
    expect(config?.gates.test.command).toBe('root-config');
  });

  test('parses valid JSON config', async () => {
    const configObj = {
      hooks: {
        PostToolUse: {
          enabled_tools: ['Edit', 'Write'],
          gates: ['format', 'test']
        }
      },
      gates: {
        format: { command: 'npm run format', on_pass: 'CONTINUE' },
        test: { command: 'npm test', on_pass: 'CONTINUE' }
      }
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    const config = await loadConfig(testDir);
    expect(config?.hooks.PostToolUse.enabled_tools).toEqual(['Edit', 'Write']);
    expect(config?.gates.format.command).toBe('npm run format');
  });

  test('rejects unknown hook event', async () => {
    const configObj = {
      hooks: {
        UnknownEvent: { gates: [] }
      },
      gates: {}
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    await expect(loadConfig(testDir)).rejects.toThrow('Unknown hook event');
  });

  test('rejects undefined gate reference', async () => {
    const configObj = {
      hooks: {
        PostToolUse: { gates: ['nonexistent'] }
      },
      gates: {}
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    await expect(loadConfig(testDir)).rejects.toThrow('references undefined gate');
  });

  test('rejects invalid action', async () => {
    const configObj = {
      hooks: {
        PostToolUse: { gates: ['test'] }
      },
      gates: {
        test: { command: 'echo test', on_pass: 'INVALID' }
      }
    };

    await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));

    await expect(loadConfig(testDir)).rejects.toThrow(
      'is not CONTINUE/BLOCK/STOP or valid gate name'
    );
  });
});
