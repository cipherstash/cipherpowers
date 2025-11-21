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
exports.executeShellCommand = executeShellCommand;
exports.executeBuiltinGate = executeBuiltinGate;
exports.executeGate = executeGate;
// plugin/hooks/hooks-app/src/gate-loader.ts
const child_process_1 = require("child_process");
const util_1 = require("util");
const path = __importStar(require("path"));
const execAsync = (0, util_1.promisify)(child_process_1.exec);
/**
 * Execute shell command from gate configuration with timeout.
 *
 * SECURITY MODEL: gates.json is trusted configuration (project-controlled, not user input).
 * Commands are executed without sanitization because:
 * 1. gates.json is committed to repository or managed by project admins
 * 2. Users cannot inject commands without write access to gates.json
 * 3. If gates.json is compromised, the project is already compromised
 *
 * This is equivalent to package.json scripts or Makefile targets - trusted project configuration.
 *
 * ERROR HANDLING: Commands timeout after 30 seconds to prevent hung gates.
 */
async function executeShellCommand(command, cwd, timeoutMs = 30000) {
    try {
        const { stdout, stderr } = await execAsync(command, { cwd, timeout: timeoutMs });
        return {
            exitCode: 0,
            output: stdout + stderr
        };
    }
    catch (error) {
        const err = error;
        if (err.killed && err.signal === 'SIGTERM') {
            return {
                exitCode: 124, // Standard timeout exit code
                output: `Command timed out after ${timeoutMs}ms`
            };
        }
        return {
            exitCode: err.code || 1,
            output: (err.stdout || '') + (err.stderr || '')
        };
    }
}
async function executeBuiltinGate(gateName, input) {
    const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || '';
    const gatePath = path.join(pluginRoot, 'hooks', 'gates', gateName);
    try {
        const module = await Promise.resolve(`${gatePath}`).then(s => __importStar(require(s)));
        return await module.execute(input);
    }
    catch (error) {
        throw new Error(`Failed to load built-in gate ${gateName}: ${error}`);
    }
}
async function executeGate(gateName, gateConfig, input) {
    if (gateConfig.command) {
        // Shell command gate
        const shellResult = await executeShellCommand(gateConfig.command, input.cwd);
        const passed = shellResult.exitCode === 0;
        return {
            passed,
            result: {
                additionalContext: shellResult.output
            }
        };
    }
    else {
        // Built-in TypeScript gate
        const result = await executeBuiltinGate(gateName, input);
        const passed = !result.decision && result.continue !== false;
        return {
            passed,
            result
        };
    }
}
