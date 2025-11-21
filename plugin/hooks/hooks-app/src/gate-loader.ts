// plugin/hooks/hooks-app/src/gate-loader.ts
import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';
import { HookInput, GateResult, GateConfig } from './types';

const execAsync = promisify(exec);

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
export async function executeShellCommand(
  command: string,
  cwd: string,
  timeoutMs: number = 30000
): Promise<ShellResult> {
  try {
    const { stdout, stderr } = await execAsync(command, { cwd, timeout: timeoutMs });
    return {
      exitCode: 0,
      output: stdout + stderr
    };
  } catch (error: unknown) {
    const err = error as { killed?: boolean; signal?: string; code?: number; stdout?: string; stderr?: string };
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

export async function executeBuiltinGate(
  gateName: string,
  input: HookInput
): Promise<GateResult> {
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || '';
  const gatePath = path.join(pluginRoot, 'hooks', 'gates', gateName);

  try {
    const module = await import(gatePath);
    return await module.execute(input);
  } catch (error) {
    throw new Error(`Failed to load built-in gate ${gateName}: ${error}`);
  }
}

export async function executeGate(
  gateName: string,
  gateConfig: GateConfig,
  input: HookInput
): Promise<{ passed: boolean; result: GateResult }> {
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
  } else {
    // Built-in TypeScript gate
    const result = await executeBuiltinGate(gateName, input);
    const passed = !result.decision && result.continue !== false;

    return {
      passed,
      result
    };
  }
}
