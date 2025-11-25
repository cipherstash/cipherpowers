import { HookInput } from './types';
/**
 * Discover context file following priority order.
 *
 * Priority (project takes precedence over plugin):
 * 1. Project: .claude/context/{name}-{stage}.md (and variations)
 * 2. Plugin: ${CLAUDE_PLUGIN_ROOT}/context/{name}-{stage}.md (and variations)
 */
export declare function discoverContextFile(cwd: string, name: string, stage: string): Promise<string | null>;
/**
 * Inject context from .claude/context/ files based on hook event.
 * This is the PRIMARY built-in gate - automatic context injection.
 *
 * Convention:
 * - .claude/context/{name}-{stage}.md
 * - e.g., .claude/context/code-review-start.md
 * - e.g., .claude/context/prompt-submit.md
 */
export declare function injectContext(hookEvent: string, input: HookInput): Promise<string | null>;
