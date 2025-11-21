export interface HookInput {
    hook_event_name: string;
    cwd: string;
    tool_name?: string;
    file_path?: string;
    agent_name?: string;
    subagent_name?: string;
    output?: string;
    user_message?: string;
    command?: string;
    skill?: string;
}
export interface GateResult {
    additionalContext?: string;
    decision?: 'block';
    reason?: string;
    continue?: false;
    message?: string;
}
export type GateExecute = (input: HookInput) => Promise<GateResult>;
export interface GateConfig {
    command?: string;
    on_pass?: string;
    on_fail?: string;
}
export interface HookConfig {
    enabled_tools?: string[];
    enabled_agents?: string[];
    gates?: string[];
}
export interface GatesConfig {
    hooks: Record<string, HookConfig>;
    gates: Record<string, GateConfig>;
}
