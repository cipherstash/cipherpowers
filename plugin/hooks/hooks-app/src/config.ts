// plugin/hooks/hooks-app/src/config.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { GatesConfig, HookConfig, GateConfig } from './types';
import { fileExists } from './utils';
import { logger } from './logger';

const KNOWN_HOOK_EVENTS = [
  'PreToolUse',
  'PostToolUse',
  'SubagentStop',
  'UserPromptSubmit',
  'SlashCommandStart',
  'SlashCommandEnd',
  'SkillStart',
  'SkillEnd',
  'SessionStart',
  'SessionEnd',
  'Stop',
  'Notification'
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
      throw new Error(
        `Unknown hook event: ${hookName}. Must be one of: ${KNOWN_HOOK_EVENTS.join(', ')}`
      );
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
        throw new Error(
          `Gate '${gateName}' action '${action}' is not CONTINUE/BLOCK/STOP or valid gate name`
        );
      }
    }
  }
}

/**
 * Get the plugin root directory from CLAUDE_PLUGIN_ROOT env var.
 * Falls back to computing relative to this file's location.
 */
function getPluginRoot(): string | null {
  const envRoot = process.env.CLAUDE_PLUGIN_ROOT;
  if (envRoot) {
    return envRoot;
  }

  // Fallback: compute from this file's location
  // This file is at: plugin/hooks/hooks-app/src/config.ts (dev)
  // Or at: plugin/hooks/hooks-app/dist/config.js (built)
  // Plugin root is: plugin/
  try {
    return path.resolve(__dirname, '..', '..', '..');
  } catch {
    return null;
  }
}

/**
 * Load a single config file
 */
async function loadConfigFile(configPath: string): Promise<GatesConfig | null> {
  if (await fileExists(configPath)) {
    const content = await fs.readFile(configPath, 'utf-8');
    return JSON.parse(content);
  }
  return null;
}

/**
 * Merge two configs. Project config takes precedence over plugin config.
 * - hooks: project hooks override plugin hooks for same event
 * - gates: project gates override plugin gates for same name
 */
function mergeConfigs(pluginConfig: GatesConfig, projectConfig: GatesConfig): GatesConfig {
  return {
    hooks: {
      ...pluginConfig.hooks,
      ...projectConfig.hooks
    },
    gates: {
      ...pluginConfig.gates,
      ...projectConfig.gates
    }
  };
}

/**
 * Load and merge project and plugin configs.
 *
 * Priority:
 * 1. Project: .claude/gates.json (highest)
 * 2. Project: gates.json
 * 3. Plugin: ${CLAUDE_PLUGIN_ROOT}/hooks/gates.json (fallback/defaults)
 *
 * Configs are MERGED - project overrides plugin for same keys.
 */
export async function loadConfig(cwd: string): Promise<GatesConfig | null> {
  const pluginRoot = getPluginRoot();

  // Load plugin config first (defaults)
  let mergedConfig: GatesConfig | null = null;

  if (pluginRoot) {
    const pluginConfigPath = path.join(pluginRoot, 'hooks', 'gates.json');
    const pluginConfig = await loadConfigFile(pluginConfigPath);
    if (pluginConfig) {
      await logger.debug('Loaded plugin gates.json', { path: pluginConfigPath });
      mergedConfig = pluginConfig;
    }
  }

  // Load project config (overrides)
  const projectPaths = [
    path.join(cwd, '.claude', 'gates.json'),
    path.join(cwd, 'gates.json')
  ];

  for (const configPath of projectPaths) {
    const projectConfig = await loadConfigFile(configPath);
    if (projectConfig) {
      await logger.debug('Loaded project gates.json', { path: configPath });
      if (mergedConfig) {
        mergedConfig = mergeConfigs(mergedConfig, projectConfig);
        await logger.debug('Merged project config with plugin config');
      } else {
        mergedConfig = projectConfig;
      }
      break; // Only load first project config found
    }
  }

  // Validate merged config
  if (mergedConfig) {
    validateConfig(mergedConfig);
  }

  return mergedConfig;
}
