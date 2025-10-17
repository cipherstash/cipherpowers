# Migrate to Official Skills System Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Migrate CipherPowers from legacy manual skill discovery to Anthropic's official Skill tool mechanism, enabling automatic skill registration, discovery, and invocation while maintaining backward compatibility.

**Architecture:** Non-breaking incremental migration across 8 phases: (1) Create plugin manifest registering 8 skills with `cipherpowers:*` namespace, (2) Update meta-skills to document Skill tool usage, (3) Update 7 commands to support dual invocation, (4) Migrate 5 agents from Read tool to Skill tool, (5) Update skill cross-references, (6) Update templates, (7) Update documentation, (8) Comprehensive testing. Practices architecture validated as separate from skills (no changes needed).

**Tech Stack:** Claude Code plugin system, Bash scripts (find-skills, find-practices), Markdown (SKILL.md format with YAML frontmatter), JSON (plugin manifest)

---

## Task 1: Create Plugin Skills Manifest

**Files:**
- Create: `.claude-plugin/skills.json`
- Read: `plugin/skills/*/SKILL.md` (for frontmatter extraction)

**Step 1: Create skills manifest with all 8 skills**

Create `.claude-plugin/skills.json`:

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

**Step 2: Verify manifest syntax**

Run: `cat .claude-plugin/skills.json | jq .`
Expected: Valid JSON output with all 8 skills

**Step 3: Test variable resolution**

Create temporary test to verify environment variables work in Skill contexts:

```bash
# Test that CLAUDE_PLUGIN_ROOT resolves correctly
echo "Testing: ${CLAUDE_PLUGIN_ROOT}practices/code-review.md"
if [[ -f "${CLAUDE_PLUGIN_ROOT}practices/code-review.md" ]]; then
  echo "✓ CLAUDE_PLUGIN_ROOT resolves correctly"
else
  echo "✗ CLAUDE_PLUGIN_ROOT does not resolve - check plugin environment"
  exit 1
fi

# Test that SUPERPOWERS_SKILLS_ROOT resolves correctly
echo "Testing: ${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/brainstorming/SKILL.md"
if [[ -f "${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/brainstorming/SKILL.md" ]]; then
  echo "✓ SUPERPOWERS_SKILLS_ROOT resolves correctly"
else
  echo "✗ SUPERPOWERS_SKILLS_ROOT does not resolve - check superpowers installation"
  exit 1
fi
```

Expected: Both variables resolve to valid paths

**Step 4: Commit manifest**

```bash
git add .claude-plugin/skills.json
git commit -m "feat: register 8 CipherPowers skills in plugin manifest

- Add cipherpowers:using-skills
- Add cipherpowers:selecting-agents
- Add cipherpowers:commit-workflow
- Add cipherpowers:conducting-code-review
- Add cipherpowers:algorithmic-command-enforcement
- Add cipherpowers:tdd-enforcement-algorithm
- Add cipherpowers:maintaining-docs-after-changes
- Add cipherpowers:capturing-learning

Enables automatic skill discovery via Skill tool.
Non-breaking: maintains backward compatibility with manual discovery."
```

---

## Task 2: Update using-skills Meta-Skill

**Files:**
- Modify: `plugin/skills/using-skills/SKILL.md:1-50`

**Step 1: Add Skill Tool invocation section**

Insert after frontmatter, before "# Getting Started with Skills (CipherPowers)":

```markdown
# Getting Started with Skills (CipherPowers)

This skill extends the upstream superpowers using-skills skill with CipherPowers-specific tool paths and conventions.

**For complete usage instructions, see the upstream skill:**
`${SUPERPOWERS_SKILLS_ROOT}/skills/using-skills/SKILL.md`

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

**Discovery Workflow:**

**Option 1: Skill Tool (Recommended for Execution)**
1. Check available_skills in Skill tool
2. Invoke: `Skill("cipherpowers:skill-name")`
3. Follow the loaded workflow

**Option 2: find-skills (Recommended for Exploration)**
1. Run `./plugin/tools/find-skills [PATTERN]`
2. Use `--local` for cipherpowers only
3. Use `--upstream` for superpowers only
4. Read results for detailed exploration

## CipherPowers-Specific Tool Paths

**Tool paths (use these when you need to search for or run skills):**
- find-skills: ./plugin/tools/find-skills (searches BOTH cipherpowers and superpowers)
- skill-run: /Users/tobyhede/.config/superpowers/skills/skills/using-skills/skill-run

**Upstream reference paths:**
- upstream find-skills: /Users/tobyhede/.config/superpowers/skills/skills/using-skills/find-skills (superpowers only)
- superpowers skills: /Users/tobyhede/.config/superpowers/skills/skills/
```

**Step 2: Verify changes don't break frontmatter**

Run: `head -10 plugin/skills/using-skills/SKILL.md`
Expected: YAML frontmatter intact, new sections added

**Step 3: Commit using-skills update**

```bash
git add plugin/skills/using-skills/SKILL.md
git commit -m "docs: add Skill tool invocation to using-skills meta-skill

- Document Skill(\"cipherpowers:*\") invocation pattern
- Explain automatic discovery via available_skills
- Maintain backward compatibility with find-skills tool
- Clarify dual discovery workflow (Skill tool vs find-skills)"
```

---

## Rollback Strategy

If migration needs to be rolled back at any point:

```bash
# Find the commit before migration started
git log --oneline | grep "feat: register 8 CipherPowers skills"

# Reset to before migration (replace <commit-hash> with actual hash)
git reset --hard <commit-hash>^

# Or reset to specific task checkpoint
git log --oneline | grep "task you want to return to"
git reset --hard <commit-hash>
```

**Safe Rollback Points:**
- After Task 1: Manifest only, no breaking changes
- After Task 10: Commands updated, agents still use old pattern
- After Task 15: Agents migrated, skills not yet cross-referenced
- After Task 21: All functionality migrated, templates/docs not yet updated

**Verification After Rollback:**
```bash
# Verify old pattern still works
./plugin/tools/find-skills "commit"
cat plugin/agents/code-reviewer.md | head -30
```

---

## Task 3: Update selecting-agents Skill

**Files:**
- Modify: `plugin/skills/selecting-agents/SKILL.md:1-200`

**Step 1: Update invocation examples to use Skill tool**

Find and replace pattern throughout file:

Before:
```markdown
Read and follow: `plugin/skills/commit-workflow/SKILL.md`
```

After:
```markdown
Invoke skill: `Skill("cipherpowers:commit-workflow")`
```

Specific sections to update:
- "## Overview" section: General skill references
- "## Documentation Agents" section: technical-writer, retrospective-writer references
- "## Debugging Agents" section: ultrathink-debugger references
- "## Development Agents" section: rust-engineer, code-reviewer references
- "## Common Confusions" section: Examples showing which agent to use

**Step 2: Add Skill Tool section after frontmatter**

Insert after line 9 (after "# Selecting Agents"):

```markdown
## Skill Invocation Pattern

When this skill recommends using a specialized agent, invoke the agent which will then use its configured skills:

**CipherPowers skills use:**
```
Skill("cipherpowers:skill-name")
```

**Superpowers skills use:**
```
Skill("superpowers:skill-name")
```

**Agents are dispatched via Task tool, then agents invoke their skills.**
```

**Step 3: Verify markdown formatting**

Run: `grep -n "Skill(" plugin/skills/selecting-agents/SKILL.md | head -10`
Expected: Multiple lines showing Skill() invocations with correct namespace

**Step 4: Commit selecting-agents update**

```bash
git add plugin/skills/selecting-agents/SKILL.md
git commit -m "docs: update selecting-agents to use Skill tool invocation

- Replace manual Read tool pattern with Skill() invocation
- Add Skill Tool invocation section explaining pattern
- Update all examples to use cipherpowers:* namespace
- Maintain clarity on agent dispatch vs skill invocation"
```

---

## Task 4: Update brainstorm Command

**Files:**
- Modify: `plugin/commands/brainstorm.md:1-26`

**Step 1: Update command to document dual invocation**

Replace entire file content:

