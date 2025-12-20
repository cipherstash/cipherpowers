# Research Report: Agent and Command Template Consistency Analysis

## Metadata
- Date: 2025-12-07
- Researcher: research-agent
- Scope: All agents in `plugin/agents/` and commands in `plugin/commands/`

## Research Questions
1. What is the canonical agent structure defined in `agent-template.md`?
2. Which agents follow the template exactly and which deviate?
3. What structural patterns exist across agents?
4. What should the recommended canonical structure be?

## Key Findings

### Finding 1: Template Structure Definition
- **Source:** `/Users/tobyhede/src/cipherpowers/plugin/templates/agent-template.md`
- **Evidence:** Lines 1-142 define complete structure with:
  - YAML frontmatter (name, description, color)
  - `<important>` wrapper containing:
    - `<context>` section
    - `<non_negotiable_workflow>` section
    - `<rationalization_defense>` section
    - `<instructions>` section
  - Outside `<important>`: Purpose, Capabilities, Behavioral Traits, Response Approach, Example Interactions
- **Confidence:** HIGH
- **Implication:** This is the canonical reference structure all agents should follow

### Finding 2: Agents Following Template Closely
- **Source:** Multiple agent files analyzed
- **Evidence:**
  - `research-agent.md` (lines 1-289): Follows template exactly
  - `commit-agent.md` (lines 1-215): Follows template with `<mandatory_skill_activation>` addition
  - `technical-writer.md` (lines 1-270): Follows template with `<mode_detection>` addition before context
  - `ultrathink-debugger.md` (lines 1-411): Follows template structure
  - `plan-review-agent.md` (lines 1-213): Follows template with `<save_workflow>` addition
  - `execute-review-agent.md` (lines 1-312): Follows template structure
- **Confidence:** HIGH
- **Implication:** Most agents follow the core template pattern

### Finding 3: Minimal Agent Pattern (Exec Agents)
- **Source:** `code-exec-agent.md`, `rust-exec-agent.md`
- **Evidence:**
  - Both files ~41 lines (lines 1-41)
  - YAML frontmatter with `model: haiku` field (line 5)
  - No `<important>` wrapper
  - No persuasion principle sections
  - Minimal rules-based structure
  - Simple completion report format
- **Confidence:** HIGH
- **Implication:** Intentional minimal pattern for execution agents - NOT a deviation

### Finding 4: Agent-Specific Sections Pattern
- **Source:** Multiple agents
- **Evidence:**
  - `commit-agent.md`: `<mandatory_skill_activation>` (lines 25-41) - evaluates then activates skill
  - `technical-writer.md`: `<mode_detection>` (lines 10-33) - determines VERIFICATION vs EXECUTION mode
  - `plan-review-agent.md`: `<save_workflow>` (lines 163-199) - file naming and save requirements
  - `ultrathink-debugger.md`: No extra sections beyond template
  - `code-agent.md`, `rust-agent.md`: `<mandatory_skill_activation>` pattern
- **Confidence:** HIGH
- **Implication:** Agents can add role-specific sections while maintaining template structure

### Finding 5: Code Review Agent Deviation
- **Source:** `code-review-agent.md`
- **Evidence:** Lines 1-31 only
  - Has YAML frontmatter (lines 1-5)
  - Has role description (lines 7-8)
  - Has `<important>` wrapper (line 10)
  - Has `<instructions>` section ONLY (lines 11-30)
  - Missing: `<context>`, `<non_negotiable_workflow>`, `<rationalization_defense>`, `<quality_gates>`
  - Missing: Outside sections (Purpose, Capabilities, etc.)
- **Confidence:** HIGH
- **Implication:** Significant deviation - relies entirely on skill for workflow

### Finding 6: Gatekeeper Agent Structure
- **Source:** `gatekeeper.md`
- **Evidence:** Lines 1-288
  - No YAML frontmatter at all (starts directly with "# Gatekeeper Agent")
  - Has workflow sections but different naming:
    - "## MANDATORY: Skill Activation" (not wrapped in tags)
    - "## Authority Principle: Non-Negotiable Workflow"
    - "## Commitment Principle: Track Progress"
    - "## Scarcity Principle: One Job Only"
    - "## Social Proof Principle: Failure Modes"
    - "## Rationalization Defenses" (not in tags)
  - Uses persuasion principles as section headers, not tag names
