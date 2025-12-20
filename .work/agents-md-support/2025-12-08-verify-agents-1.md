# Agent Consistency Review #1

## Metadata
- Date: 2025-12-08
- Reviewer: technical-writer #1
- Scope: plugin/agents/**
- Exemplars: rust-exec-agent.md, code-review-agent.md, commit-agent.md

## Exemplar Pattern Analysis

### Standard Structure Identified from Exemplars

**1. YAML Frontmatter (Required fields):**
- `name`: Agent name
- `description`: Clear role description
- `color`: Color identifier
- Optional: `model` (e.g., haiku for exec agents)

**2. Opening Statement:**
- One sentence describing the agent persona
- Example: "You are a meticulous, systematic git committer."

**3. `<instructions>` Wrapper:**
- ALL instructions must be inside `<instructions>` tags
- Clean structure with clear hierarchy

**4. Section Structure (in order):**
1. `## Instructions` heading
2. `## MANDATORY: Skill Activation` section
   - Clear skill name and path
   - Tool invocation syntax
   - "Do NOT proceed" enforcement line
3. `## MANDATORY: Context` section (if needed)
   - Files to read before starting
4. `## MANDATORY: Standards` section (if applicable)
   - Standards files to follow
5. `## Save Workflow` section (if applicable)
   - File path pattern
   - "Announce file path" instruction
6. Close `</instructions>`

**5. Thin Skill-Delegation Pattern:**
- ~30-36 lines total (exemplars range from 28-33 lines)
- NO workflow logic in agent (that's in skills)
- NO rationalization defenses (that's in skills)
- NO red flags tables (that's in skills)
- Pure delegation: read context, activate skill, follow standards
- Optional: Additional context after `</instructions>` (like in review-collation-agent)

**6. Path References:**
- Use `${CLAUDE_PLUGIN_ROOT}` for plugin paths
- Use `@` syntax for project files (README.md, CLAUDE.md)

**7. Conciseness:**
- Exemplars: 28-33 lines
- Target: ~30-50 lines max
- NO duplication of skill content

## Agent Compliance Matrix

| Agent | Lines | Compliant? | Issues |
|-------|-------|------------|--------|
| rust-exec-agent.md | 36 | ✅ YES | EXEMPLAR - perfect pattern |
| code-review-agent.md | 33 | ✅ YES | EXEMPLAR - perfect pattern |
| commit-agent.md | 28 | ✅ YES | EXEMPLAR - perfect pattern |
| code-exec-agent.md | 30 | ✅ YES | Matches pattern exactly |
| code-agent.md | 32 | ✅ YES | Matches pattern, multiple skills |
| rust-agent.md | 38 | ✅ YES | Matches pattern, multiple skills |
| plan-review-agent.md | 36 | ✅ YES | Matches pattern |
| execute-review-agent.md | 41 | ⚠️ MOSTLY | Extra context section but acceptable |
| technical-writer.md | 46 | ⚠️ MOSTLY | Extra mode detection but acceptable |
| research-agent.md | 38 | ✅ YES | Matches pattern with dual-verification note |
| gatekeeper.md | 35 | ✅ YES | Matches pattern, includes input spec |
| review-collation-agent.md | 84 | ⚠️ HYBRID | Has content after `</instructions>` but intentional |
| ultrathink-debugger.md | 116 | ❌ NO | Over-engineered, contains workflow logic |
| path-test-agent.md | 68 | ❌ NO | Test agent, not production (acceptable exception) |

## Non-Compliant Agents

### 1. ultrathink-debugger.md (MAJOR - NON-COMPLIANT)

**File:** `plugin/agents/ultrathink-debugger.md`

**Issue Type:** OVER-ENGINEERED - Contains workflow logic that should be in skills

**Current State:**
- 116 lines total
- Contains extensive workflow instructions inside `<instructions>`
- Has "Announcement" section with script
- Has "Specialization: When to Use This Agent" section with decision logic
- Has "Complex Scenario Techniques" section with 4 subsections of detailed workflow
- Has "Red Flags" table
- Has "Completion Criteria" checklist
- Has additional sections AFTER `</instructions>` (Purpose, Behavioral Traits)

**Expected State (from exemplars):**
- ~30-50 lines
- `## MANDATORY: Skill Activation` section only
- `## MANDATORY: Context` section only
- `## MANDATORY: Standards` section only
- NO workflow logic (delegated to systematic-debugging skill)
- NO announcement scripts
- NO red flags (in skill, not agent)
- NO completion criteria (in skill, not agent)

**Recommendation:**
1. Extract all workflow content to skills:
   - "Announcement" script → systematic-debugging skill
   - "Specialization: When to Use" → skill frontmatter `when_to_use`
   - "Complex Scenario Techniques" → systematic-debugging skill body
   - "Red Flags" table → systematic-debugging skill
   - "Completion Criteria" → systematic-debugging skill
2. Reduce agent to pure delegation pattern:
   ```markdown
   ---
   name: ultrathink-debugger
   description: Complex debugging specialist using systematic investigation
   color: red
   ---

   You are a complex debugging specialist.

   <instructions>
   ## Instructions

   ## MANDATORY: Skill Activation

   Use and follow these debugging skills:

   **Primary:**
   - Skill: `cipherpowers:systematic-debugging`
   - Path: `${CLAUDE_PLUGIN_ROOT}skills/systematic-debugging/SKILL.md`

   **Supporting (when applicable):**
   - Skill: `cipherpowers:root-cause-tracing`
   - Skill: `cipherpowers:defense-in-depth`

   Do NOT proceed without activating systematic-debugging skill.

   ## MANDATORY: Context

   Read before starting:
   - @README.md
   - @CLAUDE.md

   ## MANDATORY: Standards

   Read and follow:
   - ${CLAUDE_PLUGIN_ROOT}principles/testing.md
   - ${CLAUDE_PLUGIN_ROOT}principles/development.md

   </instructions>
   ```
3. Target: ~35 lines (matches exemplar pattern)

**Severity:** BLOCKING - This agent violates the thin skill-delegation pattern significantly

---

### 2. review-collation-agent.md (MINOR - HYBRID PATTERN)

**File:** `plugin/agents/review-collation-agent.md`

**Issue Type:** INCONSISTENT STRUCTURE - Contains significant content after `</instructions>`

**Current State:**
- 84 lines total
- Has correct `<instructions>` structure (42 lines)
- Has extensive content AFTER `</instructions>`:
  - "Non-Negotiable Requirements" section
  - "Final Message Format" section (large template)
  - "Red Flags" table
  - "Purpose" section
  - "Capabilities" section

**Expected State (from exemplars):**
- All content inside `<instructions>` tags
- OR content after `</instructions>` extracted to skill

**Analysis:**
This agent appears to follow a hybrid pattern intentionally:
- Core delegation inside `<instructions>`
- Additional guidance outside for orchestration
- Unlike ultrathink-debugger, this seems like agent-specific orchestration logic (not reusable workflow)

**Recommendation:**
**Option A (Preferred):** Move orchestration content into dual-verification skill:
- "Non-Negotiable Requirements" → skill workflow
- "Final Message Format" → skill output section
- "Red Flags" → skill red flags section
- Keep only delegation in agent

**Option B (Acceptable):** Document this as intentional hybrid pattern:
- Agents can have additional content after `</instructions>` for orchestration logic
- Reserve for agents with unique orchestration needs
- Must still follow thin delegation inside `<instructions>`

**Severity:** MODERATE - Consider standardizing but not blocking

---

### 3. technical-writer.md (MINOR - MODE DETECTION)

**File:** `plugin/agents/technical-writer.md`

**Issue Type:** INCONSISTENT STRUCTURE - Extra "Mode Detection" section

**Current State:**
- 46 lines total
- Has `## Mode Detection` section BEFORE skill activation
- Otherwise follows exemplar pattern

**Expected State (from exemplars):**
- No mode detection section
- Pure delegation pattern

**Analysis:**
- This agent needs mode detection for VERIFICATION vs EXECUTION
- Mode detection is agent-specific logic (not in skill)
- Only adds ~6 lines to standard pattern
- Still relatively thin (46 lines vs exemplar ~30-35)

**Recommendation:**
**Accept as valid variation** - Mode detection is agent-specific orchestration that doesn't belong in skills. Document as acceptable pattern:
- Mode detection sections allowed when agent serves multiple modes
- Must come BEFORE skill activation
- Should be concise (~5-10 lines max)

**Severity:** LOW - Acceptable variation for multi-mode agents

---

### 4. execute-review-agent.md (MINOR - EXTRA CONTEXT)

**File:** `plugin/agents/execute-review-agent.md`

**Issue Type:** INCONSISTENT STRUCTURE - Extra context explanation

**Current State:**
- 41 lines total
- Has `## Context` section with clarification about scope
- Otherwise follows exemplar pattern

**Expected State (from exemplars):**
- Context section lists files to read
- No scope clarifications (should be in skill)

**Analysis:**
- Adds 5 lines explaining role separation
- Helps prevent scope confusion
- Still thin (41 lines vs exemplar ~30-35)

**Recommendation:**
**Accept with minor refinement** - Consider moving scope clarification to verifying-plan-execution skill's "When to Use" section, but current form is acceptable for clarity.

**Severity:** LOW - Acceptable for role clarity

---

### 5. path-test-agent.md (EXCEPTION)

**File:** `plugin/agents/path-test-agent.md`

**Issue Type:** TEST AGENT - Not production pattern

**Current State:**
- 68 lines
- Contains test procedure logic
- Does NOT follow thin delegation pattern
- Intentionally contains workflow for testing

**Expected State:**
- Test agents can have different structure
- Production agents must follow exemplar pattern

**Recommendation:**
**No action required** - This is a test/utility agent, not a production agent. Mark as exception to pattern.

**Severity:** N/A - Test agent exception

## Content for Skill Extraction

### From ultrathink-debugger.md → systematic-debugging skill

**1. Announcement Script (lines 37-48):**
```markdown
## Announcement

IMMEDIATELY announce:
```
I'm using the ultrathink-debugger agent for complex debugging.

Following systematic-debugging skill (4 phases) with opus-level techniques:
1. Root Cause Investigation (evidence first)
2. Pattern Analysis (find working examples)
3. Hypothesis Testing (one variable at a time)
4. Implementation (fix root cause, verify)
```
```

**Recommendation:** Move to systematic-debugging skill's workflow section

---

**2. Specialization Guide (lines 50-59):**
```markdown
## Specialization: When to Use This Agent

**Use ultrathink-debugger for:**
- Multi-component failures (data flows through 3+ layers)
- Environment-specific issues (works locally, fails in production/CI)
- Timing/concurrency issues (intermittent, race conditions)
- Integration failures (external APIs, authentication)
- Production emergencies requiring forensics

**Do NOT use for simple bugs** - use regular debugging.
```

**Recommendation:** Move to systematic-debugging skill's `when_to_use` frontmatter

---

**3. Complex Scenario Techniques (lines 61-87):**
Entire section with subsections:
- Multi-Component Systems
- Environment-Specific Failures
- Timing/Concurrency Issues
- Integration Failures

**Recommendation:** Move to systematic-debugging skill as technique sections or appendix

---

**4. Red Flags Table (lines 89-95):**
```markdown
## Red Flags - Return to Skill

| Excuse | Reality |
|--------|---------|
| "I see the issue, skip process" | Complex bugs DECEIVE. Use systematic-debugging. |
| "Fix where error appears" | Symptom ≠ root cause. Use root-cause-tracing. |
| "Should work now" | NO claims without verification command. |
| "No time for process" | Systematic is FASTER than thrashing. |
```

**Recommendation:** Move to systematic-debugging skill's red flags section

---

**5. Completion Criteria (lines 97-103):**
```markdown
## Completion Criteria

NOT complete until:
- [ ] Root cause identified (not symptoms)
- [ ] Fix addresses root cause
- [ ] Verification command run with evidence
- [ ] No regression in related functionality
```

**Recommendation:** Move to systematic-debugging skill's completion section

---

**6. Purpose & Behavioral Traits (lines 105-116):**
After `</instructions>` close tag

**Recommendation:**
- Purpose → skill description
- Behavioral Traits → skill principles or personality section

### From review-collation-agent.md (Optional)

**1. Non-Negotiable Requirements (inside instructions):**
Could move to dual-verification skill workflow

**2. Final Message Format (inside instructions):**
Could move to dual-verification skill output section

**3. Red Flags Table (inside instructions):**
Could move to dual-verification skill

**Note:** These are less critical since the agent is relatively thin and may need orchestration logic.

## Summary

### Quantitative Analysis
- **Total agents reviewed:** 14
- **Fully compliant:** 9 (64%)
- **Mostly compliant (minor issues):** 4 (29%)
- **Non-compliant:** 1 (7%)
- **Exception (test agent):** 1

### Compliance by Category

**✅ Exemplar Pattern (9 agents):**
1. rust-exec-agent.md (36 lines)
2. code-review-agent.md (33 lines)
3. commit-agent.md (28 lines)
4. code-exec-agent.md (30 lines)
5. code-agent.md (32 lines)
6. rust-agent.md (38 lines)
7. plan-review-agent.md (36 lines)
8. research-agent.md (38 lines)
9. gatekeeper.md (35 lines)

**⚠️ Minor Variations (4 agents):**
1. execute-review-agent.md (41 lines) - Extra context for role clarity
2. technical-writer.md (46 lines) - Mode detection section
3. review-collation-agent.md (84 lines) - Hybrid pattern with post-instructions content

**❌ Non-Compliant (1 agent):**
1. ultrathink-debugger.md (116 lines) - BLOCKING - Contains extensive workflow logic

**🧪 Exception (1 agent):**
1. path-test-agent.md (68 lines) - Test agent, different pattern acceptable

### Key Findings

**1. Pattern Adherence:**
- 64% of production agents follow exemplar pattern exactly
- Pattern is clear: ~30-40 lines, pure delegation, no workflow logic
- Pattern is achievable and maintainable

**2. Anti-Pattern Identified:**
- ultrathink-debugger violates pattern significantly (116 lines vs ~35 expected)
- Contains workflow logic, red flags, completion criteria that belong in skills
- This is the OLD pattern (pre-refactor) that should be updated

**3. Acceptable Variations:**
- Mode detection (technical-writer) - agent-specific orchestration
- Role clarification (execute-review-agent) - prevents scope confusion
- Hybrid pattern (review-collation-agent) - orchestration-heavy agents may need it

**4. Content Extraction Opportunity:**
- ~60 lines of content from ultrathink-debugger should move to systematic-debugging skill
- This would reduce agent from 116 → ~35 lines (70% reduction)
- Aligns with "83% reduction" mentioned in CLAUDE.md architecture section

### Recommendations Priority

**PRIORITY 1 (BLOCKING):**
1. Refactor ultrathink-debugger.md to match exemplar pattern
   - Extract workflow to systematic-debugging skill
   - Reduce agent to pure delegation (~35 lines)
   - This is the last agent that hasn't been updated to thin delegation pattern

**PRIORITY 2 (OPTIONAL IMPROVEMENTS):**
1. Consider extracting review-collation-agent post-instructions content to dual-verification skill
2. Document acceptable variations:
   - Mode detection sections for multi-mode agents
   - Brief role clarifications for scope-sensitive agents
   - Hybrid pattern for orchestration-heavy agents (if kept)

**PRIORITY 3 (DOCUMENTATION):**
1. Update agent-template.md to reflect discovered pattern
2. Add "Acceptable Variations" section to template
3. Document line count target (~30-50 lines)

### Pattern Documentation

**The Thin Skill-Delegation Pattern:**

```markdown
---
name: agent-name
description: Clear role description
color: color
---

One sentence persona statement.

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the [skill-name] skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/[skill-name]/SKILL.md`

Tool: `Skill(skill: "cipherpowers:[skill-name]")`

Do NOT proceed without completing skill activation.

## MANDATORY: Context

Read before starting:
- @README.md
- @CLAUDE.md

## MANDATORY: Standards

Read and follow:
- ${CLAUDE_PLUGIN_ROOT}standards/[standard].md

## Save Workflow (if applicable)

Save [output] to: `.work/{YYYY-MM-DD}-[type]-{HHmmss}.md`

Announce file path in final response.

</instructions>
```

**Target:** 30-50 lines
**Key Principle:** Workflow logic lives in skills, agents are pure dispatchers

---

## Conclusion

The codebase shows strong adoption of the thin skill-delegation pattern (64% full compliance), with only one agent (ultrathink-debugger) remaining in the old pre-refactor pattern. The exemplar pattern is clear, consistent, and effective.

**Main action required:** Refactor ultrathink-debugger.md to match the exemplar pattern by extracting its workflow logic to the systematic-debugging skill.

**Status:** MOSTLY COMPLIANT with one BLOCKING issue