```markdown
# Brainstorm

Interactive design refinement using Socratic method to transform ideas into detailed designs.

<instructions>
## Instructions

### Option 1: Via Slash Command (For Users)

Use the slash command to start brainstorming:
```
/superpowers:brainstorm
```

This invokes the upstream superpowers brainstorming workflow.

### Option 2: Direct Skill Invocation (For Agents)

Agents can invoke the skill directly:
```
Skill("superpowers:brainstorming")
```

### Option 3: CipherPowers Wrapper

Use CipherPowers entry point:
```
/brainstorm
```

Which delegates to `/superpowers:brainstorm`.

### After Brainstorming

Create a plan using:
- `/plan` - CipherPowers plan command
- `/superpowers:write-plan` - Upstream superpowers workflow

**Why this structure?**
- Skill = Universal design refinement methodology (superpowers)
- Command = Thin wrapper (CipherPowers entry point)
- Integration = Seamless access to superpowers workflow

**Workflow Components:**
- **Skill:** `superpowers:brainstorming` - Complete Socratic process
- **Skill:** `superpowers:writing-plans` - Plan creation workflow
</instructions>
```

**Step 2: Verify command structure**

Run: `cat plugin/commands/brainstorm.md | grep -c "Skill("`
Expected: 2 (two Skill() invocation examples)

**Step 3: Commit brainstorm command update**

```bash
git add plugin/commands/brainstorm.md
git commit -m "docs: update brainstorm command with dual invocation pattern

- Document slash command invocation for users
- Document Skill tool invocation for agents
- Explain three-way integration (CipherPowers → superpowers)
- Clarify command vs skill distinction"
```

---

## Task 5: Update plan Command

**Files:**
- Modify: `plugin/commands/plan.md:1-30`

**Step 1: Update plan command with dual invocation**

Replace entire file content:

```markdown
# Plan

Create detailed implementation plans with bite-sized tasks for engineers with zero codebase context.

<instructions>
## Instructions

### Option 1: Via Slash Command (For Users)

Use the slash command to create a plan:
```
/superpowers:write-plan
```

This invokes the upstream superpowers plan writing workflow.

### Option 2: Direct Skill Invocation (For Agents)

Agents can invoke the skill directly:
```
Skill("superpowers:writing-plans")
```

### Option 3: CipherPowers Wrapper

Use CipherPowers entry point:
```
/plan
```

Which delegates to `/superpowers:write-plan`.

### After Planning

Execute the plan using:
- `/execute` - CipherPowers execution orchestrator
- `/superpowers:execute-plan` - Upstream superpowers executor

**Why this structure?**
- Skill = Universal plan writing methodology (superpowers)
- Command = Thin wrapper (CipherPowers entry point)
- Integration = Seamless access to superpowers workflow

**Workflow Components:**
- **Skill:** `superpowers:writing-plans` - Complete plan creation process
- **Skill:** `superpowers:executing-plans` - Batch execution workflow

**Plan Location:**
- Saved to: `docs/plans/YYYY-MM-DD-<feature-name>.md`
- Format: Bite-sized tasks (2-5 minutes each)
- Includes: Exact file paths, complete code, test commands
</instructions>
```

**Step 2: Verify command structure**

Run: `cat plugin/commands/plan.md | grep -c "Skill("`
Expected: 2 (two Skill() invocation examples)

**Step 3: Commit plan command update**

```bash
git add plugin/commands/plan.md
git commit -m "docs: update plan command with dual invocation pattern

- Document slash command invocation for users
- Document Skill tool invocation for agents
- Explain integration with superpowers workflow
- Clarify plan location and format expectations"
```

---

## Task 6: Update commit Command

**Files:**
- Modify: `plugin/commands/commit.md:1-35`

**Step 1: Update commit command with dual invocation**

Replace entire file content:

```markdown
# Commit

Systematic commit process with pre-commit checks, atomic commits, and conventional commit messages.

<instructions>
## Instructions

### Option 1: Direct Skill Invocation (For Agents)

Invoke the skill directly:
```
Skill("cipherpowers:commit-workflow")
```

This loads the complete workflow including:
- Pre-commit readiness checks (algorithmic)
- Atomic commit verification
- Conventional commit message format
- References to practices for standards

### Option 2: Via Slash Command (For Users)

Use the slash command:
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

**Pre-Commit Readiness Algorithm:**

The skill implements a 10-step algorithmic check:
1. Tests pass?
2. Checks pass?
3. Documentation updated?
4. Changes atomic?
5. No WIP markers?
6. No debug code?
7. No commented code?
8. Dependencies declared?
9. Commit message ready?
10. All files staged?

Prevents WIP commits, "will fix later", exhaustion-driven commits.
</instructions>
```

**Step 2: Verify command references**

Run: `grep -E "(Skill|practices)" plugin/commands/commit.md | wc -l`
Expected: At least 5 references

**Step 3: Commit commit command update**

```bash
git add plugin/commands/commit.md
git commit -m "docs: update commit command with Skill tool invocation

- Document Skill(\"cipherpowers:commit-workflow\") invocation
- Explain pre-commit algorithmic readiness checks
- Reference practices for standards (conventional-commits, git-guidelines)
- Clarify command → skill → practices flow"
```

---

## Task 7: Update code-review Command

**Files:**
- Modify: `plugin/commands/code-review.md:1-40`

**Step 1: Update code-review command**

Replace entire file content:

```markdown
# Code Review

Complete workflow for conducting thorough code reviews with test verification and structured feedback.

<instructions>
## Instructions

### Option 1: Via Slash Command (For Users)

Use the slash command:
```
/code-review
```

This dispatches the code-reviewer agent which invokes the review skill.

### Option 2: Direct Agent Dispatch (For Orchestrators)

Dispatch the agent via Task tool:
```
Task({
  subagent_type: "cipherpowers:code-reviewer",
  prompt: "Review the current code changes"
})
```

The agent then invokes:
```
Skill("cipherpowers:conducting-code-review")
```

### Workflow Components

**Agent:** `code-reviewer` - Enforces non-negotiable review workflow
**Skill:** `cipherpowers:conducting-code-review` - Complete review process
**Upstream Skills:**
- `superpowers:requesting-code-review` - When to request reviews
- `superpowers:receiving-code-review` - How to address feedback

**Practices:**
- `code-review.md` - Severity levels (1-4), file conventions
- `development.md` - Code quality standards
- `testing.md` - Test verification requirements

**Why this structure?**
- Command = User entry point
- Agent = Workflow enforcer (persuasion principles)
- Skill = Complete methodology
- Practices = Project-specific standards

**Review Workflow:**

1. Identify code to review (git commands)
2. Run all tests and checks (mandatory)
3. Review against ALL severity levels (1-4)
4. Find active work directory
5. Save structured feedback to work directory

**Output Location:**
- Pattern: `.work/{feature-name}/{YYYY-MM-DD}-review-{N}.md`
- Format: Structured by severity level
- Required: All 4 levels reviewed (empty sections OK)
</instructions>
```

**Step 2: Verify references**

Run: `grep -E "(Skill|Agent|practices)" plugin/commands/code-review.md | wc -l`
Expected: At least 8 references

**Step 3: Commit code-review command update**

```bash
git add plugin/commands/code-review.md
git commit -m "docs: update code-review command with multi-layer invocation

- Document slash command → agent → skill flow
- Explain Task tool agent dispatch
- Reference upstream skills (requesting/receiving code review)
- Reference practices for severity levels and standards
- Clarify command → agent → skill → practices architecture"
```

---

## Task 8: Update doc-review Command

**Files:**
- Modify: `plugin/commands/doc-review.md:1-35`

**Step 1: Update doc-review command**

Replace entire file content:

