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
/**
 * Discover context file following priority order.
 * Priority: flat > slash-command subdir > slash-command nested > skill subdir > skill nested
 */
async function discoverContextFile(cwd, name, stage) {
    const paths = [
        path.join(cwd, '.claude', 'context', `${name}-${stage}.md`),
        path.join(cwd, '.claude', 'context', 'slash-command', `${name}-${stage}.md`),
        path.join(cwd, '.claude', 'context', 'slash-command', name, `${stage}.md`),
        path.join(cwd, '.claude', 'context', 'skill', `${name}-${stage}.md`),
        path.join(cwd, '.claude', 'context', 'skill', name, `${stage}.md`)
    ];
    for (const filePath of paths) {
        if (await (0, utils_1.fileExists)(filePath)) {
            return filePath;
        }
    }
    return null;
}
/**
 * Discover agent-command scoped context file.
 * Pattern: {agent}-{command}-{stage}.md
 *
 * Priority:
 * 1. {agent}-{command}-{stage}.md (most specific)
 * 2. {agent}-{stage}.md (agent-specific)
 * 3. Standard discovery (backward compat)
 */
async function discoverAgentCommandContext(cwd, agent, commandOrSkill, stage) {
    // Strip namespace prefix from agent name (cipherpowers:rust-agent → rust-agent)
    const agentName = agent.replace(/^[^:]+:/, '');
    const paths = [];
    // Most specific: {agent}-{command}-{stage}.md
    if (commandOrSkill) {
        const contextName = commandOrSkill.replace(/^\//, '').replace(/^[^:]+:/, '');
        paths.push(path.join(cwd, '.claude', 'context', `${agentName}-${contextName}-${stage}.md`));
    }
    // Agent-specific: {agent}-{stage}.md
    paths.push(path.join(cwd, '.claude', 'context', `${agentName}-${stage}.md`));
    // Backward compat: try standard discovery with command/skill name
    if (commandOrSkill) {
        const contextName = commandOrSkill.replace(/^\//, '').replace(/^[^:]+:/, '');
        const standardPath = await discoverContextFile(cwd, contextName, stage);
        if (standardPath) {
            paths.push(standardPath);
        }
    }
    for (const filePath of paths) {
        if (await (0, utils_1.fileExists)(filePath)) {
            return filePath;
        }
    }
    return null;
}
async function injectContext(hookEvent, input) {
    let name;
    let stage;
    // Handle SubagentStop with agent-command scoping
    if (hookEvent === 'SubagentStop' && input.agent_name) {
        stage = 'end';
        // Get active command/skill from session state
        const session = new session_1.Session(input.cwd);
        const activeCommand = await session.get('active_command');
        const activeSkill = await session.get('active_skill');
        const commandOrSkill = activeCommand || activeSkill;
        const contextFile = await discoverAgentCommandContext(input.cwd, input.agent_name, commandOrSkill, stage);
        if (contextFile) {
            const content = await fs.readFile(contextFile, 'utf-8');
            return content;
        }
        return null;
    }
    // Handle SlashCommand events
    if (hookEvent === 'SlashCommandStart' || hookEvent === 'SlashCommandEnd') {
        name = input.command?.replace(/^\//, '').replace(/^[^:]+:/, '');
        stage = hookEvent === 'SlashCommandStart' ? 'start' : 'end';
    }
    // Handle Skill events
    else if (hookEvent === 'SkillStart' || hookEvent === 'SkillEnd') {
        name = input.skill?.replace(/^[^:]+:/, '');
        stage = hookEvent === 'SkillStart' ? 'start' : 'end';
    }
    if (!name || !stage)
        return null;
    const contextFile = await discoverContextFile(input.cwd, name, stage);
    if (contextFile) {
        const content = await fs.readFile(contextFile, 'utf-8');
        return content;
    }
    return null;
}
