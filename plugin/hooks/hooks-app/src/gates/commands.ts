// plugin/hooks/hooks-app/src/gates/commands.ts
import { HookInput, GateResult } from '../types';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as yaml from 'js-yaml';

/**
 * Commands Gate - Context-aware command injection
 *
 * Injects project commands from CLAUDE.md frontmatter based on the user's
 * message content and slash command. Only injects commands that are relevant
 * to the current interaction, saving tokens.
 *
 * Typical usage: UserPromptSubmit hook to inject commands when user mentions them.
 */

interface ClaudeMdFrontmatter {
  commands?: Record<string, string>;
}

/**
 * Parse CLAUDE.md frontmatter to extract commands
 */
async function parseClaudeMd(cwd: string): Promise<Record<string, string>> {
  const claudeMdPath = path.join(cwd, 'CLAUDE.md');

  try {
    const content = await fs.readFile(claudeMdPath, 'utf-8');

    // Extract YAML frontmatter between --- markers
    const frontmatterMatch = content.match(/^---\s*\n([\s\S]*?)\n---/);
    if (!frontmatterMatch) {
      return {};
    }

    const frontmatter = yaml.load(frontmatterMatch[1]) as ClaudeMdFrontmatter;
    return frontmatter.commands || {};
  } catch (error) {
    // File doesn't exist or parse error - return empty
    return {};
  }
}

/**
 * Detect which commands are needed based on canonical phrases
 */
function detectNeededCommands(userMessage: string): string[] {
  if (!userMessage) {
    return [];
  }

  const needed: string[] = [];
  const lowerMessage = userMessage.toLowerCase();

  // Detect canonical command phrases: "Run project <type> command"
  if (lowerMessage.includes('run project test command')) {
    needed.push('test');
  }

  if (lowerMessage.includes('run project check command')) {
    needed.push('check');
  }

  if (lowerMessage.includes('run project build command')) {
    needed.push('build');
  }

  if (lowerMessage.includes('run project run command')) {
    needed.push('run');
  }

  // Remove duplicates
  return [...new Set(needed)];
}

/**
 * Inject commands as XML additional context
 */
function injectCommands(needed: string[], commands: Record<string, string>): string {
  if (needed.length === 0) {
    return '';
  }

  const lines = ['<project_commands>'];

  for (const cmdType of needed) {
    const cmdValue = commands[cmdType];
    if (cmdValue) {
      lines.push(`  <${cmdType}>${cmdValue}</${cmdType}>`);
    }
  }

  lines.push('</project_commands>');

  return lines.join('\n');
}

export async function execute(input: HookInput): Promise<GateResult> {
  // Parse available commands
  const commands = await parseClaudeMd(input.cwd);

  // Detect needed commands from user message
  const userMessage = input.user_message || '';
  const neededCommands = detectNeededCommands(userMessage);

  // Only inject if commands are needed
  if (neededCommands.length > 0) {
    const contextMessage = injectCommands(neededCommands, commands);

    return {
      additionalContext: contextMessage
    };
  }

  // No commands needed - clean exit
  return {};
}
