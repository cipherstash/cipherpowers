// plugin/hooks/hooks-app/src/types.ts

export interface HookInput {
  hook_event_name: string;
  cwd: string;

  // PostToolUse
  tool_name?: string;
  file_path?: string;

  // SubagentStop
  agent_name?: string;
  subagent_name?: string;
  output?: string;

  // UserPromptSubmit
  user_message?: string;

  // SlashCommand/Skill
  command?: string;
  skill?: string;
}

export interface GateResult {
  // Success - add context and continue
  additionalContext?: string;

  // Block agent from proceeding
  decision?: 'block';
  reason?: string;

  // Stop Claude entirely
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

// Session state interface
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

// Note: active_agent NOT included - Claude Code does not provide unique
// agent identifiers. Use metadata field if you need custom agent tracking.

/** Array field keys in SessionState (for type-safe operations) */
export type SessionStateArrayKey = 'edited_files' | 'file_extensions';

/** Scalar field keys in SessionState */
export type SessionStateScalarKey = Exclude<keyof SessionState, SessionStateArrayKey | 'metadata'>;