```markdown
# Doc Review

Two-phase workflow for syncing documentation with code changes.

<instructions>
## Instructions

### Option 1: Via Slash Command (For Users)

Use the slash command:
```
/doc-review
```

This dispatches the technical-writer agent which invokes the docs skill.

### Option 2: Direct Agent Dispatch (For Orchestrators)

Dispatch the agent via Task tool:
```
Task({
  subagent_type: "cipherpowers:technical-writer",
  prompt: "Review and update documentation after code changes"
})
```

The agent then invokes:
```
Skill("cipherpowers:maintaining-docs-after-changes")
```

### Workflow Components

**Agent:** `technical-writer` - Documentation maintenance specialist
**Skill:** `cipherpowers:maintaining-docs-after-changes` - Two-phase sync process
**Practice:** `documentation.md` - Documentation standards and structure

**Why this structure?**
- Command = User entry point
- Agent = Documentation specialist
- Skill = Complete sync methodology
- Practice = Documentation standards

**Two-Phase Workflow:**

**Phase 1: Discovery (Identify What Needs Updating)**
1. Analyze code changes (git diff)
2. Identify affected documentation
3. Check for new features/changes requiring docs
4. List documentation gaps

**Phase 2: Synchronization (Update Documentation)**
1. Update API documentation
2. Update README/guides
3. Update examples
4. Update configuration docs
5. Verify completeness

**Documentation Types:**
- API docs (function signatures, parameters)
- User guides (how-to, tutorials)
- Architecture docs (design decisions)
- Configuration (environment, settings)
- Examples (code samples, use cases)
</instructions>
```

**Step 2: Verify structure**

Run: `grep "Phase" plugin/commands/doc-review.md | wc -l`
Expected: 2 (two phases documented)

**Step 3: Commit doc-review command update**

```bash
git add plugin/commands/doc-review.md
git commit -m "docs: update doc-review command with agent→skill flow

- Document slash command → technical-writer agent → skill
- Explain two-phase sync workflow (Discovery → Synchronization)
- Reference documentation.md practice for standards
- Clarify documentation types requiring updates"
```

---

## Task 9: Update summarise Command

**Files:**
- Modify: `plugin/commands/summarise.md:1-35`

**Step 1: Update summarise command**

Replace entire file content:

```markdown
# Summarise

Workflow for capturing insights and decisions from completed work.

<instructions>
## Instructions

### Option 1: Via Slash Command (For Users)

Use the slash command:
```
/summarise
```

This dispatches the retrospective-writer agent which invokes the learning skill.

### Option 2: Direct Agent Dispatch (For Orchestrators)

Dispatch the agent via Task tool:
```
Task({
  subagent_type: "cipherpowers:retrospective-writer",
  prompt: "Capture learning from the completed work"
})
```

The agent then invokes:
```
Skill("cipherpowers:capturing-learning")
```

### Workflow Components

**Agent:** `retrospective-writer` - Learning capture specialist
**Skill:** `cipherpowers:capturing-learning` - Retrospective workflow
**Practices:**
- `documentation.md` - Documentation structure standards
- `workflow.md` - Work tracking integration

**Why this structure?**
- Command = User entry point
- Agent = Retrospective specialist
- Skill = Complete learning capture methodology
- Practices = Documentation and workflow standards

**Learning Capture Workflow:**

1. Review completed work
   - Check git commits
   - Review task descriptions
   - Identify key changes

2. Identify significant insights
   - What was learned?
   - What was tried?
   - What worked/didn't work?
   - What would you do differently?

3. Capture decisions
   - Why this approach?
   - What alternatives considered?
   - What constraints influenced decision?
   - What are implications?

4. Document for future
   - Save to `docs/learning/YYYY-MM-DD-{topic}.md`
   - Include context and rationale
   - Link to related work
   - Tag for discoverability

**When to Capture Learning:**
- After completing significant features
- After complex debugging sessions
- When multiple approaches were tried
- When discovering non-obvious insights
- When work took longer than expected
</instructions>
```

**Step 2: Verify workflow documentation**

Run: `grep -E "^[0-9]\." plugin/commands/summarise.md | wc -l`
Expected: 4 (four workflow steps)

**Step 3: Commit summarise command update**

```bash
git add plugin/commands/summarise.md
git commit -m "docs: update summarise command with learning capture flow

- Document slash command → retrospective-writer → skill
- Explain four-step learning capture workflow
- Reference documentation.md and workflow.md practices
- Clarify when to capture learning (trigger conditions)"
```

---

## Task 10: Update execute Command

**Files:**
- Modify: `plugin/commands/execute.md:150-180`

**Step 1: Update skill references in execute command**

Find sections referencing skills and update to use Skill tool invocation:

Location ~Line 150 (Agent Selection section):

Before:
```markdown
Uses `plugin/skills/selecting-agents/SKILL.md` for decision guide.
```

After:
```markdown
**Agent Selection:**
Uses `Skill("cipherpowers:selecting-agents")` for decision guide.
```

Location ~Line 165 (Code Review Checkpoints):

Before:
```markdown
After each batch, considers code review per practices.
```

After:
```markdown
**Code Review Checkpoints:**
After each batch, considers invoking:
```
Skill("cipherpowers:conducting-code-review")
```
Via code-reviewer agent if changes are significant.
```

**Step 2: Add Skill Tool reference section**

Insert after line ~30 (after orchestration description):

```markdown
### Skill Integration

This command integrates multiple skills:

**Planning:**
- Reads plan created by `Skill("superpowers:writing-plans")`

**Execution:**
- Uses `Skill("superpowers:executing-plans")` pattern for batching

**Agent Selection:**
- Uses `Skill("cipherpowers:selecting-agents")` for hybrid selection

**Code Review:**
- Invokes code-reviewer agent → `Skill("cipherpowers:conducting-code-review")`

**Retrospective:**
- Prompts for `Skill("cipherpowers:capturing-learning")` when complete
```

**Step 3: Verify Skill references**

Run: `grep -c "Skill(" plugin/commands/execute.md`
Expected: At least 5 references

**Step 4: Commit execute command update**

```bash
git add plugin/commands/execute.md
git commit -m "docs: update execute command Skill tool references

- Update selecting-agents reference to use Skill() invocation
- Update code review checkpoint to use Skill() invocation
- Add Skill Integration section showing all skill touchpoints
- Document orchestration of multiple skills in workflow"
```

---

## Task 11: Update code-reviewer Agent

**Files:**
- Modify: `plugin/agents/code-reviewer.md:10-65`

**Step 1: Update context section to use Skill tool**

Replace lines 10-30 (context section):

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
       - Testing Standards: @${CLAUDE_PLUGIN_ROOT}practices/testing.md

    4. **Project Context**:
       - README: @README.md
       - Architecture: @CLAUDE.md
  </context>
```

**Step 2: Update workflow section to use Skill invocation**

Replace lines 51-63 (workflow section):

```markdown
    ### 2. Follow Conducting Code Review Skill

    YOU MUST invoke and follow:
    ```
    Skill("cipherpowers:conducting-code-review")
    ```

    This skill defines:
    - Step 1: Identify code to review (git commands)
    - Step 2: Run tests (references practices for commands)
    - Step 3: Review against standards (references practices for severity)
    - Step 4: Find work directory (references practices for conventions)
    - Step 5: Save structured review (references practices for template)

    **The skill defines HOW. You enforce that it gets done.**
```

**Step 3: Verify agent references**

Run: `grep -E "(Skill|@\\$\{CLAUDE_PLUGIN_ROOT\})" plugin/agents/code-reviewer.md | wc -l`
Expected: At least 6 references (3 skills + 3 practices)

**Step 4: Commit code-reviewer agent update**

```bash
git add plugin/agents/code-reviewer.md
git commit -m "refactor: migrate code-reviewer agent to Skill tool

- Update context section to use Skill() for upstream skills
- Update context section to use Skill() for conducting-code-review
- Maintain @\${CLAUDE_PLUGIN_ROOT} for practices references
- Update workflow to invoke Skill() instead of Read tool
- Preserve non-negotiable workflow enforcement"
```

---

## Task 12: Update rust-engineer Agent

**Files:**
- Modify: `plugin/agents/rust-engineer.md:10-70`

**Step 1: Update context section**

Replace context section with Skill tool invocations:

```markdown
  <context>
    ## Context

    YOU MUST ALWAYS LOAD IN THIS ORDER:

    1. **Upstream Skills** (universal methodology):
       - Test-Driven Development: Skill("superpowers:test-driven-development")
       - Testing Anti-Patterns: Skill("superpowers:testing-anti-patterns")
       - Code Review Reception: Skill("superpowers:receiving-code-review")

    2. **Local Skills** (organization workflows):
       - TDD Enforcement Algorithm: Skill("cipherpowers:tdd-enforcement-algorithm")
       - Commit Workflow: Skill("cipherpowers:commit-workflow")

    3. **Project Standards** (what quality looks like):
       - Development Standards: @${CLAUDE_PLUGIN_ROOT}practices/development.md
       - Testing Standards: @${CLAUDE_PLUGIN_ROOT}practices/testing.md
       - Rust Error Handling: @${CLAUDE_PLUGIN_ROOT}practices/rust/error-handling.md

    4. **Project Context**:
       - README: @README.md
       - Architecture: @CLAUDE.md
  </context>
