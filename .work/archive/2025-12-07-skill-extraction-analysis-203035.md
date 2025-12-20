# Skill Extraction Analysis

## Metadata
- **Date:** 2025-12-07
- **Researcher:** research-agent
- **Scope:** Analyze all cipherpowers agents to identify duplicated workflow logic that should be extracted into reusable skills

## Research Questions
1. What workflow logic exists in agents that could be skills?
2. Is there duplication between agents?
3. Which agents properly delegate vs embed workflow?

## Executive Summary

**Key Findings:**
- **2 agents properly delegate** (code-review-agent, review-collation-agent) - minimal files, reference skills
- **7 agents partially delegate** - reference some skills but embed other workflows
- **Major duplication found:** code-agent and rust-agent are 95% identical (~230 lines each)
- **4 high-priority skill extraction candidates** identified
- **~500 lines of duplicated boilerplate** across agents (announcement, pre-work, quality gates, etc.)

---

## Agents Inventory

### Minimal Agents (Properly Delegate)

| Agent | Size | Skills Referenced | Workflow Location |
|-------|------|-------------------|-------------------|
| **code-review-agent.md** | 31 lines | conducting-code-review | In skill ✓ |
| **review-collation-agent.md** | 128 lines | dual-verification | In skill ✓ |

**Evidence:**
- **Source:** `/Users/tobyhede/src/cipherpowers/plugin/agents/code-review-agent.md` (lines 1-31)
- **Confidence:** HIGH
- **Implication:** These demonstrate the ideal agent structure - thin delegation layer only

### Full Template Agents (Mix of Delegation and Embedding)

| Agent | Size | Skills Referenced | Embedded Workflows |
|-------|------|-------------------|--------------------|
| **research-agent.md** | 289 lines | systematic-debugging | Multi-angle exploration, evidence gathering, gap identification |
| **commit-agent.md** | 215 lines | commit-workflow | None (delegates properly) |
| **technical-writer.md** | 271 lines | maintaining-docs-after-changes | Mode detection pattern |
| **ultrathink-debugger.md** | 411 lines | systematic-debugging, root-cause-tracing, defense-in-depth | Complex debugging techniques (~150 lines) |
| **code-agent.md** | 238 lines | test-driven-development, testing-anti-patterns | Code review request workflow (~30 lines) |
| **rust-agent.md** | 244 lines | test-driven-development, testing-anti-patterns | Code review request workflow (~30 lines) |
| **plan-review-agent.md** | 213 lines | verifying-plans | None (delegates properly) |
| **execute-review-agent.md** | 312 lines | None | Entire verification workflow (~200 lines) |
| **gatekeeper.md** | 288 lines | validating-review-feedback | None (delegates properly) |

**Evidence:**
- **Source:** All agent files in `/Users/tobyhede/src/cipherpowers/plugin/agents/`
- **Confidence:** HIGH
- **Implication:** Most agents follow mixed delegation pattern - some workflows in skills, some embedded

---

## Duplicated Workflow Patterns

### Pattern 1: Announcement Boilerplate

**Found in:** ALL 11 agents

**Current duplication:** ~8-15 lines per agent × 11 agents = ~110 lines

**Example (research-agent.md lines 28-42):**
```markdown
IMMEDIATELY announce:
```
I'm using the research-agent for comprehensive topic exploration.

Non-negotiable workflow:
1. Read all context files
2. Define research scope and questions
3. Explore from multiple entry points
4. Gather evidence from multiple sources
5. Identify gaps and uncertainties
6. Save structured findings to work directory
7. No conclusions without evidence
```
```

**Similar in:**
- commit-agent.md (lines 49-61)
- technical-writer.md (lines 76-100)
- ultrathink-debugger.md (lines 48-59)
- code-agent.md (lines 52-65)
- rust-agent.md (lines 60-71)
- plan-review-agent.md (lines 50-62)
- execute-review-agent.md (lines 28-40)
- gatekeeper.md (lines 32-34)

**Evidence:**
- **Confidence:** VERY HIGH
- **Should become skill?** NO - This is agent-specific enforcement, not a reusable workflow
- **Better approach:** Extract to agent template as example, not skill

### Pattern 2: Pre-Work Checklist

**Found in:** ALL agents except minimal ones

**Current duplication:** ~6-10 lines per agent × 9 agents = ~70 lines

