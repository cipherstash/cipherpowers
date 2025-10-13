# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CipherPowers

CipherPowers is a Claude Code plugin providing a comprehensive toolkit for development teams.

## Architecture

CipherPowers combines three layers:

### 1. Skills Layer (`skills/`)

Reusable process knowledge documented using the superpowers framework. Skills are testable, discoverable guides for techniques and workflows.

**Key principles:**
- Written following TDD: test with subagents before writing
- Include rich `when_to_use` frontmatter for discovery
- Follow superpowers SKILL.md structure
- Can reference upstream superpowers skills

**Scope:**
- Organization-specific workflows and practices
- Universal skills under development (before upstreaming)
- Extensions to superpowers skills for team context

### 2. Automation Layer (`commands/`, `agents/`)

Commands and agents that dispatch to skills or provide project-specific workflows.

**Commands:** Slash commands users type (`/plan`, `/work`)
- Provide context and instructions
- Dispatch to agents for complex work
- Reference skills for process guidance

**Agents:** Specialized subagent prompts
- Handle specific roles (work-planner, code-reviewer)
- Follow skills for methodology
- Receive context from commands

### 3. Documentation Layer (`docs/`)

Standards, guidelines, and reference materials.

**Practices:** Coding standards, conventions, guidelines
**Examples:** Real-world examples and templates
**Purpose:** Support skills and provide team reference

## Integration with Superpowers

**Custom find-skills tool** (`tools/find-skills`):
- Searches `${CIPHERPOWERS_ROOT}/skills/` (org-specific)
- Searches `${SUPERPOWERS_SKILLS_ROOT}` (universal skills)
- Provides unified discovery across both collections
- Flags: `--local`, `--upstream`, or default (both)

**Skill references:**
Commands and agents reference skills from either collection transparently using standard paths.

## Using the find-skills Tool

The custom `tools/find-skills` script provides unified discovery:

```bash
# From repository root
./tools/find-skills "search pattern"

# With scope flags
./tools/find-skills --local "pattern"      # cipherpowers only
./tools/find-skills --upstream "pattern"   # superpowers only
```

## Working with Skills in this Repository

When creating or editing skills in `skills/`:

1. **Read the meta-skill:** `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md`
2. **Follow TDD:** Test with subagents BEFORE writing
3. **Use TodoWrite:** Create todos for the skill creation checklist
4. **Consider upstream:** Universal skills may be contributed to superpowers later

## Team Usage

1. Install cipherpowers as a Claude Code plugin
2. Install superpowers for universal skills
3. Both collections work together seamlessly
4. Commands dispatch to skills from either location

