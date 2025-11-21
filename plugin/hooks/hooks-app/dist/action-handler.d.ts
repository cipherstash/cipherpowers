import { GateResult, GatesConfig, HookInput } from './types';
export interface ActionResult {
    continue: boolean;
    context?: string;
    blockReason?: string;
    stopMessage?: string;
    chainedGate?: string;
}
export declare function handleAction(action: string, gateResult: GateResult, _config: GatesConfig, _input: HookInput): Promise<ActionResult>;
