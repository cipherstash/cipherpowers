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

**Commands:** Slash commands users type (`/plan`, `/work`, `/code-review`)
- Thin dispatchers providing context
- Reference practices for project-specific configuration
- Reference skills for process guidance
- Do NOT contain workflow logic (that's in agents)

**Agents:** Specialized subagent prompts with enforced workflows
- Handle specific roles (work-planner, code-reviewer, rust-engineer)
- Contain non-negotiable workflows using persuasion principles
- Reference practices for project-specific commands and conventions
- Reference skills for methodology
- Receive context from commands

**Agent-Centric Architecture:**
CipherPowers uses an agent-centric model where agents contain the complete workflow:
- **Authority Principle**: Imperative language, non-negotiable steps
- **Commitment Principle**: Announcements and checklists create accountability
- **Scarcity Principle**: Immediate requirements and completion criteria
- **Social Proof Principle**: Failure modes and rationalization defenses

**Templates:**
- `agents/_template.md` - Agent structure with persuasion principles
- `docs/practices/_template.md` - Practice structure with standards + config pattern

### 3. Documentation Layer (`docs/`)

Standards, guidelines, and reference materials.

**Practices:** Coding standards, conventions, guidelines
**Examples:** Real-world examples and templates
**Purpose:** Support skills and provide team reference

## Organizational Benefits

This three-layer separation achieves key software engineering principles:

✅ **DRY (Don't Repeat Yourself)**
- Standards live in one place (`docs/practices/`)
- Skills reference practices instead of duplicating them
- Commands reference skills instead of reimplementing workflows
- Changes propagate automatically through references

✅ **SRP (Single Responsibility Principle)**
- **Practices** define standards (WHAT to follow)
- **Skills** define workflows (HOW to do it)
- **Commands** dispatch to skills (WHEN to invoke)
- Each component has exactly one reason to change

✅ **Reusability**
- Skills are universal workflows (portable, can be upstreamed to superpowers)
- Practices are project-specific standards (customized for your team)
- Commands add project context to universal workflows
- Mix local and upstream skills seamlessly

✅ **Testability**
- Skills include TDD test scenarios using subagents
- Baseline tests prove problems exist without the skill
- With-skill tests verify effectiveness under pressure
- Test scenarios document expected violations and how skill prevents them
- See `skills/*/test-scenarios.md` for examples

✅ **Maintainability**
- Update standards once in practices, all skills benefit
- Change skill workflow without touching commands
- Add new commands without modifying skills
- Clear boundaries prevent coupling and drift

**Example: Code Review Workflow**
- `docs/practices/code-review.md` = Standards (severity levels) + Project Config (commands, file conventions)
- `agents/code-reviewer.md` = Workflow with persuasion principles (non-negotiable steps, rationalization defenses)
- `commands/code-review.md` = Thin dispatcher (sets context, invokes agent)
- Skills: References upstream "Requesting Code Review" and "Code Review Reception" skills

All components work together without duplication:
- Update severity standards in practices → agent automatically uses new standards
- Change project commands (mise run test) → agent references practice for current command
- Commands remain simple dispatchers → workflow enforcement stays in agent

**Example: Documentation Structure**
- `docs/practices/documentation.md` = Standards (formatting, completeness, structure)
- `skills/documentation/maintaining-docs-after-changes/` = Workflow (two-phase sync process)
- `skills/documentation/capturing-learning/` = Workflow (retrospective capture process)
- `commands/doc-review.md` = Dispatcher (triggers maintenance workflow with project context)
- `commands/summarise.md` = Dispatcher (triggers learning capture with work tracking integration)

All five components work together without duplication. Change documentation standards once, both skills and commands use the updated version automatically.

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

## Creating Agents and Practices

**When creating agents:**
1. Use `agents/_template.md` as starting point
2. Include all four persuasion principles (Authority, Commitment, Scarcity, Social Proof)
3. Reference practices for project-specific configuration (don't hardcode commands)
4. Reference skills for methodology
5. Make workflows non-negotiable with explicit rationalization defenses

**When creating practices:**
1. Use `docs/practices/_template.md` as starting point
2. Separate universal standards from project-specific configuration
3. Standards section: What quality looks like (universal principles)
4. Project Configuration section: Commands, file conventions, tool settings
5. Agents reference practices, not the other way around

## Team Usage

1. Install cipherpowers as a Claude Code plugin
2. Install superpowers for universal skills
3. Both collections work together seamlessly
4. Commands dispatch to agents or main Claude with practice context

