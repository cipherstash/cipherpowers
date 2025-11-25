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
exports.discoverContextFile = discoverContextFile;
exports.injectContext = injectContext;
// plugin/hooks/hooks-app/src/context.ts
const fs = __importStar(require("fs/promises"));
const path = __importStar(require("path"));
const utils_1 = require("./utils");
const session_1 = require("./session");
const logger_1 = require("./logger");
/**
 * Get the plugin root directory from CLAUDE_PLUGIN_ROOT env var.
 * Falls back to computing relative to this file's location.
 */
function getPluginRoot() {
    // First check env var (set by Claude Code when plugin is loaded)
    const envRoot = process.env.CLAUDE_PLUGIN_ROOT;
    if (envRoot) {
        return envRoot;
    }
    // Fallback: compute from this file's location
    // This file is at: plugin/hooks/hooks-app/src/context.ts (dev)
    // Or at: plugin/hooks/hooks-app/dist/context.js (built)
    // Plugin root is: plugin/
    try {
        // Go up from src/ or dist/ -> hooks-app/ -> hooks/ -> plugin/
        return path.resolve(__dirname, '..', '..', '..');
    }
    catch {
        return null;
    }
}
/**
 * Build context file paths for a given base directory.
 * Returns array of paths following priority order:
 * flat > slash-command subdir > slash-command nested > skill subdir > skill nested
 */
function buildContextPaths(baseDir, contextDir, name, stage) {
    return [
        path.join(baseDir, contextDir, `${name}-${stage}.md`),
        path.join(baseDir, contextDir, 'slash-command', `${name}-${stage}.md`),
        path.join(baseDir, contextDir, 'slash-command', name, `${stage}.md`),
        path.join(baseDir, contextDir, 'skill', `${name}-${stage}.md`),
        path.join(baseDir, contextDir, 'skill', name, `${stage}.md`)
    ];
}
/**
 * Discover context file following priority order.
 *
 * Priority (project takes precedence over plugin):
 * 1. Project: .claude/context/{name}-{stage}.md (and variations)
 * 2. Plugin: ${CLAUDE_PLUGIN_ROOT}/context/{name}-{stage}.md (and variations)
 */
async function discoverContextFile(cwd, name, stage) {
    // Project-level context (highest priority)
    const projectPaths = buildContextPaths(cwd, '.claude/context', name, stage);
    for (const filePath of projectPaths) {
        if (await (0, utils_1.fileExists)(filePath)) {
            await logger_1.logger.debug('Found project context file', { path: filePath, name, stage });
            return filePath;
        }
    }
    // Plugin-level context (fallback)
    const pluginRoot = getPluginRoot();
    if (pluginRoot) {
        const pluginPaths = buildContextPaths(pluginRoot, 'context', name, stage);
        for (const filePath of pluginPaths) {
            if (await (0, utils_1.fileExists)(filePath)) {
                await logger_1.logger.debug('Found plugin context file', { path: filePath, name, stage });
                return filePath;
            }
        }
    }
    return null;
}
/**
 * Discover agent-command scoped context file.
 * Pattern: {agent}-{command}-{stage}.md
 *
 * Priority:
 * 1. Project: {agent}-{command}-{stage}.md (most specific)
 * 2. Project: {agent}-{stage}.md (agent-specific)
 * 3. Plugin: {agent}-{command}-{stage}.md
 * 4. Plugin: {agent}-{stage}.md
 * 5. Standard discovery (backward compat, checks both project and plugin)
 */