**Example (code-agent.md lines 68-76):**
```markdown
BEFORE writing ANY code, you MUST:
- [ ] Confirm correct worktree
- [ ] Read README.md completely
- [ ] Read CLAUDE.md completely
- [ ] Read ${CLAUDE_PLUGIN_ROOT}principles/development.md
- [ ] Read ${CLAUDE_PLUGIN_ROOT}principles/testing.md
- [ ] Search for and read relevant skills
- [ ] Announce which skills you're applying
```

**Similar in:** rust-agent.md, ultrathink-debugger.md, technical-writer.md, research-agent.md, plan-review-agent.md, execute-review-agent.md, commit-agent.md

**Evidence:**
- **Confidence:** VERY HIGH
- **Should become skill?** NO - This is agent initialization, part of template/convention
- **Better approach:** Extract to agent template

### Pattern 3: Quality Gates Section

**Found in:** commit-agent, ultrathink-debugger, code-agent, rust-agent, plan-review-agent

**Current duplication:** ~15 lines × 5 agents = ~75 lines (IDENTICAL)

**Example (code-agent.md lines 209-224):**
```markdown
## Quality Gates

Quality gates are configured in ${CLAUDE_PLUGIN_ROOT}hooks/gates.json

When you complete work:
- SubagentStop hook will run project gates (check, test, etc.)
- Gate actions: CONTINUE (proceed), BLOCK (fix required), STOP (critical error)
- Gates can chain to other gates for complex workflows
- You'll see results in additionalContext and must respond appropriately

If a gate blocks:
1. Review the error output in the block reason
2. Fix the issues
3. Try again (hook re-runs automatically)
```

**Evidence:**
- **Source:** Multiple agent files, sections are byte-for-byte identical
- **Confidence:** VERY HIGH
- **Should become skill?** NO - This is documentation about the hooks system
- **Better approach:** Template fragment or standard section

### Pattern 4: Save to .work/ Directory

**Found in:** research-agent, plan-review-agent, execute-review-agent, technical-writer

**Current duplication:** ~20 lines per agent × 4 agents = ~80 lines

**Example (research-agent.md lines 105-147):**
```markdown
**YOU MUST save findings using this structure:**

```markdown
# Research Report: [Topic]

## Metadata
- Date: [YYYY-MM-DD]
- Researcher: research-agent
...
```

**File naming:** Save to `.work/{YYYY-MM-DD}-verify-research-{HHmmss}.md`
```

**Similar patterns in:**
- plan-review-agent.md (lines 163-198) - `.work/{YYYY-MM-DD}-verify-plan-{HHmmss}.md`
- execute-review-agent.md (lines 110-159) - `.work/{YYYY-MM-DD}-verify-execute-{HHmmss}.md`
- technical-writer.md (mode-specific save patterns)

**Evidence:**
- **Confidence:** HIGH
- **Should become skill?** MAYBE - Could be "work-directory-reporting" skill
- **Benefit:** Consistent file naming, report structure, announcement pattern

### Pattern 5: Rationalization Defenses

**Found in:** ALL full-template agents

**Current duplication:** ~20-40 lines per agent × 9 agents = ~270 lines (similar structure, different content)

**Example (systematic-debugging lines 215-227):**
```markdown
| Excuse | Reality |
|--------|---------|
| "Issue is simple, don't need process" | Simple issues have root causes too. Process is fast for simple bugs. |
| "Emergency, no time for process" | Systematic debugging is FASTER than guess-and-check thrashing. |
| "Just try this first, then investigate" | First fix sets the pattern. Do it right from the start. |
```

**Evidence:**
- **Confidence:** VERY HIGH
- **Should become skill?** NO - Content is agent-specific, format is template convention
- **Better approach:** Part of agent template with examples

### Pattern 6: Handling Bypass Requests Tables

**Found in:** research-agent, commit-agent, technical-writer, ultrathink-debugger, code-agent, rust-agent, plan-review-agent, execute-review-agent

**Current duplication:** ~8-12 lines per agent × 8 agents = ~80 lines

**Example (execute-review-agent.md lines 174-183):**
```markdown
| User Request | Your Response |
|--------------|---------------|
| "Tasks look good enough" | "Verification is MANDATORY. Checking each task against plan specification now." |
| "Just check the critical tasks" | "ALL tasks in batch must be verified. This is non-negotiable." |
```

**Evidence:**
- **Confidence:** VERY HIGH
- **Should become skill?** NO - Agent-specific enforcement responses
- **Better approach:** Agent template guidance

---

## Major Duplication: code-agent.md vs rust-agent.md

### Analysis

**Finding:** code-agent.md and rust-agent.md are **95% identical**

