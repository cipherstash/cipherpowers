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
export interface SessionState {
    /** Unique session identifier (timestamp-based) */
    session_id: string;
    /** ISO 8601 timestamp when session started */
    started_at: string;
    /** Currently active slash command (e.g., "/execute") */
    active_command: string | null;
    /** Currently active skill (e.g., "executing-plans") */
    active_skill: string | null;
    /** Files edited during this session */
    edited_files: string[];
    /** File extensions edited during this session (deduplicated) */
    file_extensions: string[];
    /** Custom metadata for specific workflows */
    metadata: Record<string, any>;
}
/** Array field keys in SessionState (for type-safe operations) */
export type SessionStateArrayKey = 'edited_files' | 'file_extensions';
/** Scalar field keys in SessionState */
export type SessionStateScalarKey = Exclude<keyof SessionState, SessionStateArrayKey | 'metadata'>;
