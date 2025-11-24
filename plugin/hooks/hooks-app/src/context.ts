// plugin/hooks/hooks-app/src/context.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { HookInput } from './types';
import { fileExists } from './utils';
import { Session } from './session';

/**
 * Discover context file following priority order.
 * Priority: flat > slash-command subdir > slash-command nested > skill subdir > skill nested
 */
export async function discoverContextFile(
  cwd: string,
  name: string,
  stage: string
): Promise<string | null> {
  const paths = [
    path.join(cwd, '.claude', 'context', `${name}-${stage}.md`),
    path.join(cwd, '.claude', 'context', 'slash-command', `${name}-${stage}.md`),
    path.join(cwd, '.claude', 'context', 'slash-command', name, `${stage}.md`),
    path.join(cwd, '.claude', 'context', 'skill', `${name}-${stage}.md`),
    path.join(cwd, '.claude', 'context', 'skill', name, `${stage}.md`)
  ];

  for (const filePath of paths) {
    if (await fileExists(filePath)) {
      return filePath;
    }
  }

  return null;
}

/**
 * Discover agent-command scoped context file.
 * Pattern: {agent}-{command}-{stage}.md
 *
 * Priority:
 * 1. {agent}-{command}-{stage}.md (most specific)
 * 2. {agent}-{stage}.md (agent-specific)
 * 3. Standard discovery (backward compat)
 */
async function discoverAgentCommandContext(
  cwd: string,
  agent: string,
  commandOrSkill: string | null,
  stage: string
): Promise<string | null> {
  // Strip namespace prefix from agent name (cipherpowers:rust-agent → rust-agent)
  const agentName = agent.replace(/^[^:]+:/, '');

  const paths: string[] = [];

  // Most specific: {agent}-{command}-{stage}.md
  if (commandOrSkill) {
    const contextName = commandOrSkill.replace(/^\//, '').replace(/^[^:]+:/, '');
    paths.push(path.join(cwd, '.claude', 'context', `${agentName}-${contextName}-${stage}.md`));
  }

  // Agent-specific: {agent}-{stage}.md
  paths.push(path.join(cwd, '.claude', 'context', `${agentName}-${stage}.md`));

  // Backward compat: try standard discovery with command/skill name
  if (commandOrSkill) {
    const contextName = commandOrSkill.replace(/^\//, '').replace(/^[^:]+:/, '');
    const standardPath = await discoverContextFile(cwd, contextName, stage);
    if (standardPath) {
      paths.push(standardPath);
    }
  }

  for (const filePath of paths) {
    if (await fileExists(filePath)) {
      return filePath;
    }
  }

  return null;
}

export async function injectContext(hookEvent: string, input: HookInput): Promise<string | null> {
  let name: string | undefined;
  let stage: string | undefined;

  // Handle SubagentStop with agent-command scoping
  if (hookEvent === 'SubagentStop' && input.agent_name) {
    stage = 'end';

    // Get active command/skill from session state
    const session = new Session(input.cwd);
    const activeCommand = await session.get('active_command');
    const activeSkill = await session.get('active_skill');
    const commandOrSkill = activeCommand || activeSkill;

    const contextFile = await discoverAgentCommandContext(
      input.cwd,
      input.agent_name,
      commandOrSkill,
      stage
    );

    if (contextFile) {
      const content = await fs.readFile(contextFile, 'utf-8');
      return content;
    }

    return null;
  }

  // Handle SlashCommand events
  if (hookEvent === 'SlashCommandStart' || hookEvent === 'SlashCommandEnd') {
    name = input.command?.replace(/^\//, '').replace(/^[^:]+:/, '');
    stage = hookEvent === 'SlashCommandStart' ? 'start' : 'end';
  }
  // Handle Skill events
  else if (hookEvent === 'SkillStart' || hookEvent === 'SkillEnd') {
    name = input.skill?.replace(/^[^:]+:/, '');
    stage = hookEvent === 'SkillStart' ? 'start' : 'end';
  }

  if (!name || !stage) return null;

  const contextFile = await discoverContextFile(input.cwd, name, stage);

  if (contextFile) {
    const content = await fs.readFile(contextFile, 'utf-8');
    return content;
  }

  return null;
}