**Source:**
- `/Users/tobyhede/src/cipherpowers/plugin/agents/code-agent.md` (238 lines)
- `/Users/tobyhede/src/cipherpowers/plugin/agents/rust-agent.md` (244 lines)

**Only differences:**
1. **Agent metadata** (lines 1-5): name, description, color
2. **Rust-specific standards** (rust-agent lines 40-42):
   ```markdown
   - Rust guidelines: ${CLAUDE_PLUGIN_ROOT}standards/rust/microsoft-rust-guidelines.md
   - Rust dependency guidelines: ${CLAUDE_PLUGIN_ROOT}standards/rust/dependencies.md
   ```

**Everything else is IDENTICAL:**
- Skill activation (test-driven-development, testing-anti-patterns)
- Pre-implementation checklist
- TDD enforcement
- Project command execution
- Code review request workflow
- Completion criteria
- Handling bypass requests
- Rationalization defenses
- Quality gates section

**Evidence:**
- **Confidence:** VERY HIGH (verified line-by-line)
- **Implication:** This is the #1 refactoring priority

**Recommendation:**
Create `skills/development-agent-workflow/SKILL.md` containing:
- Pre-implementation checklist
- TDD workflow reference
- Project command execution requirements
- Code review request workflow
- Completion criteria
- Common rationalization defenses

Then:
- code-agent.md: Reference skill + general development standards
- rust-agent.md: Reference skill + Rust-specific standards
- Future language-agents: Reference skill + language-specific standards

**Benefit:**
- Reduce duplication from ~230 lines × N agents to ~30 lines per agent
- Single source of truth for development workflow
- Language-specific agents become 10-20 lines (metadata + language standards)

**Confidence:** VERY HIGH

---

## Candidate Skills to Extract

### Candidate 1: Execute Verification Workflow

**Found in:** execute-review-agent.md (embedded)

**Current duplication:** ~200 lines embedded in one agent

**Workflow pattern:**
1. Read plan tasks for batch
2. Read implementation changes
3. For each task, verify: COMPLETE / INCOMPLETE / DEVIATED
4. Categorize by severity: BLOCKING / NON-BLOCKING
5. Save structured review report
6. Announce saved file location

**Evidence:**
- **Source:** `/Users/tobyhede/src/cipherpowers/plugin/agents/execute-review-agent.md` (lines 23-243)
- **Confidence:** HIGH
- **Should become skill?** YES

**Extraction benefit:**
- Consistent with other verification skills (verifying-plans, conducting-code-review)
- Reusable if multiple execute verification scenarios emerge
- Separates "what to verify" (skill) from "enforcement" (agent)

**Proposed location:** `skills/verifying-execute/SKILL.md`

**Pattern alignment:**
- Similar structure to verifying-plans skill
- Uses same template pattern (verify-template.md)
- Fits dual-verification architecture

**Recommendation:** HIGH PRIORITY

### Candidate 2: Code Review Request Workflow

**Found in:** code-agent.md, rust-agent.md (identical sections)

**Current duplication:** ~30 lines × 2 agents = ~60 lines (will grow with more language agents)

**Workflow pattern:**
1. Implementation complete, tests pass, checks pass
2. Request code review before marking complete
3. Address ALL feedback (critical, high, medium, low)
4. Document why skipping requires technical impossibility + approval

**Evidence:**
- **Source:** code-agent.md (lines 109-147), rust-agent.md (lines 115-153)
- **Confidence:** VERY HIGH

**BUT - SKILL ALREADY EXISTS!**

**Discovery:** `skills/requesting-code-review/SKILL.md` exists (referenced in code-review-agent.md line 42)

**Problem:** code-agent and rust-agent EMBED the workflow instead of referencing the skill!

**Recommendation:**
- Remove embedded workflow from code-agent.md and rust-agent.md
- Add reference: `@${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md`
- Reduces each agent by ~30 lines

**Benefit:**
- Eliminates duplication immediately
- Uses existing skill
- Agents stay thin

**Recommendation:** HIGH PRIORITY (quick win)

### Candidate 3: Research Methodology

**Found in:** research-agent.md (embedded)

**Current duplication:** ~150 lines embedded in one agent

**Workflow pattern:**
1. Define research scope and questions
2. Explore from multiple entry points (4 angles)
3. Gather evidence from multiple sources
4. Assign confidence levels (HIGH/MEDIUM/LOW)
5. Identify gaps and uncertainties
6. Save structured findings report

