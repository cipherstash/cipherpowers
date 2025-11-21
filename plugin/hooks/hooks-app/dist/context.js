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
async function injectContext(hookEvent, input) {
    let name;
    let stage;
    if (hookEvent === 'SlashCommandStart' || hookEvent === 'SlashCommandEnd') {
        name = input.command?.replace(/^\//, '');
        stage = hookEvent === 'SlashCommandStart' ? 'start' : 'end';
    }
    else if (hookEvent === 'SkillStart' || hookEvent === 'SkillEnd') {
        name = input.skill;
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