```

**Step 2: Update TDD workflow section**

Find and update TDD enforcement section (~line 50):

```markdown
    ### Non-Negotiable: TDD First

    Before ANY implementation code:
    ```
    Skill("cipherpowers:tdd-enforcement-algorithm")
    ```

    This enforces:
    1. Does failing test exist? (binary check)
    2. If NO → Write test first
    3. If YES → Proceed with minimal implementation
    4. Recovery: Delete untested code (no sunk cost exceptions)

    Or use upstream:
    ```
    Skill("superpowers:test-driven-development")
    ```

    For complete RED-GREEN-REFACTOR cycle.
```

**Step 3: Verify agent structure**

Run: `grep "Skill(" plugin/agents/rust-engineer.md | wc -l`
Expected: At least 6 Skill() invocations

**Step 4: Commit rust-engineer agent update**

```bash
git add plugin/agents/rust-engineer.md
git commit -m "refactor: migrate rust-engineer agent to Skill tool

- Update context to use Skill() for upstream skills
- Update context to use Skill() for local skills (TDD, commit)
- Maintain @\${CLAUDE_PLUGIN_ROOT} for practices (dev, testing, Rust)
- Update TDD workflow to invoke tdd-enforcement-algorithm skill
- Preserve TDD discipline enforcement"
```

---

## Task 13: Update technical-writer Agent

**Files:**
- Modify: `plugin/agents/technical-writer.md:10-50`

**Step 1: Update context section**

Replace context section:

```markdown
  <context>
    ## Context

    YOU MUST ALWAYS LOAD IN THIS ORDER:

    1. **Local Skill** (documentation workflow):
       - Maintaining Docs After Changes: Skill("cipherpowers:maintaining-docs-after-changes")

    2. **Project Standards** (what quality looks like):
       - Documentation Standards: @${CLAUDE_PLUGIN_ROOT}practices/documentation.md

    3. **Project Context**:
       - README: @README.md
       - Architecture: @CLAUDE.md
       - All docs in docs/
  </context>
```

**Step 2: Update workflow section**

Replace workflow invocation (~line 40):

```markdown
    ### 1. Invoke Documentation Workflow

    YOU MUST invoke:
    ```
    Skill("cipherpowers:maintaining-docs-after-changes")
    ```

    This two-phase process:
    - Phase 1: Discovery (identify what needs updating)
    - Phase 2: Synchronization (update documentation)

    References practices for documentation standards and structure.
```

**Step 3: Verify references**

Run: `grep -E "(Skill|practices)" plugin/agents/technical-writer.md | wc -l`
Expected: At least 3 references

**Step 4: Commit technical-writer agent update**

```bash
git add plugin/agents/technical-writer.md
git commit -m "refactor: migrate technical-writer agent to Skill tool

- Update context to use Skill() for maintaining-docs-after-changes
- Maintain @\${CLAUDE_PLUGIN_ROOT} for documentation practice
- Update workflow to invoke skill instead of Read tool
- Preserve two-phase sync process enforcement"
```

---

## Task 14: Update retrospective-writer Agent

**Files:**
- Modify: `plugin/agents/retrospective-writer.md:10-50`

**Step 1: Update context section**

Replace context section:

```markdown
  <context>
    ## Context

    YOU MUST ALWAYS LOAD IN THIS ORDER:

    1. **Local Skill** (learning capture workflow):
       - Capturing Learning: Skill("cipherpowers:capturing-learning")

    2. **Project Standards** (what quality looks like):
       - Documentation Standards: @${CLAUDE_PLUGIN_ROOT}practices/documentation.md
       - Workflow Standards: @${CLAUDE_PLUGIN_ROOT}practices/workflow.md

    3. **Project Context**:
       - README: @README.md
       - Architecture: @CLAUDE.md
       - Existing learning: docs/learning/
  </context>
```

**Step 2: Update workflow section**

Replace workflow invocation (~line 40):

```markdown
    ### 1. Invoke Learning Capture Workflow

    YOU MUST invoke:
    ```
    Skill("cipherpowers:capturing-learning")
    ```

    This workflow:
    1. Reviews completed work
    2. Identifies significant insights
    3. Captures decisions and rationale
    4. Documents for future reference

    Saves to: `docs/learning/YYYY-MM-DD-{topic}.md`

    References practices for documentation structure and workflow integration.
```

**Step 3: Verify structure**

Run: `grep "Skill(" plugin/agents/retrospective-writer.md | wc -l`
Expected: At least 1 Skill() invocation

**Step 4: Commit retrospective-writer agent update**

```bash
git add plugin/agents/retrospective-writer.md
git commit -m "refactor: migrate retrospective-writer agent to Skill tool

- Update context to use Skill() for capturing-learning
- Maintain @\${CLAUDE_PLUGIN_ROOT} for practices (docs, workflow)
- Update workflow to invoke skill instead of Read tool
- Preserve learning capture methodology"
```

---

## Task 15: Update ultrathink-debugger Agent

**Files:**
- Modify: `plugin/agents/ultrathink-debugger.md:10-60`

**Step 1: Update context section**

Replace context section:

```markdown
  <context>
    ## Context

    YOU MUST ALWAYS LOAD IN THIS ORDER:

    1. **Upstream Skills** (debugging methodology):
       - Systematic Debugging: Skill("superpowers:systematic-debugging")
       - Root Cause Tracing: Skill("superpowers:root-cause-tracing")
       - Defense in Depth: Skill("superpowers:defense-in-depth")
       - Verification Before Completion: Skill("superpowers:verification-before-completion")

    2. **Project Standards** (what quality looks like):
       - Testing Standards: @${CLAUDE_PLUGIN_ROOT}practices/testing.md
       - Development Standards: @${CLAUDE_PLUGIN_ROOT}practices/development.md

    3. **Project Context**:
       - README: @README.md
       - Architecture: @CLAUDE.md
  </context>
```

**Step 2: Update workflow section**

Replace debugging workflow invocation (~line 45):

```markdown
    ### 1. Follow Systematic Debugging Workflow

    YOU MUST invoke:
    ```
    Skill("superpowers:systematic-debugging")
    ```

    This four-phase framework:
    - Phase 1: Root cause investigation
    - Phase 2: Pattern analysis
    - Phase 3: Hypothesis testing
    - Phase 4: Implementation

    For deep investigation, also use:
    ```
    Skill("superpowers:root-cause-tracing")
    ```

    For validation layers:
    ```
    Skill("superpowers:defense-in-depth")
    ```

    Before claiming success:
    ```
    Skill("superpowers:verification-before-completion")
    ```

    References practices for test commands and verification requirements.
```

**Step 3: Verify multi-skill workflow**

Run: `grep "Skill(" plugin/agents/ultrathink-debugger.md | wc -l`
Expected: At least 4 Skill() invocations

**Step 4: Commit ultrathink-debugger agent update**

```bash
git add plugin/agents/ultrathink-debugger.md
git commit -m "refactor: migrate ultrathink-debugger agent to Skill tool

- Update context to use Skill() for 4 upstream debugging skills
- Maintain @\${CLAUDE_PLUGIN_ROOT} for practices (testing, dev)
- Update workflow to invoke multiple skills in sequence
- Preserve deep investigation methodology"
```

---

## Task 16: Update conducting-code-review Skill

**Files:**
- Modify: `plugin/skills/conducting-code-review/SKILL.md:1-150`

**Step 1: Add Required Skills section**

Insert after frontmatter and before "## Workflow" section:

```markdown
## Required Skills