**Evidence:**
- **Source:** `/Users/tobyhede/src/cipherpowers/plugin/agents/research-agent.md` (lines 54-145)
- **Confidence:** HIGH
- **Should become skill?** MAYBE

**Considerations:**
- Research-agent is only user of this workflow currently
- No duplication (only one agent does research)
- Skill extraction justified if we expect more research scenarios

**Benefit if extracted:**
- Reusable for different research contexts (codebase, API, problem investigation)
- Consistent methodology across research tasks
- Agent becomes thinner enforcement layer

**Proposed location:** `skills/research-methodology/SKILL.md`

**Recommendation:** MEDIUM PRIORITY (wait for second use case)

### Candidate 4: Complex Debugging Techniques

**Found in:** ultrathink-debugger.md (embedded)

**Current duplication:** ~150 lines of detailed techniques

**Workflow enhancements:**
- Multi-component diagnostic logging
- Environment differential analysis
- Network inspection
- Concurrency and race condition analysis
- Integration debugging

**Evidence:**
- **Source:** `/Users/tobyhede/src/cipherpowers/plugin/agents/ultrathink-debugger.md` (lines 78-103, 287-362)
- **Confidence:** HIGH
- **Should become skill?** MAYBE

**Considerations:**
- These enhance systematic-debugging skill for complex scenarios
- Could be separate skill: "complex-debugging-techniques"
- Or could be examples/extension documentation for systematic-debugging

**Benefit if extracted:**
- Reusable debugging toolkit
- Reference from regular debugging contexts
- Separates technique library from agent enforcement

**Proposed location:**
- Option A: `skills/complex-debugging-techniques/SKILL.md`
- Option B: `skills/systematic-debugging/complex-scenarios.md` (extension doc)

**Recommendation:** MEDIUM PRIORITY (complex scenarios are agent-specific currently)

---

## Agents That Should Reference Existing Skills But Don't

### code-agent.md & rust-agent.md

**Missing reference:** `requesting-code-review` skill

**Current state:** Both agents embed ~30 lines of code review request workflow

**Evidence:**
- requesting-code-review skill exists: `/Users/tobyhede/src/cipherpowers/plugin/skills/requesting-code-review/`
- code-agent.md embeds workflow: lines 109-147
- rust-agent.md embeds workflow: lines 115-153
- code-review-agent.md properly references it: line 42

**Fix:**
Replace embedded workflow with:
```markdown
YOU MUST READ:
- Code Review Request: @${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md
```

**Benefit:** Immediate ~30 line reduction per agent, single source of truth

**Confidence:** VERY HIGH

---

## Patterns That Should NOT Become Skills

### 1. Announcement Patterns
**Why:** Agent-specific enforcement, not reusable workflow
**Better approach:** Agent template guidance

### 2. Pre-Work Checklists
**Why:** Agent initialization convention, not workflow
**Better approach:** Standard agent template section

### 3. Quality Gates Documentation
**Why:** Documentation about infrastructure, not workflow
**Better approach:** Template fragment or standard section

### 4. Rationalization Defenses
**Why:** Content is agent-specific, format is template convention
**Better approach:** Agent template examples

### 5. Handling Bypass Requests
**Why:** Agent-specific enforcement responses
**Better approach:** Agent template pattern

**Evidence:**
- **Confidence:** HIGH
- **Rationale:** These are structural elements of agent design, not reusable workflows

---

## Recommendations

### Priority 1: HIGH (Immediate Action)

#### 1.1: Extract Development Agent Workflow Skill

**Problem:** code-agent.md and rust-agent.md are 95% identical (~230 lines each)

**Solution:** Create `skills/development-agent-workflow/SKILL.md`

**Contents:**
- Pre-implementation checklist (read context, verify worktree)
- TDD workflow reference (delegate to test-driven-development skill)
- Project command execution requirements (tests, checks)
- Code review request workflow (delegate to requesting-code-review skill)
- Completion criteria
- Common development rationalization defenses

**Impact:**
- Reduce code-agent.md from 238 lines → ~50 lines (metadata + enforcement)
- Reduce rust-agent.md from 244 lines → ~50 lines (metadata + Rust standards + enforcement)
- Future language agents: ~50 lines each (just add language-specific standards)
- **Eliminates:** ~180 lines of duplication per agent
- **Enables:** Easy addition of python-agent, typescript-agent, go-agent, etc.

**Confidence:** VERY HIGH

#### 1.2: Fix Missing Skill References in code-agent & rust-agent

