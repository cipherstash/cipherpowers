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
