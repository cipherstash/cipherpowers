import { HookInput } from './types';
/**
 * Discover context file following priority order.
 * Priority: flat > slash-command subdir > slash-command nested > skill subdir > skill nested
 */
export declare function discoverContextFile(cwd: string, name: string, stage: string): Promise<string | null>;
export declare function injectContext(hookEvent: string, input: HookInput): Promise<string | null>;