**Problem:** Both embed requesting-code-review workflow instead of referencing skill

**Solution:**
1. Remove lines 109-147 from code-agent.md
2. Remove lines 115-153 from rust-agent.md
3. Add skill reference: `@${CLAUDE_PLUGIN_ROOT}skills/requesting-code-review/SKILL.md`

**Impact:**
- Immediate ~30 line reduction per agent
- Fixes inconsistency (code-review-agent references it properly)
- Single source of truth for review request workflow

**Confidence:** VERY HIGH

#### 1.3: Extract Execute Verification Workflow Skill

**Problem:** execute-review-agent.md embeds entire ~200 line workflow

**Solution:** Create `skills/verifying-execute/SKILL.md`

**Contents:**
- Read plan tasks for batch
- Read implementation changes
- Task verification algorithm (COMPLETE/INCOMPLETE/DEVIATED)
- Severity categorization (BLOCKING/NON-BLOCKING)
- Structured report template
- Verification criteria and examples

**Impact:**
- Reduce execute-review-agent.md from 312 lines → ~100 lines
- Consistent with verifying-plans, conducting-code-review patterns
- Reusable if more execute verification scenarios emerge

**Confidence:** HIGH

### Priority 2: MEDIUM (Consider After Priority 1)

#### 2.1: Create Agent Template Documentation

**Problem:** ~500 lines of boilerplate duplicated across agents (announcement, pre-work, quality gates, rationalization defenses, bypass handling)

**Solution:** Create `plugin/templates/agent-template.md` with:
- Standard sections (announcement, pre-work, rationalization, bypass handling)
- Guidance on when to embed vs delegate
- Examples from existing agents
- Template fragments for copy-paste

**Impact:**
- Reduces mental burden for agent creation
- Ensures consistency in new agents
- Doesn't reduce existing agent sizes (but prevents future duplication)

**Confidence:** HIGH

#### 2.2: Extract Research Methodology Skill (Wait for Second Use Case)

**Problem:** research-agent.md embeds ~150 lines of research workflow

**Current state:** Only one agent uses this workflow

**Recommendation:** Wait until second research use case emerges, then extract

**Rationale:** Premature extraction without duplication = over-engineering

**Confidence:** MEDIUM

### Priority 3: LOW (Future Consideration)

#### 3.1: Complex Debugging Techniques

**Problem:** ultrathink-debugger embeds ~150 lines of complex debugging techniques

**Options:**
- Create separate skill: `complex-debugging-techniques`
- Add as extension doc to systematic-debugging skill

**Recommendation:** Wait for duplication or user request

**Confidence:** MEDIUM

---

## Gaps and Uncertainties

### Gap 1: Agent Template vs Skill Boundary

**Question:** When should boilerplate be in a template vs a skill?

**Current uncertainty:**
- Announcement patterns are duplicated but agent-specific
- Pre-work checklists are similar but not identical
- Some duplication is structural (template) vs workflow (skill)

**Evidence needed:** Review cipherpowers philosophy docs on agent design

**Impact:** Affects whether we create more skills or improve template guidance

### Gap 2: Language-Specific Agent Architecture

**Question:** Should language agents inherit from base development-agent, or reference shared skill?

**Current approach:** code-agent and rust-agent are separate full agents

**Alternatives:**
1. Shared skill (recommended in Priority 1.1)
2. Base agent with overrides
3. Composition with multiple skills

**Uncertainty:** Which approach aligns with cipherpowers plugin architecture philosophy?

### Gap 3: Work Directory Reporting Pattern

**Question:** Should timestamped .work/ file saving be a skill?

**Found in:** 4 agents with similar patterns

**Uncertainty:**
- Is this workflow enough to warrant a skill?
- Or is it just a file naming convention?
- Should report structure be standardized?

**Impact:** Could reduce ~80 lines of duplication if skill created

---

## Summary

### High-Confidence Findings

1. **Major duplication identified:** code-agent and rust-agent are 95% identical
   - **Confidence:** VERY HIGH
   - **Evidence:** Line-by-line comparison
   - **Impact:** ~230 lines per agent, will grow with more language agents

2. **Missing skill references:** code-agent and rust-agent embed requesting-code-review workflow
   - **Confidence:** VERY HIGH
   - **Evidence:** Skill exists, agents don't reference it
   - **Impact:** Quick win, ~30 lines per agent

3. **Execute verification embedded:** execute-review-agent contains full workflow
   - **Confidence:** HIGH
   - **Evidence:** ~200 lines of workflow logic
   - **Impact:** Inconsistent with other verification patterns

