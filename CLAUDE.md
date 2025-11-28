---
# CipherPowers uses mise for task orchestration
# See mise.toml for available tasks
---

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**Plugin Architecture Overview:** CipherPowers implements a three-layer plugin architecture that separates concerns for maintainability and reusability. Skills provide reusable workflows, automation provides commands and agents, and documentation provides practices and standards.

## CipherPowers

CipherPowers is a Claude Code plugin providing a comprehensive toolkit for development teams. Built on a three-layer plugin architecture, it separates skills (reusable workflows), automation (commands and agents), and documentation (practices and standards) to ensure team-wide consistency and maintainability.

## Development Commands

CipherPowers itself uses mise for task orchestration. These commands are used throughout the plugin's development workflow.

### Core Commands

- **Run**: N/A - This is a plugin, not a runnable application

**Note:** CipherPowers uses mise for task orchestration, but the specific tasks are project-specific. See `mise.toml` for available tasks (e.g., check-has-changes, check-tests-exist, check-docs-updated, check-atomic-commit, build-workflow, setup).

### Additional Commands

- **check-has-changes**: `mise run check-has-changes` - Verify there are uncommitted changes

**Note:** While CipherPowers itself uses mise, the plugin is tool-agnostic and works with any build/test tooling (npm, cargo, make, etc.). See `docs/configuring-project-commands.md` for details on the tool-agnostic approach.

## Architecture

CipherPowers combines three layers:

### 1. Skills Layer (`./plugin/skills/`)

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
- **Maintaining docs** (`./plugin/skills/maintaining-docs-after-changes/`) - Two-phase sync process
- **Capturing learning** (`./plugin/skills/capturing-learning/`) - Retrospective capture process

**Meta:**
- **Algorithmic enforcement** (`./plugin/skills/algorithmic-command-enforcement/`) - Why algorithms > imperatives
- **Using skills** (`./plugin/skills/using-cipherpowers/`) - CipherPowers skill discovery

**Testing:**
- **TDD enforcement** (`./plugin/skills/tdd-enforcement-algorithm/`) - Prevent code before tests

**Collaboration:**
- **Code review** (`./plugin/skills/conducting-code-review/`) - Complete review workflow
- **Commit workflow** (`./plugin/skills/commit-workflow/`) - Atomic commits with conventional format
- **Selecting agents** (`./plugin/skills/selecting-agents/`) - Choose right agent for task

### 2. Automation Layer (`./plugin/commands/`, `./plugin/agents/`)

Commands and agents that dispatch to skills or provide project-specific workflows.