**REQUIRED BACKGROUND:**
- Requesting Code Review: `Skill("superpowers:requesting-code-review")`
- Code Review Reception: `Skill("superpowers:receiving-code-review")`

**Complementary skills:**
- TDD Enforcement: `Skill("cipherpowers:tdd-enforcement-algorithm")`
- Systematic Debugging: `Skill("superpowers:systematic-debugging")`
- Verification Before Completion: `Skill("superpowers:verification-before-completion")`

## Required Practices

**Project Standards:**
- Code Review: `${CLAUDE_PLUGIN_ROOT}practices/code-review.md`
- Development: `${CLAUDE_PLUGIN_ROOT}practices/development.md`
- Testing: `${CLAUDE_PLUGIN_ROOT}practices/testing.md`
```

**Step 2: Update practice references in workflow**

Find practice references (~lines 50-100) and ensure they use `${CLAUDE_PLUGIN_ROOT}` format:

```markdown
**MUST run these commands (from `${CLAUDE_PLUGIN_ROOT}practices/code-review.md`):**
```

**Step 3: Verify references**

Run: `grep -E "(Skill|CLAUDE_PLUGIN_ROOT)" plugin/skills/conducting-code-review/SKILL.md | wc -l`
Expected: At least 8 references

**Step 4: Commit conducting-code-review skill update**

```bash
git add plugin/skills/conducting-code-review/SKILL.md
git commit -m "docs: add skill/practice references to conducting-code-review

- Add Required Skills section with upstream dependencies
- Add Required Practices section with project standards
- Use Skill() invocation for skills
- Use \${CLAUDE_PLUGIN_ROOT} for practices
- Maintain workflow integrity"
```

---

## Task 17: Update commit-workflow Skill

**Files:**
- Modify: `plugin/skills/commit-workflow/SKILL.md:1-100`

**Step 1: Add Required Practices section**

Insert after frontmatter:

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

**Step 2: Update practice references in workflow**

Find all practice references and ensure `${CLAUDE_PLUGIN_ROOT}` prefix:

```markdown
Follow the algorithmic checks from `${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md`:
```

**Step 3: Verify references**

Run: `grep "CLAUDE_PLUGIN_ROOT" plugin/skills/commit-workflow/SKILL.md | wc -l`
Expected: At least 3 references

**Step 4: Commit commit-workflow skill update**

```bash
git add plugin/skills/commit-workflow/SKILL.md
git commit -m "docs: add practice references to commit-workflow skill

- Add Required Practices section (conventional-commits, git-guidelines, algorithm)
- Add Complementary Skills section
- Ensure all practice references use \${CLAUDE_PLUGIN_ROOT}
- Maintain workflow structure"
```

---

## Task 18: Update maintaining-docs Skill

**Files:**
- Modify: `plugin/skills/documentation/maintaining-docs-after-changes/SKILL.md:1-120`

**Step 1: Add Required Practices section**

Insert after frontmatter:

```markdown
## Required Practices

**Documentation Standards:**
- Documentation: `${CLAUDE_PLUGIN_ROOT}practices/documentation.md`

## Complementary Skills

- Capturing Learning: `Skill("cipherpowers:capturing-learning")`
```

**Step 2: Update practice references**

Find all references to documentation standards and ensure correct format:

```markdown
Follow structure from `${CLAUDE_PLUGIN_ROOT}practices/documentation.md`:
```

**Step 3: Verify references**

Run: `grep "CLAUDE_PLUGIN_ROOT" plugin/skills/documentation/maintaining-docs-after-changes/SKILL.md | wc -l`
Expected: At least 2 references

**Step 4: Commit maintaining-docs skill update**

```bash
git add plugin/skills/documentation/maintaining-docs-after-changes/SKILL.md
git commit -m "docs: add practice references to maintaining-docs skill

- Add Required Practices section (documentation.md)
- Add Complementary Skills section (capturing-learning)
- Ensure practice references use \${CLAUDE_PLUGIN_ROOT}
- Maintain two-phase workflow"
```

---

## Task 19: Update capturing-learning Skill

**Files:**
- Modify: `plugin/skills/documentation/capturing-learning/SKILL.md:1-100`

**Step 1: Add Required Practices section**

Insert after frontmatter:

```markdown
## Required Practices

**Documentation Standards:**
- Documentation: `${CLAUDE_PLUGIN_ROOT}practices/documentation.md`
- Workflow: `${CLAUDE_PLUGIN_ROOT}practices/workflow.md`

## Complementary Skills

- Maintaining Docs: `Skill("cipherpowers:maintaining-docs-after-changes")`
```

**Step 2: Update practice references**

Ensure all references use correct format:

```markdown
Save to location per `${CLAUDE_PLUGIN_ROOT}practices/workflow.md`: `docs/learning/YYYY-MM-DD-{topic}.md`
```

**Step 3: Verify references**

Run: `grep "CLAUDE_PLUGIN_ROOT" plugin/skills/documentation/capturing-learning/SKILL.md | wc -l`
Expected: At least 2 references

**Step 4: Commit capturing-learning skill update**

```bash
git add plugin/skills/documentation/capturing-learning/SKILL.md
git commit -m "docs: add practice references to capturing-learning skill

- Add Required Practices section (documentation.md, workflow.md)
- Add Complementary Skills section (maintaining-docs)
- Ensure practice references use \${CLAUDE_PLUGIN_ROOT}
- Maintain learning capture workflow"
```

---

## Task 20: Update tdd-enforcement-algorithm Skill

**Files:**
- Modify: `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md:1-80`

**Step 1: Add Required Practices section**

Insert after frontmatter:

```markdown
## Required Practices

**Testing Standards:**
- Testing: `${CLAUDE_PLUGIN_ROOT}practices/testing.md`
- Development: `${CLAUDE_PLUGIN_ROOT}practices/development.md`

## Complementary Skills

- Test-Driven Development: `Skill("superpowers:test-driven-development")`
- Testing Anti-Patterns: `Skill("superpowers:testing-anti-patterns")`
```

**Step 2: Update practice references**

Ensure test command references use practices:

```markdown
Run test command from `${CLAUDE_PLUGIN_ROOT}practices/testing.md`:
```

**Step 3: Verify references**

Run: `grep -E "(Skill|CLAUDE_PLUGIN_ROOT)" plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md | wc -l`
Expected: At least 4 references

**Step 4: Commit tdd-enforcement-algorithm skill update**

```bash
git add plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
git commit -m "docs: add skill/practice references to tdd-enforcement-algorithm

- Add Required Practices section (testing.md, development.md)
- Add Complementary Skills section (TDD, testing anti-patterns)
- Use Skill() for upstream skills
- Use \${CLAUDE_PLUGIN_ROOT} for practices
- Maintain algorithmic enforcement"
```

---

## Task 21: Update algorithmic-command-enforcement Skill

**Files:**
- Modify: `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md:80-120`

**Step 1: Add Implementation Examples section**

Insert after existing content, before final section:

```markdown
## Implemented Algorithms (CipherPowers)

### TDD Enforcement Algorithm
```
Skill("cipherpowers:tdd-enforcement-algorithm")
```
Binary checks preventing code before tests.

**Pattern:** Does failing test exist? → Binary decision → No exceptions

### Git Commit Readiness Algorithm
```
${CLAUDE_PLUGIN_ROOT}practices/git-commit-algorithm.md
```
10-step algorithm in practice (project-specific commands).

**Pattern:** Sequential checks → All must pass → Prevents WIP commits

### Code Review Trigger Algorithm
Section 1 of:
```
Skill("cipherpowers:conducting-code-review")
```
Binary commit + review status checks.

**Pattern:** Has commits? AND Needs review? → Binary gates → Invalidates rationalizations

## Related Skills