4. **Boilerplate duplication:** ~500 lines across agents (announcement, pre-work, quality gates, etc.)
   - **Confidence:** VERY HIGH
   - **Evidence:** Line-by-line comparison
   - **Impact:** Template improvement opportunity

### Agents Already Doing It Right

1. **code-review-agent.md:** 31 lines, delegates to conducting-code-review skill ✓
2. **review-collation-agent.md:** 128 lines, delegates to dual-verification skill ✓
3. **commit-agent.md:** Properly delegates to commit-workflow skill ✓
4. **plan-review-agent.md:** Properly delegates to verifying-plans skill ✓
5. **gatekeeper.md:** Properly delegates to validating-review-feedback skill ✓

**Pattern:** Minimal agents with strong skill delegation

### Agents Needing Refactoring

1. **code-agent.md & rust-agent.md:** 95% identical, embed requesting-code-review workflow
   - **Priority:** HIGH
   - **Action:** Extract to shared skill, add skill references

2. **execute-review-agent.md:** Embeds entire verification workflow
   - **Priority:** HIGH
   - **Action:** Extract to skills/verifying-execute/SKILL.md

3. **research-agent.md:** Embeds research methodology
   - **Priority:** MEDIUM
   - **Action:** Wait for second use case, then extract

4. **ultrathink-debugger.md:** Embeds complex debugging techniques
   - **Priority:** LOW
   - **Action:** Consider after systematic-debugging skill usage analysis

---

## Next Steps

### Immediate Actions (Priority 1)

1. **Create skills/development-agent-workflow/SKILL.md**
   - Extract common development workflow from code-agent/rust-agent
   - Reference test-driven-development and requesting-code-review skills
   - Include completion criteria and common defenses

2. **Refactor code-agent.md and rust-agent.md**
   - Replace embedded workflows with skill references
   - Reduce to ~50 lines each (metadata + language-specific standards + enforcement)

3. **Create skills/verifying-execute/SKILL.md**
   - Extract workflow from execute-review-agent.md
   - Follow pattern of verifying-plans skill
   - Use verify-template.md for report structure

4. **Update execute-review-agent.md**
   - Replace embedded workflow with skill reference
   - Reduce to ~100 lines (metadata + enforcement)

### Follow-Up Actions (Priority 2)

5. **Create comprehensive agent template documentation**
   - Document standard sections
   - Provide template fragments
   - Examples from existing agents
   - Guidance on embed vs delegate

6. **Monitor for research methodology duplication**
   - Wait for second research use case
   - Extract to skill when duplication emerges

### Future Considerations (Priority 3)

7. **Review complex debugging techniques**
   - Analyze if they should be separate skill
   - Or extension of systematic-debugging
   - Wait for user feedback or duplication

---

## File Paths for Implementation

**Skills to create:**
- `/Users/tobyhede/src/cipherpowers/plugin/skills/development-agent-workflow/SKILL.md`
- `/Users/tobyhede/src/cipherpowers/plugin/skills/verifying-execute/SKILL.md`

**Agents to refactor:**
- `/Users/tobyhede/src/cipherpowers/plugin/agents/code-agent.md` (reduce from 238 → ~50 lines)
- `/Users/tobyhede/src/cipherpowers/plugin/agents/rust-agent.md` (reduce from 244 → ~50 lines)
- `/Users/tobyhede/src/cipherpowers/plugin/agents/execute-review-agent.md` (reduce from 312 → ~100 lines)

**Template to create:**
- `/Users/tobyhede/src/cipherpowers/plugin/templates/agent-template.md` (enhanced with standard sections)

**Total line reduction potential:** ~400 lines eliminated through skill extraction

---

## Overall Assessment

**Status:** Analysis complete - clear actionable recommendations identified

**High-Priority Opportunities:**
1. Development agent workflow extraction (eliminates ~180 lines per language agent)
2. Fix missing skill references (quick win, ~30 lines per agent)
3. Execute verification workflow extraction (eliminates ~200 lines, improves consistency)

**Medium-Priority Opportunities:**
1. Agent template improvement (prevents future duplication)
2. Research methodology skill (wait for duplication)

**Low-Priority Opportunities:**
1. Complex debugging techniques (wait for use case)

**Overall Impact:**
- Immediate: ~400 lines eliminated
- Future: Enables easy addition of new language agents (~50 lines each vs ~230 lines)
- Consistency: All verification agents follow same pattern (plan, execute, code, docs)
