// plugin/hooks/hooks-app/src/context.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { HookInput } from './types';
import { fileExists } from './utils';

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

export async function injectContext(hookEvent: string, input: HookInput): Promise<string | null> {
  let name: string | undefined;
  let stage: string | undefined;

  if (hookEvent === 'SlashCommandStart' || hookEvent === 'SlashCommandEnd') {
    name = input.command?.replace(/^\//, '');
    stage = hookEvent === 'SlashCommandStart' ? 'start' : 'end';
  } else if (hookEvent === 'SkillStart' || hookEvent === 'SkillEnd') {
    name = input.skill;
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
