---
commands:
  test: "mise run test"
  check: "mise run check"
  build: "mise run build"
---

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Plugin Architecture Overview:** CipherPowers implements a three-layer plugin architecture that separates concerns for maintainability and reusability. Skills provide reusable workflows, automation provides commands and agents, and documentation provides practices and standards.

## CipherPowers

CipherPowers is a Claude Code plugin providing a comprehensive toolkit for development teams. Built on a three-layer plugin architecture, it separates skills (reusable workflows), automation (commands and agents), and documentation (practices and standards) to ensure team-wide consistency and maintainability.

## Development Commands

CipherPowers itself uses mise for task orchestration. These commands are used throughout the plugin's development workflow.

### Core Commands

- **Tests**: `mise run test` - Run the project's test suite
- **Checks**: `mise run check` - Run quality checks (linting, formatting, type checking)
- **Build**: `mise run build` - Build/package the plugin
- **Run**: N/A - This is a plugin, not a runnable application

### Additional Commands

- **review:active**: `mise run review:active` - Find current active work directory
- **check-has-changes**: `mise run check-has-changes` - Verify there are uncommitted changes

**Note:** While CipherPowers itself uses mise, the plugin is tool-agnostic and works with any build/test tooling (npm, cargo, make, etc.). See `plugin/docs/configuring-project-commands.md` for details on the tool-agnostic approach.

## Architecture

CipherPowers combines three layers:

### 1. Skills Layer (`skills/`)

Reusable process knowledge documented as testable, discoverable guides for techniques and workflows.

**Key principles:**
- Written following TDD: test with subagents before writing
- Include rich `when_to_use` frontmatter for discovery
- Follow consistent SKILL.md structure

**Scope:**
- Organization-specific workflows and practices
- Universal skills that can be shared across projects
- Team-specific extensions and customizations

**Organization-specific skills:**

**Documentation:**
- **Maintaining docs** (`skills/maintaining-docs-after-changes/`) - Two-phase sync process
- **Capturing learning** (`skills/capturing-learning/`) - Retrospective capture process

**Meta:**
- **Algorithmic enforcement** (`skills/algorithmic-command-enforcement/`) - Why algorithms > imperatives
- **Using skills** (`skills/using-skills/`) - CipherPowers skill discovery

**Testing:**
- **TDD enforcement** (`skills/tdd-enforcement-algorithm/`) - Prevent code before tests

**Collaboration:**
- **Code review** (`skills/conducting-code-review/`) - Complete review workflow
- **Commit workflow** (`skills/commit-workflow/`) - Atomic commits with conventional format
- **Selecting agents** (`skills/selecting-agents/`) - Choose right agent for task

### 2. Automation Layer (`commands/`, `agents/`)

Commands and agents that dispatch to skills or provide project-specific workflows.

