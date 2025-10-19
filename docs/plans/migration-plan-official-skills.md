# CipherPowers Migration Plan: Official Skills System

**Version:** 1.0.0
**Date:** 2025-10-18
**Status:** Draft for Review

## Executive Summary

This plan migrates CipherPowers from the legacy skills discovery pattern to Anthropic's official Skill tool mechanism (adopted by superpowers v3.0+). The migration is **non-breaking** and **incremental**, allowing existing workflows to continue while adding new capabilities.

### Key Changes

1. **Skills Registration** - Register 8 CipherPowers skills with `cipherpowers:*` namespace
2. **Skill Tool Invocation** - Migrate from manual Read tool to automatic Skill tool execution
3. **Practices Review** - Validate that practices remain distinct from skills (they do)
4. **Agent Updates** - Update agents to use Skill tool for workflow invocation
5. **Command Refinement** - Clarify command vs skill boundaries

### Migration Outcome

**Before:**
- Skills discovered via custom find-skills tool
- Manual Read tool + announce pattern
- Skills loaded into context manually

**After:**
- Skills registered in plugin manifest with `cipherpowers:*` namespace
- Automatic discovery via Skill tool's available_skills
- Invocation via `Skill("cipherpowers:skill-name")` loads and executes automatically
- Custom find-skills tool remains for exploration
- Practices remain separate (project-specific configuration + standards)

---

## Practices System Analysis

### Current Architecture ✅ VALIDATED AS SOUND

**Practices** serve a fundamentally different purpose from **Skills**:

| Dimension | Practices | Skills |
|-----------|-----------|--------|
| **Purpose** | Standards + Project Configuration | Workflows + Processes |
| **Scope** | Project-specific (commands, paths, tool settings) | Universal (portable, potentially upstreamable) |
| **Nature** | Declarative (WHAT quality looks like) | Procedural (HOW to achieve it) |
| **Discovery** | find-practices tool | Skill tool available_skills |
| **Invocation** | Referenced by agents/skills via `${CLAUDE_PLUGIN_ROOT}` | Executed via Skill tool |
| **Examples** | Severity levels, file conventions, test commands | Code review workflow, TDD process |

### Current Practices (10 total)

1. **code-review.md** - Review standards (4 severity levels) + file conventions
2. **conventional-commits.md** - Commit message format standards
3. **development.md** - Core principles (Simple, Consistent, Documented)
4. **documentation.md** - Documentation structure and formatting
5. **git-commit-algorithm.md** - Algorithmic readiness checks
6. **git-guidelines.md** - Git workflow standards
7. **rust/error-handling.md** - Rust error patterns
8. **rust/microsoft-rust-guidelines.md** - External Rust standards reference
9. **testing.md** - Test structure and verification
10. **workflow.md** - Work organization from research to docs

### Practice Reference Pattern (34 references found)

**Agents reference practices** for project-specific configuration:
```markdown
# In agents/code-reviewer.md
${CLAUDE_PLUGIN_ROOT}practices/code-review.md
${CLAUDE_PLUGIN_ROOT}practices/development.md
${CLAUDE_PLUGIN_ROOT}practices/testing.md
```

**Skills reference practices** for standards:
```markdown
# In skills/conducting-code-review/SKILL.md
${CLAUDE_PLUGIN_ROOT}practices/code-review.md  # For severity levels
```

### Recommendation: KEEP PRACTICES AS-IS ✅

**Rationale:**

1. **Complementary Purposes** - Practices define "what" (standards), skills define "how" (workflows)
2. **Proper Separation** - Skills are portable workflows, practices are project config
3. **DRY Achieved** - Single source of truth for standards (practices) referenced by multiple skills/agents
4. **Discovery Works** - find-practices tool provides unique value (not duplicated by Skill tool)
5. **Clear Boundaries** - Template structure (Standards + Project Config) enforces distinction

**No changes needed to practices architecture.** Focus migration on skills system only.

---

## Current Skills Inventory

### 8 Core Skills (to register)

| Skill Name | Namespace | Category | Description |
|------------|-----------|----------|-------------|
| using-skills | `cipherpowers:using-skills` | Meta | CipherPowers-specific skill discovery |
| selecting-agents | `cipherpowers:selecting-agents` | Collaboration | Agent selection decision guide |
| commit-workflow | `cipherpowers:commit-workflow` | Collaboration | Pre-commit checks, atomic commits, conventional format |
| conducting-code-review | `cipherpowers:conducting-code-review` | Collaboration | Complete review workflow with test verification |
| algorithmic-command-enforcement | `cipherpowers:algorithmic-command-enforcement` | Meta | Decision tree pattern for 100% compliance |
| tdd-enforcement-algorithm | `cipherpowers:tdd-enforcement-algorithm` | Testing | Binary checks preventing code before tests |
| maintaining-docs-after-changes | `cipherpowers:maintaining-docs-after-changes` | Documentation | Two-phase doc sync process |
| capturing-learning | `cipherpowers:capturing-learning` | Documentation | Retrospective capture workflow |

### Additional Files (not registerable skills)

