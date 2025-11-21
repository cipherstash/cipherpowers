// plugin/hooks/hooks-app/src/config.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { GatesConfig } from './types';
import { fileExists } from './utils';

const KNOWN_HOOK_EVENTS = [
  'PostToolUse',
  'SubagentStop',
  'UserPromptSubmit',
  'SlashCommandStart',
  'SlashCommandEnd',
  'SkillStart',
  'SkillEnd'
];

const KNOWN_ACTIONS = ['CONTINUE', 'BLOCK', 'STOP'];

/**
 * Validate config invariants to catch configuration errors early.
 * Throws descriptive errors when invariants are violated.
 */
export function validateConfig(config: GatesConfig): void {
  // Invariant: Hook event names must be known types
  for (const hookName of Object.keys(config.hooks)) {
    if (!KNOWN_HOOK_EVENTS.includes(hookName)) {
      throw new Error(`Unknown hook event: ${hookName}. Must be one of: ${KNOWN_HOOK_EVENTS.join(', ')}`);
    }
  }

  // Invariant: Gates referenced in hooks must exist in gates config
  for (const [hookName, hookConfig] of Object.entries(config.hooks)) {
    if (hookConfig.gates) {
      for (const gateName of hookConfig.gates) {
        if (!config.gates[gateName]) {
          throw new Error(`Hook '${hookName}' references undefined gate '${gateName}'`);
        }
      }
    }
  }

  // Invariant: Gate actions must be CONTINUE/BLOCK/STOP or reference existing gates
  for (const [gateName, gateConfig] of Object.entries(config.gates)) {
    for (const action of [gateConfig.on_pass, gateConfig.on_fail]) {
      if (action && !KNOWN_ACTIONS.includes(action) && !config.gates[action]) {
        throw new Error(`Gate '${gateName}' action '${action}' is not CONTINUE/BLOCK/STOP or valid gate name`);
      }
    }
  }
}

export async function loadConfig(cwd: string): Promise<GatesConfig | null> {
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || '';

  const paths = [
    path.join(cwd, '.claude', 'gates.json'),
    path.join(cwd, 'gates.json'),
    path.join(pluginRoot, 'hooks', 'gates.json')
  ];

  for (const configPath of paths) {
    if (await fileExists(configPath)) {
      const content = await fs.readFile(configPath, 'utf-8');
      const config = JSON.parse(content);
      validateConfig(config);
      return config;
    }
  }

  return null;
}
