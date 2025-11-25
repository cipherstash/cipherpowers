"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.validateConfig = validateConfig;
exports.loadConfig = loadConfig;
// plugin/hooks/hooks-app/src/config.ts
const fs = __importStar(require("fs/promises"));
const path = __importStar(require("path"));
const utils_1 = require("./utils");
const logger_1 = require("./logger");
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
function validateConfig(config) {
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
/**
 * Get the plugin root directory from CLAUDE_PLUGIN_ROOT env var.
 * Falls back to computing relative to this file's location.
 */
function getPluginRoot() {
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
    }
    catch {
        return null;
    }
}
/**
 * Load a single config file
 */
async function loadConfigFile(configPath) {
    if (await (0, utils_1.fileExists)(configPath)) {
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
function mergeConfigs(pluginConfig, projectConfig) {
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
async function loadConfig(cwd) {
    const pluginRoot = getPluginRoot();
    // Load plugin config first (defaults)
    let mergedConfig = null;
    if (pluginRoot) {
        const pluginConfigPath = path.join(pluginRoot, 'hooks', 'gates.json');
        const pluginConfig = await loadConfigFile(pluginConfigPath);
        if (pluginConfig) {
            await logger_1.logger.debug('Loaded plugin gates.json', { path: pluginConfigPath });
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
            await logger_1.logger.debug('Loaded project gates.json', { path: configPath });
            if (mergedConfig) {
                mergedConfig = mergeConfigs(mergedConfig, projectConfig);
                await logger_1.logger.debug('Merged project config with plugin config');
            }
            else {
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
