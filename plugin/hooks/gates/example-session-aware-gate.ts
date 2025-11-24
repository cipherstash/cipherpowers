// plugin/hooks/gates/example-session-aware-gate.ts
import { Session } from '../hooks-app/dist/session';
import type { HookInput, GateResult } from '../hooks-app/dist/types';

/**
 * Example gate that uses session state
 *
 * This gate provides context when Rust files are edited during /execute,
 * reminding the user to consider using the rust-engineer agent.
 *
 * Demonstrates session state usage:
 * - Reading active_command
 * - Checking array contains
 * - Using metadata for custom tracking
 */
export async function execute(input: HookInput): Promise<GateResult> {
  const session = new Session(input.cwd);

  // Only enforce during /execute
  const activeCommand = await session.get('active_command');
  if (activeCommand !== '/execute') {
    return { additionalContext: '' };
  }

  // Check if Rust files edited
  const hasRustFiles = await session.contains('file_extensions', 'rs');
  if (!hasRustFiles) {
    return { additionalContext: '' };
  }

  // Example: Track reminder count in metadata
  const metadata = await session.get('metadata');
  const reminderCount = (metadata?.rust_reminder_count ?? 0) + 1;
  await session.set('metadata', {
    ...(metadata || {}),
    rust_reminder_count: reminderCount,
    last_rust_reminder: new Date().toISOString()
  });

  // Provide helpful context about Rust files
  return {
    additionalContext: `
## Rust Files Detected

You've edited Rust files during plan execution. Consider:
- Dispatching rust-engineer agent for TDD workflow
- Using rust-engineer for code review before completion
- Ensuring Prime Directive (type system) compliance

(Reminder #${reminderCount})
    `.trim()
  };
}