**Commands:** Slash commands users type
- CipherPowers commands: `/cipherpowers:brainstorm`, `/cipherpowers:plan`, `/cipherpowers:execute`, `/cipherpowers:code-review`, `/cipherpowers:commit`, `/cipherpowers:verify`, `/cipherpowers:summarise`
- Thin dispatchers providing context
- Reference practices for project-specific configuration
- Reference skills for process guidance
- Do NOT contain workflow logic (that's in agents)
- Some commands (like `/cipherpowers:execute`) orchestrate main Claude context with agent dispatch

**Agents:** Specialized subagent prompts with enforced workflows
- Handle specific roles (code-reviewer, rust-engineer, ultrathink-debugger, technical-writer, and others)
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
- `./plugin/templates/agent-template.md` - Agent structure with persuasion principles
- `./plugin/templates/practice-template.md` - Practice structure with standards + config pattern
- `./plugin/templates/skill-template.md` - Skill structure with when_to_use frontmatter
- `./plugin/templates/code-review-template.md` - Code review structure with standards + config pattern
- `./plugin/templates/code-review-request.md` - Code review request structure
- `./plugin/templates/verify-template.md` - Verification review structure (for dual-verification reviews)
- `./plugin/templates/verify-plan-template.md` - Plan verification structure
- `./plugin/templates/verify-collation-template.md` - Collation report structure

### 3. Documentation Layer (`./plugin/standards/`, `./plugin/examples/`)

Standards, guidelines, and reference materials.

**Practices:** Coding standards, conventions, guidelines (in `./plugin/standards/`)
**Examples:** Real-world examples and templates (in `./plugin/examples/`)
**Purpose:** Support skills and provide team reference

## Organizational Benefits

This three-layer separation achieves key software engineering principles:

✅ **DRY (Don't Repeat Yourself)**
- Standards live in one place (`./plugin/principles/`, `./plugin/standards/`)
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
- Some skills include `test-scenarios.md` files as examples

✅ **Maintainability**
- Update standards once in practices, all skills benefit
- Change skill workflow without touching commands
- Add new commands without modifying skills
- Clear boundaries prevent coupling and drift

**Example: Code Review Workflow**
- `./plugin/skills/conducting-code-review/SKILL.md` = Complete workflow (test verification, structured feedback, work directory save)
- `./plugin/standards/code-review.md` = Standards (severity levels) + Project Config (commands, file conventions)
- `./plugin/agents/code-reviewer.md` = Workflow enforcement with persuasion principles (non-negotiable steps, rationalization defenses)
- `./plugin/commands/code-review.md` = Thin dispatcher (sets context, references skill)

All components work together without duplication:
- Update severity standards in practices → agent uses new standards automatically
- Change project commands (mise run test) → skill/agent reference practice for current command
- Update workflow in skill → agent enforces updated workflow
- Commands remain simple dispatchers → workflow discovery via skills

**Example: Commit Workflow**
- `./plugin/skills/commit-workflow/SKILL.md` = Complete workflow (pre-commit checks, atomic commits, conventional format)
- `./plugin/standards/conventional-commits.md` = Commit message format standards
- `./plugin/standards/git-guidelines.md` = Git workflow standards
- `./plugin/commands/commit.md` = Thin dispatcher (references skill)

Skills enable discovery:
- Claude Code's native Skill tool discovers all skills automatically
- Agents reference skills directly: `@${CLAUDE_PLUGIN_ROOT}skills/commit-workflow/SKILL.md`
- No need to hardcode commit knowledge into every agent
- Update workflow in skill → all agents benefit

**Example: Documentation Workflow**
- `./plugin/standards/documentation.md` = Standards (formatting, completeness, structure)
- `./plugin/skills/maintaining-docs-after-changes/` = Workflow (two-phase sync process)
- `./plugin/skills/capturing-learning/` = Workflow (retrospective capture process)
- `./plugin/agents/technical-writer.md` = Mode-aware agent (VERIFICATION or EXECUTION mode)
- `./plugin/commands/summarise.md` = Dispatcher (triggers learning capture with work tracking integration)

Documentation follows the standard verify → plan → execute pattern:
- `/cipherpowers:verify docs` → Dual technical-writers find issues (VERIFICATION mode)
- `/cipherpowers:plan` → Create fix plan if complex
- `/cipherpowers:execute` → Technical-writer applies fixes (EXECUTION mode)

All components work together without duplication. Change documentation standards once, all workflows use the updated version automatically.

**Example: Plan Execution with Automatic Agent Selection**
- `./plugin/commands/execute.md` = Orchestrator command (algorithmic decision tree for when to use, hybrid agent selection, batch execution)
- `${CLAUDE_PLUGIN_ROOT}skills/executing-plans/SKILL.md` = Core workflow (batch pattern, verification)
- `./plugin/skills/selecting-agents/SKILL.md` = Agent selection guide (characteristics, scenarios)
- `./plugin/standards/code-review.md` = Review standards referenced at batch checkpoints
- Specialized agents (commit-agent, code-agent, gatekeeper, plan-review-agent, rust-agent, ultrathink-debugger, code-review-agent, technical-writer)

The /cipherpowers:execute command demonstrates:
- Algorithmic format for workflow enforcement (100% compliance vs 0-33% imperative)
- Hybrid agent selection (keyword matching + LLM analysis + user confirmation)
- Integration of multiple agents in coordinated workflow
- Automatic code review checkpoints after each batch
- Optional execute completion verification via `/cipherpowers:verify execute` (on-demand, not automatic)
- Retrospective prompting when work completes

**Example: Verification Architecture with Shared Collation**
- `./plugin/commands/verify.md` = Generic dual-verification dispatcher (works for all verification types)
- `./plugin/skills/dual-verification-review/SKILL.md` = Core dual-verification pattern (Phase 1: dispatch 2 agents, Phase 2: collate, Phase 3: present)
- `./plugin/agents/review-collation-agent.md` = Generic collation agent (compares findings from any verification type)
- `./plugin/agents/research-agent.md` = Research verification agent (multi-angle exploration with evidence)
- Specialized agents: `execute-review-agent` (plan adherence), `plan-review-agent` (plan quality), `code-review-agent` (code quality), `research-agent` (research verification)

The verification architecture demonstrates:
- DRY principle: One collation agent serves all verification types (plan, code, execute, research, docs)
- Confidence levels: Common issues (VERY HIGH), Exclusive issues (MODERATE), Divergences (INVESTIGATE)
- Clear separation: Execute verification checks plan adherence, code verification checks quality/standards, research verification explores topics
- On-demand verification: All verification types are user-requested via `/cipherpowers:verify [type]`

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
- **`./plugin/principles/`** - Fundamental development philosophies (development.md, testing.md)
- **`./plugin/standards/`** - Project-specific conventions and practices
- **`./plugin/templates/`** - Templates for agents, practices, skills
- **`./plugin/agents/`** - Specialized subagent prompts
- **`./plugin/commands/`** - Slash commands
- **`./plugin/skills/`** - Organization-specific skills
- **`./plugin/hooks/`** - Gate configuration (gates.json only)
- **`./plugin/scripts/`** - Shell scripts (plan-compliance.sh)
- **`./plugin/docs/`** - Additional documentation (configuring-project-commands.md)
- **`./plugin/context/`** - Plugin-level context injection files (fallback defaults)
- **`./plugin/examples/`** - Example documentation (currently contains README.md)

**Key distinctions:**
- `./docs/` = Documentation about building cipherpowers itself (not shipped with plugin)
- `./plugin/standards/` = Standards for users of cipherpowers (shipped with plugin)

**Principles vs Standards:**
- `./plugin/principles/` contains fundamental development philosophies that are universal across all projects (e.g., development.md defines core development approach, testing.md defines testing philosophy)
- `./plugin/standards/` contains project-specific conventions and practices that teams can customize (e.g., code review severity levels, git workflows, documentation formats)

**Referencing paths in agent markdown files**

**Convention for agent files:**

Use `${CLAUDE_PLUGIN_ROOT}` with the @ syntax for file references:

```markdown
@${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md
@${CLAUDE_PLUGIN_ROOT}standards/code-review.md
@${CLAUDE_PLUGIN_ROOT}principles/development.md
```

**DO NOT use relative paths without the variable:**
```markdown
@skills/...  ❌ Does not work in subagent contexts (confirmed via testing)
```

**For JSON configurations (hooks, etc.):**

Use the full variable syntax:
```json
"${CLAUDE_PLUGIN_ROOT}/hooks/gates.json"
```

**Rationale:**
- `${CLAUDE_PLUGIN_ROOT}` expands correctly when agents are invoked
- marketplace.json `"source": "./plugin/"` means the variable points to plugin root
- Testing confirmed @ syntax without variable does NOT work in subagents
- Consistent with existing working agents


## Skills and Practices Discovery

**Skill Discovery:**
- Skills are automatically discovered by Claude Code
- Use the Skill tool in conversations: `Skill(command: "cipherpowers:skill-name")`
- No manual discovery scripts needed
- All skills in `./plugin/skills/` are automatically available

**Practices Discovery:**
Browse `./plugin/standards/` directory directly. Each practice includes YAML frontmatter with:
- `name`: Practice name
- `description`: Brief description
- `when_to_use`: Guidance on when to apply
- `applies_to`: Scope (all projects, specific languages, etc.)

**Direct References:**
Commands and agents reference skills and practices using environment variables:
- `@${CLAUDE_PLUGIN_ROOT}standards/practice-name.md` - Direct practice reference
- Skills are invoked via Skill tool, not direct file references

## Quality Hooks

CipherPowers provides gate configurations in `./plugin/hooks/gates.json`. The turboshovel plugin (required) provides the hooks runtime that executes these gates.

**Gates defined:**
- `plan-compliance` - Verify agents provide STATUS in completion reports (shell script at `./plugin/scripts/plan-compliance.sh`)

**Setup:**

1. Install turboshovel plugin (see turboshovel documentation for installation instructions)

2. CipherPowers gates.json will be automatically discovered by turboshovel

**Migration from older CipherPowers versions:**
- Previous versions included hooks implementation directly
- Now requires separate turboshovel plugin for hooks runtime
- Gate configurations remain in `./plugin/hooks/gates.json`
- Custom gates in old gates.json should be migrated manually (see gates.json.backup)

See turboshovel documentation for:
- Hook points and gate actions (CONTINUE, BLOCK, STOP, chaining)
- Convention-based context injection
- Full configuration guide

## Working with Skills in this Repository

When creating or editing skills in `./plugin/skills/`:

1. **Read the meta-skill:** `${CLAUDE_PLUGIN_ROOT}skills/writing-skills/SKILL.md`
2. **Follow TDD:** Test with subagents BEFORE writing
3. **Use TodoWrite:** Create todos for the skill creation checklist
4. **Consider sharing:** Universal skills can be shared across projects and teams
5. **Skills are auto-discovered:** Once created in `./plugin/skills/`, they're automatically available via the Skill tool

## Creating Agents and Practices

**When creating agents:**
1. Use `@${CLAUDE_PLUGIN_ROOT}templates/agent-template.md` as starting point
2. Include all four persuasion principles (Authority, Commitment, Scarcity, Social Proof)
3. Reference practices using `@${CLAUDE_PLUGIN_ROOT}standards/...` or `@${CLAUDE_PLUGIN_ROOT}principles/...` syntax
4. Reference skills using `@${CLAUDE_PLUGIN_ROOT}skills/...` syntax
5. Make workflows non-negotiable with explicit rationalization defenses

**When creating practices:**
1. Use `@${CLAUDE_PLUGIN_ROOT}templates/practice-template.md` as starting point
2. Separate universal standards from project-specific configuration
3. Standards section: What quality looks like (universal principles)
4. Project Configuration section: Commands, file conventions, tool settings
5. Agents reference practices, not the other way around

## Plugin Development

When developing CipherPowers plugin components:

**Directory Structure:**
- `./plugin/commands/` - Slash commands (thin dispatchers)
- `./plugin/agents/` - Specialized subagent prompts with enforced workflows
- `./plugin/principles/`, `./plugin/standards/` - Standards and project configuration
- `./plugin/skills/` - Organization-specific skills
- `./plugin/hooks/` - Gate configuration (gates.json only)
- `./plugin/scripts/` - Shell scripts (plan-compliance.sh)
- `./plugin/templates/` - Templates for agents, practices, and skills
- `./plugin/examples/` - Example documentation

**Key Principles:**
- Commands are thin dispatchers that reference agents or skills
- Agents enforce workflows using persuasion principles (Authority, Commitment, Scarcity, Social Proof)
- Practices separate universal standards from project-specific configuration
- Skills follow TDD approach with test scenarios
- Use templates (`@${CLAUDE_PLUGIN_ROOT}templates/`) as starting points

**Development Workflow:**
1. Start with templates from `@${CLAUDE_PLUGIN_ROOT}templates/` directory
2. For skills: Follow TDD approach with test scenarios before implementation
3. For agents: Include all four persuasion principles (Authority, Commitment, Scarcity, Social Proof)
4. For practices: Separate universal standards from project-specific configuration
5. Skills are auto-discovered; practices can be browsed in `./plugin/standards/`
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
- See `./plugin/skills/capturing-learning/SKILL.md` for methodology


## Team Usage

1. Install cipherpowers as a Claude Code plugin
2. Commands dispatch to agents or main Claude with practice context

