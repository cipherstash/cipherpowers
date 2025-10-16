# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## CipherPowers

CipherPowers is a Claude Code plugin providing a comprehensive toolkit for development teams. Built on a three-layer plugin architecture, it separates skills (reusable workflows), automation (commands and agents), and documentation (practices and standards) to ensure team-wide consistency and maintainability.

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
- `plugin/templates/agent-template.md` - Agent structure with persuasion principles
- `plugin/templates/practice-template.md` - Practice structure with standards + config pattern
- `plugin/templates/skill-template.md` - Practice structure with standards + config pattern

### 3. Documentation Layer (`docs/`)

Standards, guidelines, and reference materials.

**Practices:** Coding standards, conventions, guidelines
**Examples:** Real-world examples and templates
**Purpose:** Support skills and provide team reference

## Organizational Benefits

This three-layer separation achieves key software engineering principles:

✅ **DRY (Don't Repeat Yourself)**
- Standards live in one place (`plugin/practices/`)
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
- `skills/conducting-code-review/SKILL.md` = Complete workflow (test verification, structured feedback, work directory save)
- `plugin/practices/code-review.md` = Standards (severity levels) + Project Config (commands, file conventions)
- `plugin/agents/code-reviewer.md` = Workflow enforcement with persuasion principles (non-negotiable steps, rationalization defenses)
- `plugin/commands/code-review.md` = Thin dispatcher (sets context, references skill)
- Skills: References upstream "Requesting Code Review" and "Code Review Reception" skills

All components work together without duplication:
- Update severity standards in practices → agent uses new standards automatically
- Change project commands (mise run test) → skill/agent reference practice for current command
- Update workflow in skill → agent enforces updated workflow
- Commands remain simple dispatchers → workflow discovery via skills

**Example: Commit Workflow**
- `skills/commit-workflow/SKILL.md` = Complete workflow (pre-commit checks, atomic commits, conventional format)
- `plugin/practices/conventional-commits.md` = Commit message format standards
- `plugin/practices/git-guidelines.md` = Git workflow standards
- `plugin/commands/commit.md` = Thin dispatcher (references skill)

Skills enable discovery:
- Any agent can run `find-skills "commit"` and discover workflow
- No need to hardcode commit knowledge into every agent
- Update workflow in skill → all agents benefit

**Example: Documentation Structure**
- `plugin/practices/documentation.md` = Standards (formatting, completeness, structure)
- `skills/documentation/maintaining-docs-after-changes/` = Workflow (two-phase sync process)
- `skills/documentation/capturing-learning/` = Workflow (retrospective capture process)
- `plugin/commands/doc-review.md` = Dispatcher (triggers maintenance workflow with project context)
- `plugin/commands/summarise.md` = Dispatcher (triggers learning capture with work tracking integration)

All five components work together without duplication. Change documentation standards once, both skills and commands use the updated version automatically.

## Environment Variables

**CLAUDE_PLUGIN_ROOT**: Path to the cipherpowers plugin installation
- Set automatically when plugin is loaded (value: `${PLUGIN_DIR}`)
- Use in agents/commands for practice references: `@${CLAUDE_PLUGIN_ROOT}plugin/practices/name.md`
- Also used by `find-practices` tool for discovery

**SUPERPOWERS_SKILLS_ROOT**: Path to superpowers skills installation
- Provided by superpowers plugin
- Use for universal skill references: `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md`

**CIPHERPOWERS_MARKETPLACE_ROOT**: (Optional) Path to marketplace installation for shared practices
- Set if using cipherpowers as a local marketplace
- Enables `--upstream` flag in `find-practices`

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
- **`plugin/practices/`** - Coding standards, conventions, guidelines
- **`plugin/templates/`** - Templates for agents, practices, skills
- **`plugin/agents/`** - Specialized subagent prompts
- **`plugin/commands/`** - Slash commands
- **`plugin/skills/`** - Organization-specific skills
- **`plugin/tools/`** - Discovery and utility tools

**Key distinction:**
- `./docs/plans/` = Plans for building cipherpowers
- `./plugin/practices/` = Standards for users of cipherpowers

**Referencing paths**
n Claude Code, the ${CLAUDE_PLUGIN_ROOT} environment variable is crucial for referencing paths in plugin commands. This variable is intended to point to the root directory of the plugin, allowing commands to access scripts and resources relative to the plugin's location.
Command Structure

You MUST ALWAYS use the following structure to reference paths:

    ${CLAUDE_PLUGIN_ROOT}path/to/file

    ${CLAUDE_PLUGIN_ROOT}practices/code-review.md


## Integration with Superpowers

**Custom find-skills tool** (`${CLAUDE_PLUGIN_ROOT}tools/find-skills`):
- Searches `${CLAUDE_PLUGIN_ROOT}skills/` (org-specific)
- Searches `${SUPERPOWERS_SKILLS_ROOT}/skills/` (universal skills)
- Provides unified discovery across both collections
- Flags: `--local`, `--upstream`, or default (both)

**Custom find-practices tool** (`${CLAUDE_PLUGIN_ROOT}tools/find-practices`):
- Searches `${CLAUDE_PLUGIN_ROOT}plugin/practices/` (local practices)
- Searches `${CIPHERPOWERS_MARKETPLACE_ROOT}/plugin/practices/` (marketplace practices, if available)
- Extracts YAML frontmatter (name, description, when_to_use)
- Flags: `--local`, `--upstream`, or default (both)

**References:**
Commands and agents reference skills and practices transparently using standard paths.

## Using the Discovery Tools

**find-skills**: Discover available skills

```bash
# From repository root or via relative path
./plugin/tools/find-skills "search pattern"

# With scope flags
./plugin/tools/find-skills --local "pattern"      # cipherpowers only
./plugin/tools/find-skills --upstream "pattern"   # superpowers only
```

**find-practices**: Discover available practices

```bash
# From repository root or via relative path
./plugin/tools/find-practices "search pattern"

# With scope flags
./plugin/tools/find-practices --local "pattern"      # cipherpowers only
./plugin/tools/find-practices --upstream "pattern"   # marketplace only
```

**Direct references**: When you know the exact path

```markdown
# In agents or commands
${CLAUDE_PLUGIN_ROOT}practices/code-review.md
${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/brainstorming/SKILL.md
${CLAUDE_PLUGIN_ROOT}practices/code-review.md  # Relative to plugin root
${CLAUDE_PLUGIN_ROOT}conducting-code-review/SKILL.md  # Relative to plugin root
```

## Working with Skills in this Repository

When creating or editing skills in `skills/`:

1. **Read the meta-skill:** `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md`
2. **Follow TDD:** Test with subagents BEFORE writing
3. **Use TodoWrite:** Create todos for the skill creation checklist
4. **Consider upstream:** Universal skills may be contributed to superpowers later

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
- `plugin/practices/` - Standards and project configuration
- `plugin/skills/` - Organization-specific skills
- `plugin/templates/` - Templates for agents, practices, and skills
- `plugin/tools/` - Discovery and utility tools

**Key Principles:**
- Commands are thin dispatchers that reference agents or skills
- Agents enforce workflows using persuasion principles (Authority, Commitment, Scarcity, Social Proof)
- Practices separate universal standards from project-specific configuration
- Skills follow TDD approach with test scenarios
- Use templates (`${CLAUDE_PLUGIN_ROOT}templates/`) as starting points

**Environment Variables:**
- Use `${CLAUDE_PLUGIN_ROOT}` for plugin-relative paths
- Use `${SUPERPOWERS_SKILLS_ROOT}` for upstream skill references
- These enable proper path resolution in all contexts

## Team Usage

1. Install cipherpowers as a Claude Code plugin
2. Install superpowers for universal skills
3. Both collections work together seamlessly
4. Commands dispatch to agents or main Claude with practice context