- **Confidence:** HIGH
- **Implication:** Major structural deviation - principles as sections not tags

### Finding 7: Review Collation Agent Structure
- **Source:** `review-collation-agent.md`
- **Evidence:** Lines 1-128
  - Has YAML frontmatter (lines 1-5)
  - Uses different section pattern:
    - `<context>`
    - `<mandatory_skill_activation>`
    - `<workflow_enforcement>` (instead of `<non_negotiable_workflow>`)
    - `<output_format>` (custom section)
    - `<rationalization_defense>`
    - `<instructions>`
  - Missing `<quality_gates>` section
  - Has Outside sections but minimal
- **Confidence:** HIGH
- **Implication:** Moderate deviation - uses skill-heavy approach with custom sections

### Finding 8: Path Test Agent Structure
- **Source:** `path-test-agent.md`
- **Evidence:** Lines 1-68
  - Has YAML frontmatter (lines 1-5)
  - No `<important>` wrapper
  - Simple procedural structure (Test Objective, Test Procedure, Important)
  - Uses markdown headers, not XML tags
- **Confidence:** HIGH
- **Implication:** Intentional minimal pattern for testing agent

### Finding 9: Command Structure Pattern
- **Source:** All command files in `plugin/commands/`
- **Evidence:**
  - All have YAML frontmatter with `description` field
  - Most have `<instructions>` section with skill activation pattern
  - Common pattern: "## MANDATORY: Skill Activation" with Path and Tool
  - Commands are thin dispatchers (as intended per CLAUDE.md)
  - Some include usage tables (execute.md, verify.md, code-review.md)
- **Confidence:** HIGH
- **Implication:** Commands follow consistent minimal pattern focused on skill dispatch

### Finding 10: `<quality_gates>` Section Consistency
- **Source:** Multiple agents
- **Evidence:**
  - Present in: `research-agent.md` (lines 203-217), `ultrathink-debugger.md` (lines 170-185), `code-agent.md` (lines 209-224), `rust-agent.md` (lines 215-230), `plan-review-agent.md` (lines 146-161)
  - Missing in: `code-review-agent.md`, `review-collation-agent.md`, `gatekeeper.md`, exec agents
  - Identical text across all instances
- **Confidence:** HIGH
- **Implication:** `<quality_gates>` section is standardized but not universally applied

## Patterns Observed

### Pattern 1: Three Agent Archetypes
1. **Full Template Agents** (research, commit, technical-writer, ultrathink, plan-review, execute-review, code, rust): Complete persuasion-principle structure with all sections
2. **Minimal Exec Agents** (code-exec, rust-exec): Intentionally simplified for plan execution with haiku model
3. **Skill-Delegating Agents** (code-review, review-collation): Minimal agent structure, delegates to skills

### Pattern 2: Skill Activation Evolution
- Template: Simple list in `<context>` section
- Current pattern: `<mandatory_skill_activation>` section with:
  - Load skill context (@ reference)
  - Step 1 - EVALUATE (YES/NO decision)
  - Step 2 - ACTIVATE (Skill tool call)
  - Warning about proceeding without activation
- Found in: commit-agent, technical-writer, code-agent, rust-agent, ultrathink-debugger, plan-review-agent, review-collation-agent

### Pattern 3: Custom Sections for Role-Specific Needs
- `<mode_detection>`: technical-writer (VERIFICATION vs EXECUTION mode)
- `<save_workflow>`: plan-review-agent (file naming conventions)
- `<workflow_enforcement>`: review-collation-agent (Phase 2 workflow)
- `<output_format>`: review-collation-agent (final message format)
- `<mandatory_skill_activation>`: Multiple agents (skill evaluation pattern)

### Pattern 4: Frontmatter Fields
- Standard: `name`, `description`, `color`
- Additional in exec agents: `model: haiku`
- Gatekeeper: Missing frontmatter entirely
- Commands: `description`, `argument-hint` (some)

