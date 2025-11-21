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
// plugin/hooks/hooks-app/__tests__/context.test.ts
const context_1 = require("../src/context");
const fs = __importStar(require("fs/promises"));
const path = __importStar(require("path"));
const os = __importStar(require("os"));
describe('Context Injection', () => {
    let testDir;
    beforeEach(async () => {
        testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'hooks-test-'));
    });
    afterEach(async () => {
        await fs.rm(testDir, { recursive: true, force: true });
    });
    test('returns null when no context file exists', async () => {
        const result = await (0, context_1.discoverContextFile)(testDir, 'test-command', 'start');
        expect(result).toBeNull();
    });
    test('discovers flat context file', async () => {
        const contextDir = path.join(testDir, '.claude', 'context');
        await fs.mkdir(contextDir, { recursive: true });
        await fs.writeFile(path.join(contextDir, 'test-command-start.md'), 'content');
        const result = await (0, context_1.discoverContextFile)(testDir, 'test-command', 'start');
        expect(result).toBe(path.join(contextDir, 'test-command-start.md'));
    });
    test('discovers slash-command subdirectory', async () => {
        const contextDir = path.join(testDir, '.claude', 'context', 'slash-command');
        await fs.mkdir(contextDir, { recursive: true });
        await fs.writeFile(path.join(contextDir, 'test-command-start.md'), 'content');
        const result = await (0, context_1.discoverContextFile)(testDir, 'test-command', 'start');
        expect(result).toBe(path.join(contextDir, 'test-command-start.md'));
    });
    test('discovers nested slash-command directory', async () => {
        const contextDir = path.join(testDir, '.claude', 'context', 'slash-command', 'test-command');
        await fs.mkdir(contextDir, { recursive: true });
        await fs.writeFile(path.join(contextDir, 'start.md'), 'content');
        const result = await (0, context_1.discoverContextFile)(testDir, 'test-command', 'start');
        expect(result).toBe(path.join(contextDir, 'start.md'));
    });
    test('discovers skill context', async () => {
        const contextDir = path.join(testDir, '.claude', 'context', 'skill');
        await fs.mkdir(contextDir, { recursive: true });
        await fs.writeFile(path.join(contextDir, 'test-skill-start.md'), 'content');
        const result = await (0, context_1.discoverContextFile)(testDir, 'test-skill', 'start');
        expect(result).toBe(path.join(contextDir, 'test-skill-start.md'));
    });
    test('follows priority order - flat wins', async () => {
        const contextBase = path.join(testDir, '.claude', 'context');
        await fs.mkdir(path.join(contextBase, 'slash-command'), { recursive: true });
        await fs.writeFile(path.join(contextBase, 'test-command-start.md'), 'flat');
        await fs.writeFile(path.join(contextBase, 'slash-command', 'test-command-start.md'), 'subdir');
        const result = await (0, context_1.discoverContextFile)(testDir, 'test-command', 'start');
        expect(result).toBe(path.join(contextBase, 'test-command-start.md'));
    });
});
