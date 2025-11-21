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
async function loadConfig(cwd) {
    const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || '';
    const paths = [path.join(cwd, '.claude', 'gates.json'), path.join(cwd, 'gates.json')];
    // Only add plugin root path if CLAUDE_PLUGIN_ROOT is set
    if (pluginRoot) {
        paths.push(path.join(pluginRoot, 'hooks', 'gates.json'));
    }
    for (const configPath of paths) {
        if (await (0, utils_1.fileExists)(configPath)) {
            const content = await fs.readFile(configPath, 'utf-8');
            const config = JSON.parse(content);
            validateConfig(config);
            return config;
        }
    }
    return null;
}
