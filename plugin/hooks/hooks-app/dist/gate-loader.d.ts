import { HookInput, GateResult, GateConfig } from './types';
export interface ShellResult {
    exitCode: number;
    output: string;
}
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
export declare function executeShellCommand(command: string, cwd: string, timeoutMs?: number): Promise<ShellResult>;
/**
 * Load and execute a built-in TypeScript gate
 *
 * Built-in gates are TypeScript modules in src/gates/ that export an execute function.
 * Gate names use kebab-case and are mapped to camelCase module names:
 * - "plan-compliance" → planCompliance
 * - "plugin-path" → pluginPath
 * - "commands" → commands
 */
export declare function executeBuiltinGate(gateName: string, input: HookInput): Promise<GateResult>;
export declare function executeGate(gateName: string, gateConfig: GateConfig, input: HookInput): Promise<{
    passed: boolean;
    result: GateResult;
}>;
