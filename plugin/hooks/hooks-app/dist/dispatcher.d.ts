import { HookInput, HookConfig, GateConfig } from './types';
export declare function shouldProcessHook(input: HookInput, hookConfig: HookConfig): boolean;
export interface DispatchResult {
    context?: string;
    blockReason?: string;
    stopMessage?: string;
}
/**
 * Check if gate should run based on keyword matching (UserPromptSubmit only).
 * Gates without keywords always run (backwards compatible).
 *
 * Note: Uses substring matching, not word-boundary matching. This means "test"
 * will match "latest" or "contest". This is intentional for flexibility - users
 * can say "let's test this" or "testing the feature" and both will match.
 * If word-boundary matching is needed in the future, consider using regex like:
 * /\b${keyword}\b/i.test(message)
 */
export declare function gateMatchesKeywords(gateConfig: GateConfig, userMessage: string | undefined): boolean;
export declare function dispatch(input: HookInput): Promise<DispatchResult>;