- `plugin/skills/README.md` - Skills directory overview
- `plugin/skills/subagents.md` - Agent guidance (informational)
- `plugin/skills/restructuring-for-testability.md` - Guidance doc
- `plugin/skills/collaboration/emergency-stop/` - Special workflow (needs review)

---

## Migration Phases

### Phase 0: Prerequisites ✅ COMPLETE

- [x] Analyze superpowers v3.0+ migration approach
- [x] Understand Skill tool mechanism
- [x] Review practices architecture
- [x] Inventory current skills
- [x] Create migration plan

### Phase 1: Plugin Manifest & Registration

**Goal:** Register CipherPowers skills in plugin system without changing existing behavior.

#### 1.1 Create Skills Manifest

**File:** `.claude-plugin/skills.json` (or update existing manifest)

```json
{
  "skills": [
    {
      "name": "cipherpowers:using-skills",
      "path": "plugin/skills/using-skills/SKILL.md",
      "description": "CipherPowers-specific skill discovery using custom find-skills tool",
      "when_to_use": "when starting any conversation"
    },
    {
      "name": "cipherpowers:selecting-agents",
      "path": "plugin/skills/selecting-agents/SKILL.md",
      "description": "Decision guide for choosing the right specialized agent for each task type",
      "when_to_use": "before dispatching work to specialized agents, when multiple agents could apply"
    },
    {
      "name": "cipherpowers:commit-workflow",
      "path": "plugin/skills/commit-workflow/SKILL.md",
      "description": "Systematic commit process with pre-commit checks, atomic commits, and conventional commit messages",
      "when_to_use": "when committing code changes, before creating pull requests, when another agent needs to commit work"
    },
    {
      "name": "cipherpowers:conducting-code-review",
      "path": "plugin/skills/conducting-code-review/SKILL.md",
      "description": "Complete workflow for conducting thorough code reviews with test verification and structured feedback",
      "when_to_use": "when you have uncommitted changes OR completed work OR about to merge, to determine if code review is required. Also when conducting code review, when another agent asks you to review code, after being dispatched by requesting-code-review skill"
    },
    {
      "name": "cipherpowers:algorithmic-command-enforcement",
      "path": "plugin/skills/meta/algorithmic-command-enforcement/SKILL.md",
      "description": "Pattern for creating decision tree workflows that achieve 100% compliance vs 0-33% imperative compliance",
      "when_to_use": "when creating discipline-enforcing workflows, when agents ignore imperative instructions under pressure, when workflow compliance is critical"
    },
    {
      "name": "cipherpowers:tdd-enforcement-algorithm",
      "path": "plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md",
      "description": "Binary decision algorithm preventing implementation code before failing tests exist",
      "when_to_use": "when implementing any feature or bug fix, before writing any implementation code, when enforcing test-first discipline"
    },
    {
      "name": "cipherpowers:maintaining-docs-after-changes",
      "path": "plugin/skills/documentation/maintaining-docs-after-changes/SKILL.md",
      "description": "Two-phase workflow for syncing documentation with code changes",
      "when_to_use": "after code changes that affect documentation, when updating API endpoints or features, when architecture changes"
    },
    {
      "name": "cipherpowers:capturing-learning",
      "path": "plugin/skills/documentation/capturing-learning/SKILL.md",
      "description": "Workflow for capturing insights and decisions from completed work",
      "when_to_use": "after completing significant features or debugging, when multiple approaches were tried, when discovering non-obvious insights"
    }
  ]
}
```

**Testing:**
```bash
# After plugin reload, verify skills appear in available_skills
# In Claude Code session:
/help skills  # Should show cipherpowers:* skills
```

#### 1.2 Verify Registration

**Acceptance Criteria:**
- [ ] All 8 skills visible in Skill tool's available_skills
- [ ] Skills show correct `when_to_use` criteria
- [ ] Namespace `cipherpowers:*` distinguishes from `superpowers:*`
- [ ] Existing workflows continue to work (no breaking changes)

**Rollback:** Remove skills manifest, revert to manual discovery only.

---

### Phase 2: Update Meta-Skills

**Goal:** Document the new Skill tool invocation pattern while maintaining backward compatibility.

#### 2.1 Update using-skills/SKILL.md

**File:** `plugin/skills/using-skills/SKILL.md`

**Changes:**

1. **Add Skill Tool Section** (before existing content)

```markdown
## Using CipherPowers Skills

CipherPowers skills are registered with the official Claude Code Skill tool:

**Discovery:**
- Skills appear automatically in Skill tool's available_skills
- Or use `./plugin/tools/find-skills [PATTERN]` to search both repositories

**Invocation:**
```
Skill("cipherpowers:skill-name")
```

**Example:**
```
Skill("cipherpowers:commit-workflow")
```

The Skill tool automatically:
- Loads the skill content
- Announces skill usage
- Executes the workflow
```

2. **Update Discovery Workflow**

```markdown
## Discovery Workflow

**Option 1: Skill Tool (Recommended for Execution)**
1. Check available_skills in Skill tool
2. Invoke: `Skill("cipherpowers:skill-name")`
3. Follow the loaded workflow

**Option 2: find-skills (Recommended for Exploration)**
1. Run `./plugin/tools/find-skills [PATTERN]`
2. Use `--local` for cipherpowers only
3. Use `--upstream` for superpowers only
4. Read results for detailed exploration
```