**Complementary skills:**
- Test-Driven Development: `Skill("superpowers:test-driven-development")`
- Systematic Debugging: `Skill("superpowers:systematic-debugging")`
- Verification Before Completion: `Skill("superpowers:verification-before-completion")`
```

**Step 2: Verify structure**

Run: `grep "Skill(" plugin/skills/meta/algorithmic-command-enforcement/SKILL.md | wc -l`
Expected: At least 4 references

**Step 3: Commit algorithmic-command-enforcement skill update**

```bash
git add plugin/skills/meta/algorithmic-command-enforcement/SKILL.md
git commit -m "docs: add implementation examples to algorithmic-enforcement

- Add CipherPowers implementation examples section
- Reference tdd-enforcement-algorithm skill
- Reference git-commit-algorithm practice
- Reference conducting-code-review skill
- Add Related Skills section"
```

---

## Task 22: Update find-skills Tool

**Files:**
- Modify: `plugin/tools/find-skills:1-20`

**Step 1: Add header explaining dual purpose**

Insert at top of file after shebang:

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
#
# FLAGS:
# --local    Search only CipherPowers skills
# --upstream Search only Superpowers skills
# (default)  Search both repositories
#
# EXAMPLES:
# ./plugin/tools/find-skills "commit"
# ./plugin/tools/find-skills --local "review"
# ./plugin/tools/find-skills --upstream "brainstorming"
```

**Step 2: Verify tool still works**

Run: `./plugin/tools/find-skills "commit" | head -5`
Expected: Tool executes, finds commit-workflow skill

**Step 3: Commit find-skills update**

```bash
git add plugin/tools/find-skills
git commit -m "docs: add header to find-skills explaining dual discovery

- Add purpose statement (exploration vs execution)
- Document Skill tool invocation pattern for execution
- Document find-skills pattern for discovery
- Add flag documentation and examples
- Clarify when to use each approach"
```

---

## Task 23: Update skill-template

**Files:**
- Modify: `plugin/templates/skill-template.md:1-80`

**Step 1: Update template with Skill tool sections**

Replace entire file:

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

## Quick Reference

[Table or list showing common patterns]

## Workflow

### Step 1: [Action]

[Detailed instructions]

**Commands:**
```bash
[exact commands from practices]
```

**Expected output:**
```
[what success looks like]
```

### Step 2: [Action]

[Continue pattern]

## Examples

### Example 1: [Scenario]

[Complete walkthrough]

## Testing

[How to verify this workflow works]

## Related Documentation

- [Related doc]: [path or URL]
```

**Step 2: Verify template structure**

Run: `grep "##" plugin/templates/skill-template.md | wc -l`
Expected: At least 10 sections

**Step 3: Commit skill-template update**

```bash
git add plugin/templates/skill-template.md
git commit -m "docs: update skill-template with Skill tool pattern

- Add Invocation section showing Skill() usage
- Add Required Skills section structure
- Add Required Practices section structure
- Distinguish skills (Skill()) from practices (\${CLAUDE_PLUGIN_ROOT})
- Maintain workflow structure and examples"
```

---

## Task 24: Update agent-template

**Files:**
- Modify: `plugin/templates/agent-template.md:1-100`

**Step 1: Update context section pattern**

Find context section and replace with:

```markdown
  <context>
    ## Context

    YOU MUST ALWAYS LOAD IN THIS ORDER:

    1. **Upstream Skills** (universal methodology):
       - Skill Name: Skill("superpowers:skill-name")
       - Skill Name: Skill("superpowers:skill-name")

    2. **Local Skills** (organization workflows):
       - Skill Name: Skill("cipherpowers:skill-name")
       - Skill Name: Skill("cipherpowers:skill-name")

    3. **Project Standards** (what quality looks like):
       - Practice Name: @${CLAUDE_PLUGIN_ROOT}practices/practice-name.md
       - Practice Name: @${CLAUDE_PLUGIN_ROOT}practices/practice-name.md

    4. **Project Context**:
       - README: @README.md
       - Architecture: @CLAUDE.md
  </context>
```

**Step 2: Update workflow section pattern**

Find workflow section and update with Skill invocation:

```markdown
    ### 2. Follow [Skill Name] Workflow

    YOU MUST invoke and follow:
    ```
    Skill("cipherpowers:skill-name")
    ```

    This skill defines:
    - Step 1: [What it does]
    - Step 2: [What it does]
    - Step 3: [What it does]

    References practices for project-specific configuration.

    **The skill defines HOW. You enforce that it gets done.**
```

**Step 3: Verify template shows three reference types**

Run: `grep -E "(Skill|@\\$\{CLAUDE_PLUGIN_ROOT\}|@README)" plugin/templates/agent-template.md | wc -l`
Expected: At least 8 references showing all three types

**Step 4: Commit agent-template update**

```bash
git add plugin/templates/agent-template.md
git commit -m "docs: update agent-template with Skill tool pattern

- Update context section with Skill() invocations
- Distinguish skills, practices, and project files
- Show namespace usage (superpowers:*, cipherpowers:*)
- Update workflow section with Skill invocation
- Preserve persuasion principles structure"
```

---

## Task 25: Update CLAUDE.md Architecture Documentation

**Files:**
- Modify: `CLAUDE.md:203-260`

**Step 1: Update Integration with Superpowers section**

Replace lines 203-218 with:

```markdown
## Integration with Superpowers

**Skill Registration:**
- CipherPowers skills registered via `.claude-plugin/skills.json`
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

**Step 2: Update Skills Layer section**

Replace lines 15-28 with:

```markdown
### 1. Skills Layer (`plugin/skills/`)

Reusable process knowledge documented using the superpowers framework. Skills are testable, discoverable guides for techniques and workflows.

**Registration:**
- All skills registered in `.claude-plugin/skills.json`
- Namespace: `cipherpowers:*`
- Discoverable via Skill tool's available_skills

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

**Step 3: Update Discovery section**

Replace lines 220-252 with:

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

**Step 4: Verify documentation**

Run: `grep -c "Skill(" CLAUDE.md`
Expected: At least 10 references

**Step 5: Commit CLAUDE.md update**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with Skill tool architecture

- Update Integration section with skill registration
- Add invocation patterns table (skills, practices, files)
- Update Skills Layer section with registration details
- Update Discovery section with Skill tool and find-tools
- Clarify skill vs practice distinction throughout"
```

---

## Task 26: Update README.md

**Files:**
- Modify: `README.md:30-120`

**Step 1: Add Using CipherPowers section**

Insert after installation section:

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

**Step 2: Verify structure**

Run: `grep "###" README.md | head -10`
Expected: Section headers for Skills, Practices, Slash Commands

**Step 3: Commit README.md update**

```bash
git add README.md
git commit -m "docs: add Using CipherPowers section to README

- Add Skills section listing 8 skills with namespaces
- Add Practices section listing 10 practices
- Add Slash Commands section listing 7 commands
- Include invocation patterns for each type
- Include discovery tool usage examples"
```

---

## Task 27: Create Migration Verification Test

**Files:**
- Create: `plugin/tools/verify-migration`

**Step 1: Create verification script**

