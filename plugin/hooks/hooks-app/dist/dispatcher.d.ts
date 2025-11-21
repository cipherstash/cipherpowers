import { HookInput, HookConfig } from './types';
export declare function shouldProcessHook(input: HookInput, hookConfig: HookConfig): boolean;
export interface DispatchResult {
    context?: string;
    blockReason?: string;
    stopMessage?: string;
}
export declare function dispatch(input: HookInput): Promise<DispatchResult>;