**Commands:** Slash commands users type
- CipherPowers commands: `/brainstorm`, `/plan`, `/execute`, `/code-review`, `/commit`, `/doc-review`, `/summarise`
- Thin dispatchers providing context
- Reference practices for project-specific configuration
- Reference skills for process guidance
- Do NOT contain workflow logic (that's in agents)
- Some commands (like `/execute`) orchestrate main Claude context with agent dispatch

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
- `plugin/templates/agent-template.md` - Agent structure with persuasion principles
- `plugin/templates/practice-template.md` - Practice structure with standards + config pattern
- `plugin/templates/skill-template.md` - Practice structure with standards + config pattern
- `plugin/templates/code-review-template.md` - Code review structure with standards + config pattern

### 3. Documentation Layer (`docs/`)

Standards, guidelines, and reference materials.

**Practices:** Coding standards, conventions, guidelines
**Examples:** Real-world examples and templates
**Purpose:** Support skills and provide team reference

## Organizational Benefits

This three-layer separation achieves key software engineering principles:

✅ **DRY (Don't Repeat Yourself)**
- Standards live in one place (`plugin/principles/`, `plugin/standards/`)
- Skills reference practices instead of duplicating them
- Commands reference skills instead of reimplementing workflows
- Changes propagate automatically through references

✅ **SRP (Single Responsibility Principle)**
- **Practices** define standards (WHAT to follow)
- **Skills** define workflows (HOW to do it)
- **Commands** dispatch to skills (WHEN to invoke)
- Each component has exactly one reason to change

✅ **Reusability**
- Skills are universal workflows (portable, can be shared across projects)
- Practices are project-specific standards (customized for your team)
- Commands add project context to universal workflows
- Skills can be reused across different teams and organizations

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
- `skills/conducting-code-review/SKILL.md` = Complete workflow (test verification, structured feedback, work directory save)
- `plugin/standards/code-review.md` = Standards (severity levels) + Project Config (commands, file conventions)
- `plugin/agents/code-reviewer.md` = Workflow enforcement with persuasion principles (non-negotiable steps, rationalization defenses)
- `plugin/commands/code-review.md` = Thin dispatcher (sets context, references skill)

All components work together without duplication:
- Update severity standards in practices → agent uses new standards automatically
- Change project commands (mise run test) → skill/agent reference practice for current command
- Update workflow in skill → agent enforces updated workflow
- Commands remain simple dispatchers → workflow discovery via skills

**Example: Commit Workflow**
- `skills/commit-workflow/SKILL.md` = Complete workflow (pre-commit checks, atomic commits, conventional format)
- `plugin/standards/conventional-commits.md` = Commit message format standards
- `plugin/standards/git-guidelines.md` = Git workflow standards
- `plugin/commands/commit.md` = Thin dispatcher (references skill)

Skills enable discovery:
- Claude Code's native Skill tool discovers all skills automatically
- Agents reference skills directly: `${CLAUDE_PLUGIN_ROOT}/skills/commit-workflow/SKILL.md`
- No need to hardcode commit knowledge into every agent
- Update workflow in skill → all agents benefit

**Example: Documentation Structure**
- `plugin/standards/documentation.md` = Standards (formatting, completeness, structure)
- `skills/maintaining-docs-after-changes/` = Workflow (two-phase sync process)
- `skills/capturing-learning/` = Workflow (retrospective capture process)
- `plugin/commands/doc-review.md` = Dispatcher (triggers maintenance workflow with project context)
- `plugin/commands/summarise.md` = Dispatcher (triggers learning capture with work tracking integration)

All five components work together without duplication. Change documentation standards once, both skills and commands use the updated version automatically.

**Example: Plan Execution with Automatic Agent Selection**
- `plugin/commands/execute.md` = Orchestrator command (algorithmic decision tree for when to use, hybrid agent selection, batch execution)
- `${CLAUDE_PLUGIN_ROOT}skills/executing-plans/SKILL.md` = Core workflow (batch pattern, verification)
- `plugin/skills/selecting-agents/SKILL.md` = Agent selection guide (characteristics, scenarios)
- `plugin/standards/code-review.md` = Review standards referenced at batch checkpoints
- Specialized agents (rust-engineer, ultrathink-debugger, code-reviewer, technical-writer, retrospective-writer)

The /execute command demonstrates:
- Algorithmic format for workflow enforcement (100% compliance vs 0-33% imperative)
- Hybrid agent selection (keyword matching + LLM analysis + user confirmation)
- Integration of multiple agents in coordinated workflow
- Automatic code review checkpoints after each batch
- Retrospective prompting when work completes

## Environment Variables

**CLAUDE_PLUGIN_ROOT**: Path to the cipherpowers plugin installation
- Set automatically when plugin is loaded (value: `${PLUGIN_DIR}`)
- Points to `plugin/` directory due to marketplace.json `"source": "./plugin/"`
- Use in agents/commands for practice references: `@${CLAUDE_PLUGIN_ROOT}standards/name.md`
- Use for all plugin-relative paths in commands and agents

**CIPHERPOWERS_MARKETPLACE_ROOT**: (Optional) Path to marketplace installation for shared practices
- Set if using cipherpowers as a local marketplace
- Used for accessing shared practices from marketplace

## Directory Structure

CipherPowers uses a clear separation between project documentation and plugin content:

**`./docs/` - Project Documentation**
- Documentation about cipherpowers itself (the project)
- Planning documents, analysis, research
- Historical records and development notes
- NOT shipped with plugin
- Lives in project repository root

**`./plugin/` - Plugin Content**
- All content shipped with the plugin to users
- **`plugin/principles/`, `plugin/standards/`** - Coding standards, conventions, guidelines
- **`plugin/templates/`** - Templates for agents, practices, skills
- **`plugin/agents/`** - Specialized subagent prompts
- **`plugin/commands/`** - Slash commands
- **`plugin/skills/`** - Organization-specific skills
- **`plugin/hooks/`** - Quality enforcement hooks (PostToolUse, SubagentStop)
- **`plugin/examples/`** - Example configurations and templates

**Key distinction:**
- `./docs/plans/` = Plans for building cipherpowers
- `./plugin/standards/` = Standards for users of cipherpowers

**Referencing paths**
In Claude Code, the ${CLAUDE_PLUGIN_ROOT} environment variable is crucial for referencing paths in plugin commands. Due to marketplace.json configuration (`"source": "./plugin/"`), this variable points directly to the plugin/ directory, so paths should NOT include the /plugin/ prefix.

You MUST ALWAYS use the following structure to reference paths:

    ${CLAUDE_PLUGIN_ROOT}path/to/file

Example:

    ${CLAUDE_PLUGIN_ROOT}standards/code-review.md


## Skills and Practices Discovery

**Skill Discovery:**
- Skills are automatically discovered by Claude Code
- Use the Skill tool in conversations: `Skill(command: "cipherpowers:skill-name")`
- No manual discovery scripts needed
- All skills in `plugin/skills/` are automatically available

**Practices Discovery:**
Browse `plugin/standards/` directory directly. Each practice includes YAML frontmatter with:
- `name`: Practice name
- `description`: Brief description
- `when_to_use`: Guidance on when to apply
- `applies_to`: Scope (all projects, specific languages, etc.)

**Direct References:**
Commands and agents reference skills and practices using environment variables:
- `@${CLAUDE_PLUGIN_ROOT}standards/practice-name.md` - Direct practice reference
- Skills are invoked via Skill tool, not direct file references

## Quality Hooks

CipherPowers provides automated quality enforcement through Claude Code's hook system.

**Hook Points:**
- **PostToolUse**: Runs after code editing tools (Edit, Write, etc.)
- **SubagentStop**: Runs when specialized agents complete their work

**Configuration:**
Quality hooks use project-level `gates.json` configuration files with priority:
1. `.claude/gates.json` (recommended - project-specific)
2. `gates.json` (project root)
3. `${CLAUDE_PLUGIN_ROOT}/hooks/gates.json` (plugin default fallback)

**Gate Actions:**
- **CONTINUE**: Proceed (default on pass)
- **BLOCK**: Prevent agent from proceeding (default on fail)
- **STOP**: Stop Claude entirely
- **{gate_name}**: Chain to another gate (subroutine call)

**Setup:**
```bash
# Copy example configuration
mkdir -p .claude
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json .claude/gates.json

# Customize for project's build tooling
vim .claude/gates.json
```

**Example configurations:**
- `examples/strict.json` - Block on all failures (production code)
- `examples/permissive.json` - Warn only (prototyping)
- `examples/pipeline.json` - Chained gates (format → check → test)

**Documentation:**
- `plugin/hooks/README.md` - Overview and examples
- `plugin/hooks/SETUP.md` - Configuration guide
- `plugin/hooks/INTEGRATION_TESTS.md` - Testing procedures

**Benefits:**
- Consistent quality enforcement across all agent work
- Early issue detection at edit time or completion
- Flexible per-gate actions (enforce, warn, stop, chain)
- Tool-agnostic (works with npm, cargo, mise, make, etc.)
- Self-contained project-level configuration

## Working with Skills in this Repository

When creating or editing skills in `plugin/skills/`:

1. **Read the meta-skill:** `${CLAUDE_PLUGIN_ROOT}skills/writing-skills/SKILL.md`
2. **Follow TDD:** Test with subagents BEFORE writing
3. **Use TodoWrite:** Create todos for the skill creation checklist
4. **Consider sharing:** Universal skills can be shared across projects and teams
5. **Skills are auto-discovered:** Once created in `plugin/skills/`, they're automatically available via the Skill tool

## Creating Agents and Practices

**When creating agents:**
1. Use `${CLAUDE_PLUGIN_ROOT}templates/agent-template.md` as starting point
2. Include all four persuasion principles (Authority, Commitment, Scarcity, Social Proof)
3. Reference practices for project-specific configuration (don't hardcode commands)
4. Reference skills for methodology
5. Make workflows non-negotiable with explicit rationalization defenses

**When creating practices:**
1. Use `${CLAUDE_PLUGIN_ROOT}templates/practice-template.md` as starting point
2. Separate universal standards from project-specific configuration
3. Standards section: What quality looks like (universal principles)
4. Project Configuration section: Commands, file conventions, tool settings
5. Agents reference practices, not the other way around

## Plugin Development

When developing CipherPowers plugin components:

**Directory Structure:**
- `plugin/commands/` - Slash commands (thin dispatchers)
- `plugin/agents/` - Specialized subagent prompts with enforced workflows
- `plugin/principles/`, `plugin/standards/` - Standards and project configuration
- `plugin/skills/` - Organization-specific skills
- `plugin/hooks/` - Quality enforcement hooks (PostToolUse, SubagentStop)
- `plugin/templates/` - Templates for agents, practices, and skills
- `plugin/examples/` - Example configurations and templates

**Key Principles:**
- Commands are thin dispatchers that reference agents or skills
- Agents enforce workflows using persuasion principles (Authority, Commitment, Scarcity, Social Proof)
- Practices separate universal standards from project-specific configuration
- Skills follow TDD approach with test scenarios
- Use templates (`${CLAUDE_PLUGIN_ROOT}templates/`) as starting points

**Development Workflow:**
1. Start with templates from `${CLAUDE_PLUGIN_ROOT}templates/`
2. For skills: Follow TDD approach with test scenarios before implementation
3. For agents: Include all four persuasion principles (Authority, Commitment, Scarcity, Social Proof)
4. For practices: Separate universal standards from project-specific configuration
5. Skills are auto-discovered; practices can be browsed in `plugin/standards/`
6. Ensure proper references using environment variables

**Environment Variables:**
- Use `${CLAUDE_PLUGIN_ROOT}` for plugin-relative paths
- This enables proper path resolution in all contexts

**Plugin Structure Best Practices:**
- Keep commands thin - they should only dispatch to agents or skills
- Put workflow logic in agents, not commands
- Reference practices for project-specific configuration (don't hardcode)
- Make agent workflows non-negotiable with explicit rationalization defenses
- Skills should be universal and reusable across projects and teams

## Learning and Retrospectives

CipherPowers captures significant lessons from development work to build organizational knowledge.

**When to capture:**
- After completing significant features
- When multiple approaches were tried
- When work took longer than expected
- When discovering non-obvious insights
- See `plugin/skills/capturing-learning/SKILL.md` for methodology


## Team Usage

1. Install cipherpowers as a Claude Code plugin
2. Commands dispatch to agents or main Claude with practice context