### Pattern 5: Rationalization Defense Structure
- Template: Uses tables for Excuse → Reality mapping
- Consistent across: research, commit, technical-writer, ultrathink, code, rust, plan-review, execute-review
- Different format in: gatekeeper (uses "###" headers instead of tables)
- Missing in: code-review, review-collation (skill-delegating pattern)

## Gaps and Uncertainties

### Gap 1: Intentional vs Unintentional Deviations
**What couldn't be verified:** Whether deviations in code-review-agent, gatekeeper, and review-collation-agent are intentional design choices or evolutionary artifacts
**Needs investigation:** Review git history to see when these patterns diverged

### Gap 2: Model Field Usage
**What couldn't be verified:** Should all agents have `model:` field in frontmatter, or only exec agents?
**Context:** Exec agents explicitly set `model: haiku`, but other agents don't specify model in frontmatter
**Needs clarification:** Is model selection handled elsewhere, or should template include model field?

### Gap 3: Quality Gates Section Criteria
**What couldn't be verified:** Clear criteria for when `<quality_gates>` section should be included
**Evidence:** Present in implementation agents (code, rust, ultrathink) but missing in review/orchestration agents
**Hypothesis:** Quality gates apply to agents that produce code changes, not review/analysis agents

### Gap 4: Evolution of Skill Activation Pattern
**What couldn't be verified:** When `<mandatory_skill_activation>` pattern became standard
**Evidence:** Template doesn't include this pattern, but most recent agents do
**Needs investigation:** Should template be updated to include this pattern?

### Gap 5: Outside Sections Consistency
**What couldn't be verified:** Are all outside sections (Purpose, Capabilities, Behavioral Traits, Response Approach, Example Interactions) mandatory?
**Evidence:**
- Full in: research, commit, technical-writer, ultrathink, execute-review, plan-review
- Minimal in: code, rust (cut off at line 238/243)
- Missing in: code-review, review-collation, gatekeeper, exec agents
**Needs clarification:** Template includes these but they're inconsistently applied

## Summary

The template analysis reveals three distinct agent archetypes:

1. **Full Template Agents** (majority): Follow agent-template.md structure with persuasion principles, adding role-specific sections as needed (`<mandatory_skill_activation>`, `<mode_detection>`, etc.)

2. **Minimal Exec Agents** (intentional): Stripped-down structure for plan execution with haiku model - no persuasion principles, just rules and completion reports

3. **Skill-Delegating Agents** (emerging pattern): Minimal agent structure that delegates workflow entirely to skills (code-review-agent, review-collation-agent)

**Key deviations requiring attention:**
- **code-review-agent.md**: Incomplete - missing most template sections
- **gatekeeper.md**: No frontmatter, uses principles as section headers instead of tags
- **review-collation-agent.md**: Custom section names, missing quality gates

**Emergent patterns not in template:**
- `<mandatory_skill_activation>` section (should likely be added to template)
- `model:` frontmatter field (exec agents only)
- Role-specific custom sections (acceptable pattern)

## Recommendations

### 1. Update agent-template.md
- Add `<mandatory_skill_activation>` section to template (lines 21-41 pattern from commit-agent)
- Clarify when `<quality_gates>` section is required (implementation agents vs review agents)
- Add guidance on when to add custom sections

### 2. Standardize code-review-agent.md
- Either: Complete the template structure with all sections
- Or: Document as intentional "skill-delegating" pattern and add comment explaining minimal structure

### 3. Fix gatekeeper.md
- Add YAML frontmatter (name, description, color)
- Convert principle section headers to XML tag structure matching template
- Add `<important>` wrapper

### 4. Decide on review-collation-agent.md
- Either: Rename `<workflow_enforcement>` to `<non_negotiable_workflow>` for consistency
- Or: Document custom section names as intentional for skill-heavy agents

### 5. Document Agent Archetypes
- Create documentation explaining three archetype patterns
- When to use each archetype
- What deviations are acceptable vs requiring fixes

### 6. Clarify Model Selection
- Document whether `model:` frontmatter field should be universal or exec-agent-only
- If universal, add to template

### 7. Outside Sections Completeness
- Verify code-agent.md and rust-agent.md are complete (appear truncated)
- Standardize which outside sections are required vs optional