async function discoverAgentCommandContext(cwd, agent, commandOrSkill, stage) {
    // Strip namespace prefix from agent name (cipherpowers:rust-agent → rust-agent)
    const agentName = agent.replace(/^[^:]+:/, '');
    const contextName = commandOrSkill?.replace(/^\//, '').replace(/^[^:]+:/, '');
    // Project-level paths (highest priority)
    const projectPaths = [];
    if (contextName) {
        projectPaths.push(path.join(cwd, '.claude', 'context', `${agentName}-${contextName}-${stage}.md`));
    }
    projectPaths.push(path.join(cwd, '.claude', 'context', `${agentName}-${stage}.md`));
    for (const filePath of projectPaths) {
        if (await (0, utils_1.fileExists)(filePath)) {
            await logger_1.logger.debug('Found project agent context file', { path: filePath, agent: agentName, stage });
            return filePath;
        }
    }
    // Plugin-level paths (fallback)
    const pluginRoot = getPluginRoot();
    if (pluginRoot) {
        const pluginPaths = [];
        if (contextName) {
            pluginPaths.push(path.join(pluginRoot, 'context', `${agentName}-${contextName}-${stage}.md`));
        }
        pluginPaths.push(path.join(pluginRoot, 'context', `${agentName}-${stage}.md`));
        for (const filePath of pluginPaths) {
            if (await (0, utils_1.fileExists)(filePath)) {
                await logger_1.logger.debug('Found plugin agent context file', { path: filePath, agent: agentName, stage });
                return filePath;
            }
        }
    }
    // Backward compat: try standard discovery with command/skill name
    // (discoverContextFile already checks both project and plugin)
    if (contextName) {
        const standardPath = await discoverContextFile(cwd, contextName, stage);
        if (standardPath) {
            return standardPath;
        }
    }
    return null;
}
/**
 * Extract name and stage from hook event.
 * Returns { name, stage } for context file discovery.
 *
 * Mapping:
 * - SlashCommandStart → { name: command, stage: 'start' }
 * - SlashCommandEnd → { name: command, stage: 'end' }
 * - SkillStart → { name: skill, stage: 'start' }
 * - SkillEnd → { name: skill, stage: 'end' }
 * - PreToolUse → { name: tool_name, stage: 'pre' }
 * - PostToolUse → { name: tool_name, stage: 'post' }
 * - SubagentStop → { name: agent_name, stage: 'end' } (special handling)
 * - UserPromptSubmit → { name: 'prompt', stage: 'submit' }
 * - Stop → { name: 'agent', stage: 'stop' }
 * - SessionStart → { name: 'session', stage: 'start' }
 * - SessionEnd → { name: 'session', stage: 'end' }
 * - Notification → { name: 'notification', stage: 'receive' }
 */
function extractNameAndStage(hookEvent, input) {
    switch (hookEvent) {
        case 'SlashCommandStart':
            return input.command
                ? { name: input.command.replace(/^\//, '').replace(/^[^:]+:/, ''), stage: 'start' }
                : null;
        case 'SlashCommandEnd':
            return input.command
                ? { name: input.command.replace(/^\//, '').replace(/^[^:]+:/, ''), stage: 'end' }
                : null;
        case 'SkillStart':
            return input.skill ? { name: input.skill.replace(/^[^:]+:/, ''), stage: 'start' } : null;
        case 'SkillEnd':
            return input.skill ? { name: input.skill.replace(/^[^:]+:/, ''), stage: 'end' } : null;
        case 'PreToolUse':
            return input.tool_name ? { name: input.tool_name.toLowerCase(), stage: 'pre' } : null;
        case 'PostToolUse':
            return input.tool_name ? { name: input.tool_name.toLowerCase(), stage: 'post' } : null;
        case 'SubagentStop':
            // SubagentStop has special handling - uses agent-command scoping
            return null;
        case 'UserPromptSubmit':
            return { name: 'prompt', stage: 'submit' };
        case 'Stop':
            return { name: 'agent', stage: 'stop' };
        case 'SessionStart':
            return { name: 'session', stage: 'start' };
        case 'SessionEnd':
            return { name: 'session', stage: 'end' };
        case 'Notification':
            return { name: 'notification', stage: 'receive' };
        default:
            return null;
    }
}
/**
 * Inject context from .claude/context/ files based on hook event.
 * This is the PRIMARY built-in gate - automatic context injection.
 *
 * Convention:
 * - .claude/context/{name}-{stage}.md
 * - e.g., .claude/context/code-review-start.md
 * - e.g., .claude/context/prompt-submit.md
 */
async function injectContext(hookEvent, input) {
    await logger_1.logger.debug('Context injection starting', { event: hookEvent, cwd: input.cwd });
    // Handle SubagentStop with agent-command scoping (special case)
    if (hookEvent === 'SubagentStop' && input.agent_name) {
        const session = new session_1.Session(input.cwd);
        const activeCommand = await session.get('active_command');
        const activeSkill = await session.get('active_skill');
        const commandOrSkill = activeCommand || activeSkill;
        const contextFile = await discoverAgentCommandContext(input.cwd, input.agent_name, commandOrSkill, 'end');
        if (contextFile) {
            const content = await fs.readFile(contextFile, 'utf-8');
            await logger_1.logger.info('Injecting agent context', {
                event: hookEvent,
                agent: input.agent_name,
                file: contextFile
            });
            return content;
        }
        return null;
    }
    // Standard context discovery for all other hooks
    const extracted = extractNameAndStage(hookEvent, input);
    if (!extracted) {
        await logger_1.logger.debug('No name/stage extracted', { event: hookEvent });
        return null;
    }
    const { name, stage } = extracted;
    const contextFile = await discoverContextFile(input.cwd, name, stage);
    if (contextFile) {
        const content = await fs.readFile(contextFile, 'utf-8');
        await logger_1.logger.info('Injecting context', {
            event: hookEvent,
            name,
            stage,
            file: contextFile
        });
        return content;
    }
    await logger_1.logger.debug('No context file found', { event: hookEvent, name, stage });
    return null;
}
