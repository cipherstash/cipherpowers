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
// plugin/hooks/hooks-app/__tests__/config.test.ts
const config_1 = require("../src/config");
const fs = __importStar(require("fs/promises"));
const path = __importStar(require("path"));
const os = __importStar(require("os"));
describe('Config Loading', () => {
    let testDir;
    beforeEach(async () => {
        testDir = await fs.mkdtemp(path.join(os.tmpdir(), 'hooks-test-'));
    });
    afterEach(async () => {
        await fs.rm(testDir, { recursive: true, force: true });
    });
    test('returns null when no config exists', async () => {
        const config = await (0, config_1.loadConfig)(testDir);
        expect(config).toBeNull();
    });
    test('loads .claude/gates.json with highest priority', async () => {
        const claudeDir = path.join(testDir, '.claude');
        await fs.mkdir(claudeDir);
        const config1 = { hooks: {}, gates: { test: { command: 'claude-config' } } };
        const config2 = { hooks: {}, gates: { test: { command: 'root-config' } } };
        await fs.writeFile(path.join(claudeDir, 'gates.json'), JSON.stringify(config1));
        await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(config2));
        const config = await (0, config_1.loadConfig)(testDir);
        expect(config?.gates.test.command).toBe('claude-config');
    });
    test('loads gates.json from root when .claude does not exist', async () => {
        const config1 = { hooks: {}, gates: { test: { command: 'root-config' } } };
        await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(config1));
        const config = await (0, config_1.loadConfig)(testDir);
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
        const config = await (0, config_1.loadConfig)(testDir);
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
        await expect((0, config_1.loadConfig)(testDir)).rejects.toThrow('Unknown hook event');
    });
    test('rejects undefined gate reference', async () => {
        const configObj = {
            hooks: {
                PostToolUse: { gates: ['nonexistent'] }
            },
            gates: {}
        };
        await fs.writeFile(path.join(testDir, 'gates.json'), JSON.stringify(configObj));
        await expect((0, config_1.loadConfig)(testDir)).rejects.toThrow('references undefined gate');
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
        await expect((0, config_1.loadConfig)(testDir)).rejects.toThrow('is not CONTINUE/BLOCK/STOP or valid gate name');
    });
});