```bash
#!/usr/bin/env bash
# Migration verification script
# Tests that all skills, practices, and references are correct

set -e

echo "=== CipherPowers Migration Verification ==="
echo ""

# Test 1: Skills manifest exists
echo "✓ Checking skills manifest..."
if [[ ! -f .claude-plugin/skills.json ]]; then
  echo "✗ FAIL: .claude-plugin/skills.json not found"
  exit 1
fi

# Test 2: Manifest has 8 skills
echo "✓ Checking skill count..."
SKILL_COUNT=$(jq '.skills | length' .claude-plugin/skills.json)
if [[ "$SKILL_COUNT" != "8" ]]; then
  echo "✗ FAIL: Expected 8 skills, found $SKILL_COUNT"
  exit 1
fi

# Test 3: All skills use cipherpowers namespace
echo "✓ Checking namespaces..."
NON_CP=$(jq -r '.skills[].name' .claude-plugin/skills.json | grep -v "^cipherpowers:" || true)
if [[ -n "$NON_CP" ]]; then
  echo "✗ FAIL: Found non-cipherpowers namespaces: $NON_CP"
  exit 1
fi

# Test 4: All skill paths exist
echo "✓ Checking skill file paths..."
for path in $(jq -r '.skills[].path' .claude-plugin/skills.json); do
  if [[ ! -f "$path" ]]; then
    echo "✗ FAIL: Skill file not found: $path"
    exit 1
  fi
done

# Test 5: Agents use Skill() invocations
echo "✓ Checking agent Skill() usage..."
AGENT_SKILL_COUNT=$(grep -r "Skill(" plugin/agents/*.md | wc -l | tr -d ' ')
# Expected: 5 agents × ~3 skills each (context + workflow sections) = ~15
# Minimum threshold: 10 (allows some variation)
# Breakdown: code-reviewer(3), rust-engineer(5), technical-writer(1), retrospective-writer(1), ultrathink-debugger(4)
if [[ "$AGENT_SKILL_COUNT" -lt 10 ]]; then
  echo "✗ FAIL: Expected at least 10 Skill() invocations in agents, found $AGENT_SKILL_COUNT"
  exit 1
fi

# Test 6: Agents reference practices correctly
echo "✓ Checking agent practice references..."
AGENT_PRACTICE_COUNT=$(grep -r "@\${CLAUDE_PLUGIN_ROOT}" plugin/agents/*.md | wc -l | tr -d ' ')
# Expected: 5 agents × ~2-3 practices each = ~10-15
# Minimum threshold: 10
# Each agent needs: practices context section + workflow references
if [[ "$AGENT_PRACTICE_COUNT" -lt 10 ]]; then
  echo "✗ FAIL: Expected at least 10 practice references in agents, found $AGENT_PRACTICE_COUNT"
  exit 1
fi

# Test 7: Skills reference practices correctly
echo "✓ Checking skill practice references..."
SKILL_PRACTICE_COUNT=$(grep -r "\${CLAUDE_PLUGIN_ROOT}" plugin/skills/**/*.md | wc -l | tr -d ' ')
# Expected: 8 skills, but only 6 have Required Practices sections
# conducting-code-review(3), commit-workflow(3), maintaining-docs(1), capturing-learning(2),
# tdd-enforcement(2), algorithmic-enforcement(1) = ~12
# Minimum threshold: 10 (allows some variation)
if [[ "$SKILL_PRACTICE_COUNT" -lt 10 ]]; then
  echo "✗ FAIL: Expected at least 10 practice references in skills, found $SKILL_PRACTICE_COUNT"
  exit 1
fi

# Test 8: Commands updated
echo "✓ Checking command Skill() usage..."
CMD_SKILL_COUNT=$(grep -r "Skill(" plugin/commands/*.md | wc -l | tr -d ' ')
# Expected: 7 commands × ~2 references each (option sections + workflow components) = ~14
# Minimum threshold: 8 (at least 1 per command)
if [[ "$CMD_SKILL_COUNT" -lt 8 ]]; then
  echo "✗ FAIL: Expected at least 8 Skill() references in commands, found $CMD_SKILL_COUNT"
  exit 1
fi

# Test 9: find-skills tool has header
echo "✓ Checking find-skills header..."
if ! grep -q "INVOCATION PATTERNS" plugin/tools/find-skills; then
  echo "✗ FAIL: find-skills missing updated header"
  exit 1
fi

# Test 10: CLAUDE.md updated
echo "✓ Checking CLAUDE.md documentation..."
if ! grep -q "Skill(\"cipherpowers:" CLAUDE.md; then
  echo "✗ FAIL: CLAUDE.md missing Skill invocation examples"
  exit 1
fi

# Test 11: README.md updated
echo "✓ Checking README.md documentation..."
if ! grep -q "## Using CipherPowers" README.md; then
  echo "✗ FAIL: README.md missing Using CipherPowers section"
  exit 1
fi

# Test 12: Backward compatibility - find-skills still works
echo "✓ Checking backward compatibility..."
if ! ./plugin/tools/find-skills "commit" | grep -q "commit-workflow"; then
  echo "✗ FAIL: find-skills backward compatibility broken"
  exit 1
fi

# Test 13: Backward compatibility - skills can be read manually
echo "✓ Checking manual skill file access..."
if [[ ! -f "plugin/skills/commit-workflow/SKILL.md" ]]; then
  echo "✗ FAIL: Skills not accessible via file path"
  exit 1
fi

# Test 14: Backward compatibility - practices still work
echo "✓ Checking practices backward compatibility..."
if ! ./plugin/tools/find-practices "review" | grep -q "code-review"; then
  echo "✗ FAIL: find-practices backward compatibility broken"
  exit 1
fi

echo ""
echo "=== ✓ All verification tests passed! ==="
echo ""
echo "Migration complete. Skills registered, agents updated, documentation synced."
echo "Backward compatibility maintained: find-skills, find-practices, manual file access all work."
```

**Step 2: Make executable**

Run: `chmod +x plugin/tools/verify-migration`
Expected: Script is executable

**Step 3: Commit verification script**

```bash
git add plugin/tools/verify-migration
git commit -m "test: add migration verification script

- Check skills manifest exists with 8 skills
- Verify all namespaces are cipherpowers:*
- Verify all skill paths exist
- Check agents use Skill() invocations
- Check agents reference practices correctly
- Check skills reference practices correctly
- Check commands updated with Skill()
- Check find-skills header updated
- Check CLAUDE.md documentation
- Check README.md documentation

Provides automated verification of complete migration."
```

---

## Task 28: Run Migration Verification

**Files:**
- Test: All migration changes

**Step 1: Run verification script**

Run: `./plugin/tools/verify-migration`
Expected: All checks pass with "✓ All verification tests passed!"

If failures occur:
- Review failure messages
- Fix identified issues
- Re-run verification

**Step 2: Manual spot checks**

Check random files:
```bash
# Check an agent
cat plugin/agents/code-reviewer.md | head -50

# Check a skill
cat plugin/skills/commit-workflow/SKILL.md | head -40

# Check a command
cat plugin/commands/commit.md

# Check documentation
grep "Skill(" CLAUDE.md | head -5
```

Expected: All show correct patterns (Skill() for skills, ${CLAUDE_PLUGIN_ROOT} for practices)

**Step 3: Test find-skills tool**

Run: `./plugin/tools/find-skills "commit"`
Expected: Finds commit-workflow skill with helpful header

Run: `./plugin/tools/find-skills --local "review"`
Expected: Finds conducting-code-review skill

**Step 4: Test find-practices tool**

Run: `./plugin/tools/find-practices "commit"`
Expected: Finds conventional-commits and git-commit-algorithm

**Step 5: Commit verification results**

```bash
git add .
git commit -m "test: verify migration completion

Ran migration verification script:
- ✓ Skills manifest with 8 skills
- ✓ All namespaces cipherpowers:*
- ✓ All skill paths exist
- ✓ Agents use Skill() invocations
- ✓ Practice references correct
- ✓ Commands updated
- ✓ Documentation synced

Manual spot checks:
- ✓ Agents show correct pattern
- ✓ Skills show correct pattern
- ✓ Commands show correct pattern
- ✓ find-skills works
- ✓ find-practices works

Migration verified complete."
```

---

## Task 29: Final Integration Test

**Files:**
- Create: `plugin/tools/integration-test`

**Step 1: Create integration test script**