3. **Maintain Backward Compatibility**

Keep existing Read tool documentation for agents that haven't migrated yet.

**Testing:**
- [ ] Document explains both invocation patterns
- [ ] Examples show correct namespace usage
- [ ] Backward compatibility preserved

#### 2.2 Update selecting-agents/SKILL.md

**File:** `plugin/skills/selecting-agents/SKILL.md`

**Changes:**

Update agent invocation examples to use Skill tool:

```markdown
**Before:**
Read and follow: `plugin/skills/commit-workflow/SKILL.md`

**After:**
Invoke skill: `Skill("cipherpowers:commit-workflow")`
```

**Testing:**
- [ ] Agent selection examples use Skill tool
- [ ] Namespace consistency maintained

---

### Phase 3: Update Commands

**Goal:** Clarify command vs skill boundaries and update dispatching patterns.

#### 3.1 Update All 7 Commands

**Files:**
- `plugin/commands/brainstorm.md`
- `plugin/commands/plan.md`
- `plugin/commands/execute.md`
- `plugin/commands/code-review.md`
- `plugin/commands/commit.md`
- `plugin/commands/doc-review.md`
- `plugin/commands/summarise.md`

**Pattern for each command:**

```markdown
# {Command Name}

{Description}

<instructions>
## Instructions

### Option 1: Direct Skill Invocation (For Agents)

Invoke the skill directly:
```
Skill("cipherpowers:{skill-name}")
```

### Option 2: Via Slash Command (For Users)

Use the slash command:
```
/{command-name}
```

### Option 3: Upstream Workflow (If Applicable)

For brainstorm/plan, can use superpowers directly:
```
SlashCommand("/superpowers:{command}")
```

**Why this structure?**
- Slash command = User-facing entry point
- Skill = Agent-invokable workflow with full context
- Flexible invocation depending on caller
</instructions>
```

**Example: commit.md**

```markdown
# Commit

Systematic commit process with pre-commit checks, atomic commits, and conventional commit messages.

<instructions>
## Instructions

### For Agents: Direct Skill Invocation

```
Skill("cipherpowers:commit-workflow")
```

This loads the complete workflow including:
- Pre-commit readiness checks
- Atomic commit verification
- Conventional commit message format
- References to practices for standards

### For Users: Slash Command

```
/commit
```

Dispatches main Claude to use commit-workflow skill.

### Workflow Components

**Skill:** `cipherpowers:commit-workflow` - Complete process
**Practices:**
- `conventional-commits.md` - Message format standards
- `git-guidelines.md` - Git workflow standards
- `git-commit-algorithm.md` - Readiness algorithm

**Why this structure?**
- Command = Thin dispatcher (entry point)
- Skill = Complete workflow (methodology)
- Practices = Project configuration (commands, standards)
</instructions>
```

**Testing for each command:**
- [ ] Documents both invocation methods
- [ ] References correct skill namespace
- [ ] Explains skill vs practice distinction
- [ ] Maintains backward compatibility

#### 3.2 Special Case: execute.md

**File:** `plugin/commands/execute.md`

**Note:** This command orchestrates in main context (not pure dispatcher). Update references:

```markdown
**Agent Selection:**
Uses `Skill("cipherpowers:selecting-agents")` for decision guide.

**Code Review Checkpoints:**
After each batch, considers invoking `Skill("cipherpowers:conducting-code-review")`.
```

**Testing:**
- [ ] Skill tool invocations correct
- [ ] Orchestration logic unchanged
- [ ] Batch execution still works

---

### Phase 4: Update Agents

**Goal:** Migrate agents from manual Read tool to Skill tool invocation.

#### 4.1 Update All 5 Agents

**Files:**
- `plugin/agents/code-reviewer.md`
- `plugin/agents/rust-engineer.md`
- `plugin/agents/technical-writer.md`
- `plugin/agents/retrospective-writer.md`
- `plugin/agents/ultrathink-debugger.md`

**Pattern for each agent:**

**Before:**
```markdown
<context>
  ## Context

  YOU MUST ALWAYS READ IN THIS ORDER:

  1. **Local Skill** (complete workflow):
     - Conducting Code Review: @skills/conducting-code-review/SKILL.md

  2. **Project Standards** (what quality looks like):
     - Code Review Standards: ${CLAUDE_PLUGIN_ROOT}practices/code-review.md
```

**After:**
```markdown
<context>
  ## Context

  YOU MUST ALWAYS LOAD IN THIS ORDER:

  1. **Upstream Skills** (universal methodology):
     - Requesting Code Review: Skill("superpowers:requesting-code-review")
     - Code Review Reception: Skill("superpowers:receiving-code-review")

  2. **Local Skill** (complete workflow):
     - Conducting Code Review: Skill("cipherpowers:conducting-code-review")

  3. **Project Standards** (what quality looks like):
     - Code Review Standards: @${CLAUDE_PLUGIN_ROOT}practices/code-review.md
     - Development Standards: @${CLAUDE_PLUGIN_ROOT}practices/development.md

  4. **Project Context**:
     - README: @README.md
     - Architecture: @CLAUDE.md
```

