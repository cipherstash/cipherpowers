// plugin/hooks/hooks-app/src/gate-loader.ts
import { exec } from 'child_process';
import { promisify } from 'util';
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
  } catch (error: any) {
    if (error.killed && error.signal === 'SIGTERM') {
      return {
        exitCode: 124, // Standard timeout exit code
        output: `Command timed out after ${timeoutMs}ms`
      };
    }
    return {
      exitCode: error.code || 1,
      output: (error.stdout || '') + (error.stderr || '')
    };
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
    // Built-in TypeScript gate (to be implemented later)
    throw new Error(`Built-in gate ${gateName} not yet implemented`);
  }
}