```bash
#!/usr/bin/env bash
# Integration test script for migration verification
# Tests end-to-end workflows

set -e

echo "=== CipherPowers Integration Tests ==="
echo ""

# Test 1: Skill manifest registration
echo "✓ Testing skill manifest registration..."
MANIFEST_SKILLS=$(jq -r '.skills[].name' .claude-plugin/skills.json)
EXPECTED_SKILLS=(
  "cipherpowers:using-skills"
  "cipherpowers:selecting-agents"
  "cipherpowers:commit-workflow"
  "cipherpowers:conducting-code-review"
  "cipherpowers:algorithmic-command-enforcement"
  "cipherpowers:tdd-enforcement-algorithm"
  "cipherpowers:maintaining-docs-after-changes"
  "cipherpowers:capturing-learning"
)

for skill in "${EXPECTED_SKILLS[@]}"; do
  if ! echo "$MANIFEST_SKILLS" | grep -q "$skill"; then
    echo "✗ FAIL: Skill not found in manifest: $skill"
    exit 1
  fi
done

# Test 2: Agent references are correct
echo "✓ Testing agent skill references..."
for agent in plugin/agents/*.md; do
  agent_name=$(basename "$agent" .md)

  # Check for Skill() invocations
  if ! grep -q "Skill(" "$agent"; then
    echo "✗ FAIL: Agent $agent_name has no Skill() invocations"
    exit 1
  fi

  # Check for practice references
  if ! grep -q "@\${CLAUDE_PLUGIN_ROOT}" "$agent"; then
    echo "✗ FAIL: Agent $agent_name has no practice references"
    exit 1
  fi
done

# Test 3: Skill cross-references are correct
echo "✓ Testing skill cross-references..."
for skill in plugin/skills/commit-workflow/SKILL.md \
            plugin/skills/conducting-code-review/SKILL.md \
            plugin/skills/documentation/maintaining-docs-after-changes/SKILL.md \
            plugin/skills/documentation/capturing-learning/SKILL.md \
            plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md; do

  if [[ ! -f "$skill" ]]; then
    echo "✗ FAIL: Skill file not found: $skill"
    exit 1
  fi

  # Should have Required Practices section
  if ! grep -q "## Required Practices" "$skill"; then
    echo "✗ FAIL: Skill missing Required Practices section: $skill"
    exit 1
  fi
done

# Test 4: Commands reference skills correctly
echo "✓ Testing command skill references..."
for cmd in plugin/commands/*.md; do
  cmd_name=$(basename "$cmd" .md)

  # Most commands should reference Skill() invocation
  if [[ "$cmd_name" != "tdd" ]] && [[ "$cmd_name" != "refactor" ]] && [[ "$cmd_name" != "pr-draft" ]] && [[ "$cmd_name" != "pr-review" ]]; then
    if ! grep -q "Skill(" "$cmd"; then
      echo "✗ FAIL: Command $cmd_name missing Skill() reference"
      exit 1
    fi
  fi
done

# Test 5: Practices are discoverable
echo "✓ Testing practice discovery..."
PRACTICE_COUNT=$(./plugin/tools/find-practices "" | grep -c "^Use " || true)
if [[ "$PRACTICE_COUNT" -lt 10 ]]; then
  echo "✗ FAIL: Expected at least 10 practices, found $PRACTICE_COUNT"
  exit 1
fi

# Test 6: Skills are discoverable
echo "✓ Testing skill discovery..."
LOCAL_SKILL_COUNT=$(./plugin/tools/find-skills --local "" | grep -c "cipherpowers:" || true)
if [[ "$LOCAL_SKILL_COUNT" -lt 8 ]]; then
  echo "✗ FAIL: Expected at least 8 local skills, found $LOCAL_SKILL_COUNT"
  exit 1
fi

# Test 7: Backward compatibility - find-skills still works
echo "✓ Testing backward compatibility..."
if ! ./plugin/tools/find-skills "commit" | grep -q "commit-workflow"; then
  echo "✗ FAIL: find-skills backward compatibility broken"
  exit 1
fi

# Test 8: Variable resolution in skill files
echo "✓ Testing variable resolution patterns..."
for skill in plugin/skills/**/*.md; do
  # If skill references practices, should use ${CLAUDE_PLUGIN_ROOT}
  if grep -q "practices/" "$skill" 2>/dev/null; then
    if ! grep -q "\${CLAUDE_PLUGIN_ROOT}" "$skill"; then
      echo "⚠ WARNING: Skill $skill references practices without \${CLAUDE_PLUGIN_ROOT}"
    fi
  fi
done

# Test 9: Documentation consistency
echo "✓ Testing documentation consistency..."
if ! grep -q "Skill(\"cipherpowers:" CLAUDE.md; then
  echo "✗ FAIL: CLAUDE.md missing Skill invocation examples"
  exit 1
fi

if ! grep -q "## Using CipherPowers" README.md; then
  echo "✗ FAIL: README.md missing Using CipherPowers section"
  exit 1
fi

# Test 10: Template consistency
echo "✓ Testing template updates..."
if ! grep -q "Skill(\"cipherpowers:" plugin/templates/skill-template.md; then
  echo "✗ FAIL: skill-template.md not updated with Skill() pattern"
  exit 1
fi

if ! grep -q "Skill(\"superpowers:" plugin/templates/agent-template.md; then
  echo "✗ FAIL: agent-template.md not updated with Skill() pattern"
  exit 1
fi

echo ""
echo "=== ✓ All integration tests passed! ==="
echo ""
echo "Verified:"
echo "  - Skill manifest registration (8 skills)"
echo "  - Agent skill/practice references (5 agents)"
echo "  - Skill cross-references (6 skills)"
echo "  - Command skill references (7 commands)"
echo "  - Practice discovery (10+ practices)"
echo "  - Skill discovery (8+ skills)"
echo "  - Backward compatibility (find-skills)"
echo "  - Variable resolution patterns"
echo "  - Documentation consistency"
echo "  - Template updates"
```

**Step 2: Make executable**

Run: `chmod +x plugin/tools/integration-test`

**Step 3: Run integration tests**

Run: `./plugin/tools/integration-test`
Expected: All tests pass

**Step 4: Commit integration test script**

```bash
git add plugin/tools/integration-test
git commit -m "test: add executable integration test suite

Tests end-to-end workflows:
- ✓ Skill manifest registration (8 skills)
- ✓ Agent skill/practice references (5 agents)
- ✓ Skill cross-references (6 skills with Required Practices)
- ✓ Command skill references (7 commands)
- ✓ Practice discovery (10+ practices via find-practices)
- ✓ Skill discovery (8+ skills via find-skills)
- ✓ Backward compatibility (find-skills still works)
- ✓ Variable resolution patterns in skills
- ✓ Documentation consistency (CLAUDE.md, README.md)
- ✓ Template updates (skill-template, agent-template)

Provides automated verification of complete migration."
```

---

## Task 30: Update Migration Plan Status

**Files:**
- Modify: `docs/migration-plan-official-skills.md:1-50`

**Step 1: Update migration plan with completion status**

Add at top of file after header:

```markdown
**Status:** ✅ COMPLETED (2025-10-18)

**Completion Summary:**
- ✅ Phase 1: Plugin manifest created with 8 skills registered
- ✅ Phase 2: Meta-skills updated (using-skills, selecting-agents)
- ✅ Phase 3: All 7 commands updated with dual invocation
- ✅ Phase 4: All 5 agents migrated to Skill tool
- ✅ Phase 5: All 6 skills updated with cross-references
- ✅ Phase 6: Templates updated (skill, agent, find-skills)
- ✅ Phase 7: Documentation updated (CLAUDE.md, README.md)
- ✅ Phase 8: Verification tests created and passed

**Migration Results:**
- 8 skills registered with cipherpowers:* namespace
- 5 agents migrated to Skill() invocation
- 7 commands document dual invocation (slash + Skill)
- 10 practices remain as project-specific standards
- 2 discovery tools enhanced (find-skills, find-practices)
- 3 templates updated with new patterns
- 100% backward compatibility maintained
- All verification tests passing
```

**Step 2: Commit migration plan update**

```bash
git add docs/migration-plan-official-skills.md
git commit -m "docs: mark migration plan as completed

Migration to official Skill tool mechanism complete:
- All 8 phases executed successfully
- All verification tests passing
- End-to-end workflows tested
- Documentation synced
- Backward compatibility maintained

Next: Monitor adoption and consider upstream contributions."
```

---

## Execution Complete

All 30 tasks completed! The migration is ready for:

**Option 1: Review & Deploy**
- Review all commits
- Test in staging environment
- Deploy to production

**Option 2: Iterative Testing**
- Test each phase incrementally
- Verify before proceeding to next phase
- Rollback point after each commit

**Verification:**
```bash
./plugin/tools/verify-migration
```

**Success Criteria:**
✅ All 8 skills registered
✅ All 5 agents use Skill tool
✅ All 7 commands updated
✅ All practices still work
✅ All documentation synced
✅ Backward compatibility maintained
✅ Verification tests pass