**Key Changes:**
1. Skills use `Skill()` tool invocation
2. Practices still use `@${CLAUDE_PLUGIN_ROOT}` references (they're not skills)
3. Project files still use `@` references
4. Clear distinction between skill loading and practice referencing

#### 4.2 Agent-Specific Updates

**code-reviewer.md:**
```markdown
### 2. Follow Conducting Code Review Skill

YOU MUST invoke and follow:
```
Skill("cipherpowers:conducting-code-review")
```

This skill defines:
- Step 1: Identify code to review
- Step 2: Run tests (references practices for commands)
- Step 3: Review against standards (references practices for severity)
- Step 4: Find work directory
- Step 5: Save structured review
```

**rust-engineer.md:**
```markdown
### Non-Negotiable: TDD First

Before ANY implementation code:
```
Skill("cipherpowers:tdd-enforcement-algorithm")
```

Or use upstream:
```
Skill("superpowers:test-driven-development")
```
```

**technical-writer.md:**
```markdown
### Documentation Workflow

After code changes:
```
Skill("cipherpowers:maintaining-docs-after-changes")
```

This two-phase process syncs docs with code.
```

**retrospective-writer.md:**
```markdown
### Learning Capture Workflow

After completing work:
```
Skill("cipherpowers:capturing-learning")
```

This workflow captures insights and decisions.
```

**ultrathink-debugger.md:**
```markdown
### Debugging Workflow

```
Skill("superpowers:systematic-debugging")
```

References practices for test commands and verification.
```

**Testing for each agent:**
- [ ] Skill invocations use correct namespace
- [ ] Practice references use `@${CLAUDE_PLUGIN_ROOT}`
- [ ] Project file references use `@`
- [ ] Non-negotiable workflows updated
- [ ] Rationalization defense tables still work

---

### Phase 5: Update Skill Cross-References

**Goal:** Update how skills reference other skills and practices.

#### 5.1 Update Skills That Reference Other Skills

**conducting-code-review/SKILL.md:**

```markdown
## Required Skills

**REQUIRED BACKGROUND:**
- Requesting Code Review: `Skill("superpowers:requesting-code-review")`
- Code Review Reception: `Skill("superpowers:receiving-code-review")`

**Complementary skills:**
- TDD Enforcement: `Skill("cipherpowers:tdd-enforcement-algorithm")`
- Systematic Debugging: `Skill("superpowers:systematic-debugging")`

## Required Practices

**Project Standards:**
- Code Review: `${CLAUDE_PLUGIN_ROOT}practices/code-review.md`
- Development: `${CLAUDE_PLUGIN_ROOT}practices/development.md`
- Testing: `${CLAUDE_PLUGIN_ROOT}practices/testing.md`
```

**commit-workflow/SKILL.md:**

```markdown
## Required Practices

**Standards:**
- Conventional Commits: `${CLAUDE_PLUGIN_ROOT}practices/conventional-commits.md`
- Git Guidelines: `${CLAUDE_PLUGIN_ROOT}practices/git-guidelines.md`
- Commit Algorithm: `${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md`

## Complementary Skills

- Conducting Code Review: `Skill("cipherpowers:conducting-code-review")`
- Verification Before Completion: `Skill("superpowers:verification-before-completion")`
```

**maintaining-docs-after-changes/SKILL.md:**

```markdown
## Required Practices

**Documentation Standards:**
- Documentation: `${CLAUDE_PLUGIN_ROOT}practices/documentation.md`

## Complementary Skills

- Capturing Learning: `Skill("cipherpowers:capturing-learning")`
```

**capturing-learning/SKILL.md:**

```markdown
## Required Practices

**Documentation Standards:**
- Documentation: `${CLAUDE_PLUGIN_ROOT}practices/documentation.md`
- Workflow: `${CLAUDE_PLUGIN_ROOT}practices/workflow.md`

## Complementary Skills

- Maintaining Docs: `Skill("cipherpowers:maintaining-docs-after-changes")`
```

**Testing:**
- [ ] All skill references use `Skill()` invocation format
- [ ] All practice references use `${CLAUDE_PLUGIN_ROOT}` path format
- [ ] Distinction between Required and Complementary maintained
- [ ] Namespace prefixes correct

#### 5.2 Update algorithmic-command-enforcement/SKILL.md

**Add Implementation Examples Section:**

```markdown
## Implemented Algorithms (CipherPowers)

### TDD Enforcement Algorithm
```
Skill("cipherpowers:tdd-enforcement-algorithm")
```
Binary checks preventing code before tests.

### Git Commit Readiness Algorithm
```
${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md
```
10-step algorithm in practice (project-specific commands).

### Code Review Trigger Algorithm
Section 1 of:
```
Skill("cipherpowers:conducting-code-review")
```
Binary commit + review status checks.
```

---

### Phase 6: Update Tools & Templates

**Goal:** Update discovery tools and templates to reflect new patterns.

#### 6.1 Update find-skills Tool

**File:** `plugin/tools/find-skills`

**Add header comment:**

```bash
#!/usr/bin/env bash
# CipherPowers find-skills tool
#
# PURPOSE: Discovery and exploration of skills across repositories
#
# NOTE: CipherPowers skills are also registered with the Skill tool.
# - Use find-skills for EXPLORATION (search, browse, understand)
# - Use Skill tool for EXECUTION (invoke workflows)
#
# INVOCATION PATTERNS:
# - Discovery: ./plugin/tools/find-skills "pattern"
# - Execution: Skill("cipherpowers:skill-name")
```

**Testing:**
- [ ] Tool still searches both repositories
- [ ] Header explains discovery vs execution
- [ ] Examples show both patterns

#### 6.2 Update skill-template.md

**File:** `plugin/templates/skill-template.md`

**Add sections:**

```markdown
---
name: skill-name-lowercase-kebab-case
description: Brief third-person description (1-2 sentences)
when_to_use: when [specific trigger condition requiring this skill]
version: 1.0.0
---

# Skill Name

## Overview

[Core principle in 1-2 sentences]

## Invocation

This skill is registered as:
```
Skill("cipherpowers:skill-name")
```

## Required Skills

**REQUIRED BACKGROUND:**
- Skill Name: `Skill("namespace:skill-name")`

**REQUIRED SUB-SKILL:**
- Skill Name: `Skill("namespace:skill-name")`

**Complementary skills:**
- Skill Name: `Skill("namespace:skill-name")`

## Required Practices

**Project Standards:**
- Practice Name: `${CLAUDE_PLUGIN_ROOT}practices/practice-name.md`

[Rest of skill template...]
```

**Testing:**
- [ ] Template shows invocation pattern
- [ ] Distinguishes skills from practices
- [ ] Examples use correct namespace format

#### 6.3 Update agent-template.md

**File:** `plugin/templates/agent-template.md`

```markdown
<context>
  ## Context

  YOU MUST ALWAYS LOAD IN THIS ORDER:

  1. **Upstream Skills** (universal methodology):
     - Skill Name: Skill("superpowers:skill-name")

  2. **Local Skills** (organization workflows):
     - Skill Name: Skill("cipherpowers:skill-name")

  3. **Project Standards** (what quality looks like):
     - Practice Name: @${CLAUDE_PLUGIN_ROOT}practices/practice-name.md

  4. **Project Context**:
     - README: @README.md
     - Architecture: @CLAUDE.md
</context>
```

**Testing:**
- [ ] Shows three distinct reference patterns (Skills/Practices/Files)
- [ ] Namespace examples correct
- [ ] Order preserved (Skills → Practices → Context)

---

### Phase 7: Update Documentation

**Goal:** Update all documentation to reflect new architecture.

#### 7.1 Update CLAUDE.md

**File:** `CLAUDE.md`

**Section: "Integration with Superpowers"**

```markdown
## Integration with Superpowers

**Skill Registration:**
- CipherPowers skills registered via plugin manifest
- Namespace: `cipherpowers:*` (distinguishes from `superpowers:*`)
- Discovery: Automatic via Skill tool's available_skills
- Invocation: `Skill("cipherpowers:skill-name")`

**Custom find-skills tool** (`./plugin/tools/find-skills`):
- Searches `plugin/skills/` (organization-specific)
- Searches `${SUPERPOWERS_SKILLS_ROOT}/skills/` (universal skills)
- Use for exploration and browsing
- Provides unified search across both collections

**Custom find-practices tool** (`./plugin/tools/find-practices`):
- Searches `plugin/practices/` (local practices)
- Searches `${CIPHERPOWERS_MARKETPLACE_ROOT}/plugin/practices/` (marketplace, if available)
- Practices are NOT skills (they're project configuration + standards)
- Use for discovering project-specific standards

**Invocation Patterns:**

| Type | Invocation | Example |
|------|------------|---------|
| **CipherPowers Skill** | `Skill("cipherpowers:name")` | `Skill("cipherpowers:commit-workflow")` |
| **Superpowers Skill** | `Skill("superpowers:name")` | `Skill("superpowers:brainstorming")` |
| **Practice** | `@${CLAUDE_PLUGIN_ROOT}practices/name.md` | `@${CLAUDE_PLUGIN_ROOT}practices/code-review.md` |
| **Project File** | `@filename.md` | `@README.md` |
```

**Section: "Skills Layer"**

```markdown
### 1. Skills Layer (`plugin/skills/`)

Reusable process knowledge documented using the superpowers framework. Skills are testable, discoverable guides for techniques and workflows.

**Registration:**
- All skills registered in plugin manifest
- Namespace: `cipherpowers:*`
- Discoverable via Skill tool

**Key principles:**
- Written following TDD: test with subagents before writing
- Include rich `when_to_use` frontmatter for discovery
- Follow superpowers SKILL.md structure
- Can reference upstream superpowers skills via `Skill("superpowers:*")`
- Reference practices for project-specific standards via `${CLAUDE_PLUGIN_ROOT}`

**Invocation:**
```
Skill("cipherpowers:skill-name")
```

**Scope:**
- Organization-specific workflows and practices
- Universal skills under development (before upstreaming)
- Extensions to superpowers skills for team context
```

**Section: "Using Discovery Tools"**

```markdown
## Discovery & Invocation

### Skills Discovery

**Option 1: Skill Tool (Automatic)**
- CipherPowers skills appear in available_skills
- Namespace: `cipherpowers:*`
- No manual search required

**Option 2: find-skills (Manual Exploration)**
```bash
./plugin/tools/find-skills "search pattern"
./plugin/tools/find-skills --local "pattern"      # cipherpowers only
./plugin/tools/find-skills --upstream "pattern"   # superpowers only
```

### Skills Invocation

**Direct execution:**
```
Skill("cipherpowers:skill-name")
Skill("superpowers:skill-name")
```

**Via slash command:**
```
/commit    # Dispatches to Skill("cipherpowers:commit-workflow")
/brainstorm  # Dispatches to Skill("superpowers:brainstorming")
```

### Practices Discovery

```bash
./plugin/tools/find-practices "search pattern"
./plugin/tools/find-practices --local "pattern"      # cipherpowers only
./plugin/tools/find-practices --upstream "pattern"   # marketplace only
```

### Practices Reference

**In agents/skills:**
```markdown
${CLAUDE_PLUGIN_ROOT}practices/practice-name.md
```

**Example:**
```markdown
${CLAUDE_PLUGIN_ROOT}practices/code-review.md
```
```

**Testing:**
- [ ] Invocation patterns documented
- [ ] Skill vs practice distinction clear
- [ ] Examples use correct syntax
- [ ] Discovery tools explained

#### 7.2 Update README.md

**File:** `README.md`

Add section after installation:

```markdown
## Using CipherPowers

### Skills

CipherPowers provides 8 organization-specific skills:

**Collaboration:**
- `cipherpowers:commit-workflow` - Systematic commit process
- `cipherpowers:conducting-code-review` - Complete review workflow
- `cipherpowers:selecting-agents` - Agent selection guide

**Documentation:**
- `cipherpowers:maintaining-docs-after-changes` - Doc sync workflow
- `cipherpowers:capturing-learning` - Retrospective capture

**Testing:**
- `cipherpowers:tdd-enforcement-algorithm` - Binary TDD checks

**Meta:**
- `cipherpowers:using-skills` - CipherPowers skill discovery
- `cipherpowers:algorithmic-command-enforcement` - Decision tree pattern

**Invocation:**
```
Skill("cipherpowers:skill-name")
```

**Discovery:**
```bash
./plugin/tools/find-skills "pattern"
```

### Practices

CipherPowers provides 10 project-specific practices defining standards and configuration:

- Code Review Standards
- Conventional Commits
- Development Principles
- Documentation Structure
- Git Guidelines
- Testing Standards
- Workflow Organization
- Rust Error Handling
- Git Commit Algorithm

**Discovery:**
```bash
./plugin/tools/find-practices "pattern"
```

**Reference in agents/skills:**
```markdown
${CLAUDE_PLUGIN_ROOT}practices/practice-name.md
```

### Slash Commands

User-facing entry points:

- `/brainstorm` - Design refinement
- `/plan` - Implementation planning
- `/execute` - Plan execution with agent dispatch
- `/code-review` - Review workflow
- `/commit` - Commit workflow
- `/doc-review` - Documentation maintenance
- `/summarise` - Retrospective capture
```

**Testing:**
- [ ] Clear sections for Skills, Practices, Commands
- [ ] Invocation examples correct
- [ ] Discovery tools documented

---

### Phase 8: Testing & Validation

**Goal:** Comprehensive testing of migrated system.

#### 8.1 Registration Testing

**Test Cases:**

1. **Skill Discovery**
   ```
   # In Claude Code session
   Check available_skills includes:
   - cipherpowers:using-skills
   - cipherpowers:selecting-agents
   - cipherpowers:commit-workflow
   - cipherpowers:conducting-code-review
   - cipherpowers:algorithmic-command-enforcement
   - cipherpowers:tdd-enforcement-algorithm
   - cipherpowers:maintaining-docs-after-changes
   - cipherpowers:capturing-learning
   ```

2. **Skill Invocation**
   ```
   Skill("cipherpowers:commit-workflow")
   # Should load and execute skill
   ```

3. **Namespace Isolation**
   ```
   # Verify cipherpowers:* distinct from superpowers:*
   Skill("cipherpowers:using-skills")  # CipherPowers version
   Skill("superpowers:using-skills")   # Superpowers version
   ```

**Acceptance:**
- [ ] All 8 skills appear in available_skills
- [ ] Skill invocation loads content automatically
- [ ] Namespaces don't conflict
- [ ] when_to_use criteria visible

#### 8.2 Agent Testing

**Test each agent:**

1. **code-reviewer**
   ```bash
   # Dispatch code-reviewer agent
   # Verify it:
   - Invokes Skill("cipherpowers:conducting-code-review")
   - References ${CLAUDE_PLUGIN_ROOT}practices/code-review.md
   - Follows complete workflow
   ```

2. **rust-engineer**
   ```bash
   # Dispatch rust-engineer agent
   # Verify it:
   - Invokes Skill("cipherpowers:tdd-enforcement-algorithm")
   - References ${CLAUDE_PLUGIN_ROOT}practices/development.md
   - Enforces TDD discipline
   ```

3. **technical-writer**
   ```bash
   # Dispatch technical-writer agent
   # Verify it:
   - Invokes Skill("cipherpowers:maintaining-docs-after-changes")
   - References ${CLAUDE_PLUGIN_ROOT}practices/documentation.md
   - Follows two-phase sync
   ```

4. **retrospective-writer**
   ```bash
   # Dispatch retrospective-writer agent
   # Verify it:
   - Invokes Skill("cipherpowers:capturing-learning")
   - References ${CLAUDE_PLUGIN_ROOT}practices/workflow.md
   - Captures insights correctly
   ```

5. **ultrathink-debugger**
   ```bash
   # Dispatch ultrathink-debugger agent
   # Verify it:
   - Invokes Skill("superpowers:systematic-debugging")
   - References ${CLAUDE_PLUGIN_ROOT}practices/testing.md
   - Follows debug workflow
   ```

**Acceptance:**
- [ ] All agents invoke skills correctly
- [ ] Practice references still work
- [ ] Workflows execute completely
- [ ] No regression in behavior

#### 8.3 Command Testing

**Test each command:**

1. `/commit`
   - [ ] Dispatches to Skill("cipherpowers:commit-workflow")
   - [ ] Follows algorithmic checks
   - [ ] References conventional-commits.md

2. `/code-review`
   - [ ] Dispatches to code-reviewer agent
   - [ ] Agent invokes conducting-code-review skill
   - [ ] Review file saved correctly

3. `/doc-review`
   - [ ] Dispatches to technical-writer agent
   - [ ] Agent invokes maintaining-docs skill
   - [ ] Docs synced with code

4. `/summarise`
   - [ ] Dispatches to retrospective-writer agent
   - [ ] Agent invokes capturing-learning skill
   - [ ] Learning captured

5. `/brainstorm`
   - [ ] Can dispatch to Skill("superpowers:brainstorming")
   - [ ] Workflow executes correctly

6. `/plan`
   - [ ] Can dispatch to Skill("superpowers:writing-plans")
   - [ ] Plan created correctly

7. `/execute`
   - [ ] Uses Skill("cipherpowers:selecting-agents")
   - [ ] Batch execution works
   - [ ] Code review checkpoints function

**Acceptance:**
- [ ] All commands work as before
- [ ] Skill dispatching correct
- [ ] No workflow regressions

#### 8.4 Discovery Tool Testing

**Test find-skills:**
```bash
./plugin/tools/find-skills "commit"
# Should find: cipherpowers:commit-workflow

./plugin/tools/find-skills --local "review"
# Should find: cipherpowers:conducting-code-review

./plugin/tools/find-skills --upstream "brainstorming"
# Should find: superpowers:brainstorming

./plugin/tools/find-skills "tdd"
# Should find both repos' TDD skills
```

**Test find-practices:**
```bash
./plugin/tools/find-practices "review"
# Should find: code-review.md

./plugin/tools/find-practices --local "git"
# Should find: git-guidelines.md, git-commit-algorithm.md, conventional-commits.md
```

**Acceptance:**
- [ ] find-skills searches both repositories
- [ ] Flags (--local, --upstream) work correctly
- [ ] find-practices searches practices only
- [ ] Output format unchanged

#### 8.5 Integration Testing

**End-to-end workflow tests:**

1. **Feature Development Flow**
   ```
   /brainstorm → /plan → /execute → /code-review → /commit → /summarise
   ```
   - [ ] Each command dispatches correctly
   - [ ] Skills invoke as expected
   - [ ] Practices referenced properly
   - [ ] Complete flow works end-to-end

2. **Documentation Flow**
   ```
   Code changes → /doc-review → /commit
   ```
   - [ ] technical-writer agent invoked
   - [ ] maintaining-docs skill executes
   - [ ] documentation.md practice applied
   - [ ] Commit workflow follows

3. **Review Flow**
   ```
   /code-review → address feedback → /code-review → /commit
   ```
   - [ ] code-reviewer agent invoked
   - [ ] conducting-code-review skill executes
   - [ ] code-review.md practice applied
   - [ ] Review file saved to work directory

**Acceptance:**
- [ ] All workflows function end-to-end
- [ ] No regressions vs pre-migration
- [ ] Skill/practice/command integration seamless

---

## Migration Checklist

### Phase 1: Registration ✅
- [ ] Create/update plugin manifest with skills registration
- [ ] Verify 8 skills appear in available_skills
- [ ] Test namespace isolation (cipherpowers:* vs superpowers:*)
- [ ] Confirm existing workflows still work

### Phase 2: Meta-Skills ✅
- [ ] Update using-skills/SKILL.md with Skill tool documentation
- [ ] Update selecting-agents/SKILL.md with Skill tool examples
- [ ] Test both skills invoke correctly
- [ ] Verify backward compatibility

### Phase 3: Commands ✅
- [ ] Update all 7 command files with new pattern
- [ ] Document dual invocation (slash + Skill tool)
- [ ] Special handling for execute.md orchestration
- [ ] Test each command works

### Phase 4: Agents ✅
- [ ] Update code-reviewer.md
- [ ] Update rust-engineer.md
- [ ] Update technical-writer.md
- [ ] Update retrospective-writer.md
- [ ] Update ultrathink-debugger.md
- [ ] Test each agent invokes skills correctly

### Phase 5: Skill Cross-References ✅
- [ ] Update conducting-code-review/SKILL.md
- [ ] Update commit-workflow/SKILL.md
- [ ] Update maintaining-docs-after-changes/SKILL.md
- [ ] Update capturing-learning/SKILL.md
- [ ] Update algorithmic-command-enforcement/SKILL.md
- [ ] Update tdd-enforcement-algorithm/SKILL.md
- [ ] Test skill-to-skill references work

### Phase 6: Tools & Templates ✅
- [ ] Update find-skills tool header
- [ ] Update skill-template.md
- [ ] Update agent-template.md
- [ ] Test templates generate correct format

### Phase 7: Documentation ✅
- [ ] Update CLAUDE.md architecture section
- [ ] Update CLAUDE.md integration section
- [ ] Update CLAUDE.md discovery section
- [ ] Update README.md with skills/practices/commands sections
- [ ] Test documentation accuracy

### Phase 8: Testing & Validation ✅
- [ ] Registration testing (8 test cases)
- [ ] Agent testing (5 agents)
- [ ] Command testing (7 commands)
- [ ] Discovery tool testing (find-skills, find-practices)
- [ ] Integration testing (3 end-to-end flows)

---

## Rollback Plan

If migration fails at any phase:

### Phase 1 Rollback
- Remove/revert plugin manifest
- Skills revert to manual discovery only
- No other changes needed (backward compatible)

### Phase 2-7 Rollback
- Revert file changes via git
- Plugin manifest can stay (doesn't break old pattern)
- Agents/commands continue using Read tool pattern

### Complete Rollback
```bash
git checkout main
git branch -D migration/official-skills
# Delete plugin manifest skills section
# Reload plugin
```

---

## Success Criteria

Migration is successful when:

✅ **All 8 skills registered** and discoverable via Skill tool
✅ **All 5 agents** invoke skills using Skill tool
✅ **All 7 commands** dispatch correctly using new pattern
✅ **All practices** continue to be referenced via ${CLAUDE_PLUGIN_ROOT}
✅ **Discovery tools** (find-skills, find-practices) work correctly
✅ **End-to-end workflows** function without regression
✅ **Documentation** accurately reflects new architecture
✅ **Backward compatibility** maintained (no breaking changes)
✅ **Tests pass** for registration, agents, commands, tools, integration

---

## Timeline Estimate

**Phase 1:** 1-2 hours (manifest creation, testing)
**Phase 2:** 1 hour (2 skills update)
**Phase 3:** 2 hours (7 commands update)
**Phase 4:** 3 hours (5 agents update + testing)
**Phase 5:** 2 hours (6 skills cross-reference update)
**Phase 6:** 1 hour (3 templates update)
**Phase 7:** 2 hours (documentation update)
**Phase 8:** 4 hours (comprehensive testing)

**Total:** 16-17 hours

Can be split across multiple sessions, tested incrementally.

---

## Post-Migration Recommendations

### 1. Monitor Adoption

Track which skills are invoked most frequently via available_skills vs find-skills.

### 2. Upstream Candidates

Consider contributing these skills to superpowers upstream:
- algorithmic-command-enforcement (universal pattern)
- tdd-enforcement-algorithm (variant of existing TDD skill)

### 3. Practices Evolution

Continue maintaining practices as project-specific configuration. Consider:
- Adding more practices as standards evolve
- Creating practice categories (rust/, testing/, git/, etc.)
- Marketplace sharing for practices (if beneficial)

### 4. New Skills Development

Use updated templates for any new skills:
- Follow new invocation pattern from start
- Register in manifest immediately
- Test with Skill tool during development

### 5. Documentation Maintenance

Keep CLAUDE.md and README.md in sync as:
- New skills added
- New practices created
- Architecture evolves

---

## Questions for Review

1. **Manifest Location:** Should skills manifest be `.claude-plugin/skills.json` or integrated into existing manifest?

2. **Namespace Granularity:** Keep flat `cipherpowers:*` or use categories like `cipherpowers:docs:*`, `cipherpowers:testing:*`?

3. **Emergency Stop Skill:** The `plugin/skills/collaboration/emergency-stop/` needs review - register or archive?

4. **Practice Discoverability:** Should practices YAML frontmatter be enhanced to match skill frontmatter format?

5. **Skill Visibility:** Should all 8 skills be user-visible in available_skills, or mark some as agent-only?

6. **Testing Strategy:** Run full test suite before merging, or test each phase incrementally?

---

## Conclusion

This migration plan provides a **non-breaking, incremental path** to adopt Anthropic's official Skill tool mechanism while preserving CipherPowers' proven three-layer architecture (Skills/Automation/Documentation).

**Key insight:** Practices serve a fundamentally different purpose from skills and should remain separate. The Skill tool handles workflow invocation; practices provide project-specific configuration and standards.

**Next step:** Review this plan, address questions, then execute phase-by-phase with testing at each stage.
