# Agent Consistency Review #2

## Metadata
- Date: 2025-12-08
- Reviewer: code-agent #2
- Scope: plugin/agents/**
- Exemplars: rust-exec-agent.md (35 lines), code-review-agent.md (32 lines), commit-agent.md (28 lines)

## Exemplar Pattern Analysis

### Standard Structure (from 3 exemplars)

**YAML Frontmatter (Required fields):**
```yaml
---
name: agent-name
description: Brief description
color: color-name
model: haiku  # Optional - only for exec agents
---
```

**Opening Statement:**
- Single line persona description
- Example: "You are a meticulous, pragmatic principal engineer acting as a code reviewer."

**`<instructions>` Wrapper:**
- MANDATORY wrapper: `<instructions>` ... `</instructions>`
- All instructions contained within this wrapper

**Section Structure (Inside `<instructions>`):**

1. **## Instructions** header

2. **## MANDATORY: Skill Activation** (REQUIRED)
   - Declares skill(s) being used
   - Path to skill: `${CLAUDE_PLUGIN_ROOT}skills/.../SKILL.md`
   - Tool invocation syntax: `Skill(skill: "cipherpowers:skill-name")`
   - Non-negotiable statement: "Do NOT proceed without completing skill activation."

3. **## MANDATORY: Context** (REQUIRED)
   - README.md and CLAUDE.md references
   - Additional context as needed

4. **## MANDATORY: Standards** (OPTIONAL)
   - Standards references using `${CLAUDE_PLUGIN_ROOT}standards/...`
   - Only when standards are needed

5. **## Save Workflow** (OPTIONAL)
   - Only for verification/review agents
   - File path template with timestamp
   - "Announce file path in final response."

**Characteristics:**
- **Concise:** 28-37 lines (avg 31.67 lines)
- **Pure delegation:** Workflow logic lives in skills
- **No rationalization defenses:** Skills handle enforcement
- **No red flags tables:** Skills handle anti-patterns
- **No detailed procedures:** Skills provide detailed steps
- **Thin wrapper:** Just context + skill activation + save instructions

### Pattern Summary

Exemplar agents follow a **thin skill-delegation pattern**:
1. Minimal YAML frontmatter
2. Single-line persona
3. `<instructions>` wrapper
4. Skill activation (path + tool syntax)
5. Context reading (README, CLAUDE, standards)
6. Save workflow (if applicable)
7. NO workflow logic, NO red flags, NO detailed procedures

## Agent Compliance Matrix

| Agent | Lines | Compliant? | Issues |
|-------|-------|------------|--------|
| commit-agent.md | 28 | ✅ YES | EXEMPLAR - Perfect pattern |
| code-review-agent.md | 32 | ✅ YES | EXEMPLAR - Perfect pattern |
| rust-exec-agent.md | 35 | ✅ YES | EXEMPLAR - Perfect pattern |
| code-exec-agent.md | 29 | ✅ YES | Perfect pattern match |
| code-agent.md | 31 | ✅ YES | Perfect pattern match |
| rust-agent.md | 37 | ✅ YES | Perfect pattern match |
| research-agent.md | 37 | ✅ YES | Perfect pattern match |
| plan-review-agent.md | 35 | ✅ YES | Perfect pattern match |
| execute-review-agent.md | 40 | ✅ YES | Perfect pattern match (extra context note) |
| gatekeeper.md | 34 | ✅ YES | Perfect pattern match |
| technical-writer.md | 45 | ✅ YES | Perfect pattern + mode detection (justified) |
| path-test-agent.md | 67 | ⚠️ SPECIAL | Test agent - special purpose, not production |
| review-collation-agent.md | 83 | ❌ NO | OVER-ENGINEERED - contains workflow logic |
| ultrathink-debugger.md | 115 | ❌ NO | OVER-ENGINEERED - contains workflow logic |

## Compliant Agents

### Exemplar-Level Compliance (28-40 lines)

**commit-agent.md (28 lines) ✅**
- Perfect exemplar
- YAML frontmatter ✓
- Single-line persona ✓
- `<instructions>` wrapper ✓
- Skill activation with path + tool syntax ✓
- Standards references ✓
- No workflow logic ✓

**code-review-agent.md (32 lines) ✅**
- Perfect exemplar
- Includes save workflow section ✓
- Pure delegation to conducting-code-review skill ✓

**rust-exec-agent.md (35 lines) ✅**
- Perfect exemplar
- Includes `model: haiku` for exec agent ✓
- Pure delegation to following-plans skill ✓

**code-exec-agent.md (29 lines) ✅**
- Matches exemplar pattern exactly
- Includes `model: haiku` ✓

**code-agent.md (31 lines) ✅**
- Matches exemplar pattern exactly
- Activates multiple skills appropriately ✓

**rust-agent.md (37 lines) ✅**
- Matches exemplar pattern exactly
- Includes Rust-specific standards ✓

**research-agent.md (37 lines) ✅**
- Matches exemplar pattern exactly
- Includes dual-verification context note ✓

**plan-review-agent.md (35 lines) ✅**
- Matches exemplar pattern exactly
- Includes save workflow ✓

**execute-review-agent.md (40 lines) ✅**
- Matches exemplar pattern exactly
- Extra context note about separation of concerns ("Not your job") - justified ✓

**gatekeeper.md (34 lines) ✅**
- Matches exemplar pattern exactly
- Includes "Required Input" section - appropriate for orchestration agent ✓

**technical-writer.md (45 lines) ✅**
- Slightly longer but justified
- Includes "Mode Detection" section (VERIFICATION vs EXECUTION) ✓
- Conditional save workflow based on mode ✓
- Still follows thin delegation pattern ✓

## Non-Compliant Agents

### 1. review-collation-agent.md (83 lines) ❌

**Issue Type:** OVER-ENGINEERED

**Current State:**
- 83 lines (262% of exemplar average)
- Contains detailed "Non-Negotiable Requirements" section
- Contains "Final Message Format" with exact template
- Contains "Red Flags - Return to Skill" table
- Contains "Purpose" and "Capabilities" sections outside `<instructions>`

**Expected State (from exemplars):**
- ~30-40 lines
- Pure delegation to dual-verification skill
- No workflow logic in agent
- No red flags table
- No detailed procedures

**Violations:**
1. **Workflow logic in agent:**
   ```markdown
   ## Non-Negotiable Requirements

   1. Read BOTH reviews completely before starting
   2. Use template EXACTLY (no custom sections)
   3. Mark exclusive issues as "pending cross-check"
   4. Save report to .work/ directory
   5. Announce `/cipherpowers:revise common` availability in final message
   ```
   → This belongs in `dual-verification` skill

2. **Detailed message template in agent:**
   ```markdown
   ## Final Message Format

   ```
   Collated report saved to: [path]
   ...
   ```
   ```
   → This belongs in skill or template

3. **Red flags table in agent:**
   ```markdown
   ## Red Flags - Return to Skill

   | Excuse | Reality |
   ...
   ```
   → Skills handle rationalization defenses, not agents

4. **Purpose/Capabilities sections outside `<instructions>`:**
   - These sections duplicate skill content
   - Agents should be thin wrappers

**Recommendation:**

Extract to skill:
- Non-negotiable requirements → `dual-verification` skill workflow
- Final message format → skill or template
- Red flags table → skill enforcement section
- Purpose/capabilities → skill description

Reduce agent to:
```markdown
---
name: review-collation-agent
description: Systematic collation of dual independent reviews (works for any review type)
color: cyan
---

You are the Review Collator - the systematic analyst who compares two independent reviews.

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow the dual-verification skill exactly as written.

Path: `${CLAUDE_PLUGIN_ROOT}skills/dual-verification/SKILL.md`

Tool: `Skill(skill: "cipherpowers:dual-verification")`

Do NOT proceed without completing skill activation.

## MANDATORY: Context

Read before starting:
- @README.md
- @CLAUDE.md

## Save Workflow

Use template: `${CLAUDE_PLUGIN_ROOT}templates/verify-collation-template.md`

Save to: `.work/{YYYY-MM-DD}-verify-{type}-collated-{HHmmss}.md`

Announce file path in final response.

</instructions>
```

**Impact:** Reduce from 83 lines to ~30 lines (64% reduction)

### 2. ultrathink-debugger.md (115 lines) ❌

**Issue Type:** OVER-ENGINEERED

**Current State:**
- 115 lines (362% of exemplar average)
- Contains "Announcement" section with exact text
- Contains "Specialization: When to Use This Agent" section
- Contains "Complex Scenario Techniques" section with detailed procedures
- Contains "Red Flags - Return to Skill" table
- Contains "Completion Criteria" checklist
- Contains "Purpose" and "Behavioral Traits" sections outside `<instructions>`

**Expected State (from exemplars):**
- ~30-40 lines
- Pure delegation to systematic-debugging skill
- No workflow logic in agent
- No detailed techniques
- No red flags table

**Violations:**

1. **Announcement template in agent:**
   ```markdown
   ## Announcement

   IMMEDIATELY announce:
   ```
   I'm using the ultrathink-debugger agent for complex debugging.
   ...
   ```
   ```
   → This belongs in skill workflow

2. **Specialization guidance in agent:**
   ```markdown
   ## Specialization: When to Use This Agent

   **Use ultrathink-debugger for:**
   - Multi-component failures...
   - Environment-specific issues...
   ...
   ```
   → This belongs in skill `when_to_use` frontmatter

3. **Detailed techniques in agent:**
   ```markdown
   ## Complex Scenario Techniques

   ### Multi-Component Systems
   - Add diagnostic logging at EACH component boundary
   - Log what enters and exits each layer
   ...
   ```
   → This belongs in skill detailed procedures

4. **Red flags table in agent:**
   ```markdown
   ## Red Flags - Return to Skill

   | Excuse | Reality |
   ...
   ```
   → Skills handle rationalization defenses

5. **Completion criteria in agent:**
   ```markdown
   ## Completion Criteria

   NOT complete until:
   - [ ] Root cause identified
   ...
   ```
   → This belongs in skill workflow

6. **Purpose/Behavioral Traits outside `<instructions>`:**
   - Duplicates skill content
   - Agents should be thin wrappers

**Recommendation:**

Extract to skill(s):
- Announcement template → `systematic-debugging` skill opening section
- Specialization guidance → skill `when_to_use` frontmatter
- Complex scenario techniques → skill detailed procedures
- Red flags → skill enforcement section
- Completion criteria → skill workflow steps
- Behavioral traits → skill description

Reduce agent to:
```markdown
---
name: ultrathink-debugger
description: Complex debugging specialist for multi-component systems, production issues, and mysterious behavior
color: red
---

You are an ultrathink expert debugging specialist for complex, multi-layered software problems.

<instructions>
## Instructions

## MANDATORY: Skill Activation

Use and follow these debugging skills:

**Primary:**
- Systematic Debugging: `${CLAUDE_PLUGIN_ROOT}skills/systematic-debugging/SKILL.md`
- Tool: `Skill(skill: "cipherpowers:systematic-debugging")`

**Supporting (when applicable):**
- Root Cause Tracing: `${CLAUDE_PLUGIN_ROOT}skills/root-cause-tracing/SKILL.md`
- Defense in Depth: `${CLAUDE_PLUGIN_ROOT}skills/defense-in-depth/SKILL.md`

Do NOT proceed without activating systematic-debugging skill.

## MANDATORY: Context

Read before starting:
- @README.md
- @CLAUDE.md

## MANDATORY: Standards

- ${CLAUDE_PLUGIN_ROOT}principles/testing.md
- ${CLAUDE_PLUGIN_ROOT}principles/development.md

</instructions>
```

**Impact:** Reduce from 115 lines to ~40 lines (65% reduction)

### 3. path-test-agent.md (67 lines) ⚠️

**Issue Type:** SPECIAL PURPOSE (Not Production)

**Current State:**
- 67 lines
- Test agent for verifying path resolution
- Contains detailed test procedure
- Does NOT follow thin delegation pattern

**Assessment:**
- This is a **test utility**, not a production agent
- Purpose is to verify plugin infrastructure, not perform production work
- Special-purpose agents may need different patterns

**Recommendation:**
- **ACCEPTABLE as-is** - test utilities have different requirements
- Consider moving to `plugin/tests/` directory if it exists
- Add comment to clarify this is test infrastructure, not production

**No action required** - special purpose justified

## Content for Skill Extraction

### From review-collation-agent.md

**Extract to `dual-verification` skill:**

1. **Non-negotiable workflow steps:**
   - Read both reviews completely before starting
   - Use template exactly (no custom sections)
   - Mark exclusive issues as "pending cross-check"
   - Save report to .work/ directory
   - Announce revise command availability

2. **Final message format template:**
   - Executive summary structure
   - Status classification
   - Next steps guidance

3. **Rationalization defenses (red flags):**
   - "Reviews mostly agree, skip comparison" → Compare systematically
   - "Exclusive issue probably wrong" → Present with MODERATE confidence
   - "Template is too simple" → Use template exactly
   - "I should add analysis" → Collation not review

4. **Collation methodology:**
   - Common issues → VERY HIGH confidence
   - Exclusive issues → MODERATE confidence (pending cross-check)
   - Divergence resolution process

### From ultrathink-debugger.md

**Extract to `systematic-debugging` skill:**

1. **Announcement template:**
   - Opening statement
   - Four-phase framework overview
   - Process commitment

2. **Specialization guidance (when_to_use):**
   - Multi-component failures (3+ layers)
   - Environment-specific issues (local vs production)
   - Timing/concurrency issues
   - Integration failures
   - Production emergencies

3. **Complex scenario techniques:**
   - Multi-component diagnostics (logging at boundaries)
   - Environment comparison procedures
   - Timing/concurrency investigation
   - Integration failure debugging

4. **Rationalization defenses (red flags):**
   - "I see the issue, skip process" → Complex bugs deceive
   - "Fix where error appears" → Symptom ≠ root cause
   - "Should work now" → No claims without verification
   - "No time for process" → Systematic is faster

5. **Completion criteria:**
   - Root cause identified (not symptoms)
   - Fix addresses root cause
   - Verification command with evidence
   - No regression

**Create supporting skills:**
- `root-cause-tracing` skill (if not exists)
- `defense-in-depth` skill (if not exists)

## Summary

**Total agents reviewed:** 14

**Compliance breakdown:**
- **Compliant (exemplar-level):** 11 agents (79%)
  - commit-agent (28 lines) ✅
  - code-exec-agent (29 lines) ✅
  - code-agent (31 lines) ✅
  - code-review-agent (32 lines) ✅
  - gatekeeper (34 lines) ✅
  - plan-review-agent (35 lines) ✅
  - rust-exec-agent (35 lines) ✅
  - research-agent (37 lines) ✅
  - rust-agent (37 lines) ✅
  - execute-review-agent (40 lines) ✅
  - technical-writer (45 lines) ✅

- **Special Purpose:** 1 agent (7%)
  - path-test-agent (67 lines) ⚠️ - Test utility, acceptable

- **Non-Compliant (over-engineered):** 2 agents (14%)
  - review-collation-agent (83 lines) ❌
  - ultrathink-debugger (115 lines) ❌

**Content to extract:**
- 2 major extractions (review-collation → dual-verification skill, ultrathink-debugger → systematic-debugging skill)
- Total lines to reduce: 83 + 115 = 198 lines
- Target reduction: ~128 lines (65% reduction to ~70 lines combined)

**Key Findings:**

1. **79% compliance rate** - Excellent adoption of thin skill-delegation pattern
2. **Two outliers** - Both contain workflow logic that belongs in skills
3. **Pattern is clear** - Exemplars at 28-37 lines set strong standard
4. **Technical-writer justified extension** - Mode detection adds value without bloating
5. **No structural violations** - All agents use `<instructions>` wrapper correctly

**Priority Actions:**

1. **HIGH:** Extract review-collation-agent workflow to dual-verification skill
2. **HIGH:** Extract ultrathink-debugger workflow to systematic-debugging skill
3. **LOW:** Consider moving path-test-agent to test directory (optional)

**Success Metrics:**
- 11 agents already comply with exemplar pattern (79%)
- 2 agents need refactoring (14%)
- 1 test agent acceptable as-is (7%)
- Overall architecture is strong with clear outliers

## Additional Observations

### Positive Patterns

1. **Consistent YAML frontmatter** - All agents use proper YAML format
2. **Skill activation syntax** - All use Path + Tool pattern correctly
3. **Standards references** - All use `${CLAUDE_PLUGIN_ROOT}` correctly
4. **Save workflows** - Verification agents follow timestamp pattern
5. **Context requirements** - All require README.md and CLAUDE.md

### Architectural Strengths

1. **Clear separation** - 11 agents demonstrate pure delegation
2. **Skill-centric** - Agents reference skills, don't duplicate
3. **Concise** - Average compliant agent: 35 lines
4. **Maintainable** - Single source of truth in skills

### Recommendations for Future Agents

1. **Target 30-40 lines** - Exemplar range
2. **No workflow logic** - Delegate to skills
3. **No red flags** - Skills handle enforcement
4. **No detailed procedures** - Skills provide steps
5. **Minimal context** - Just activation + references
6. **Save workflow only** - For verification agents

### Template Adherence

All agents follow the structure from `plugin/templates/agent-template.md`, but two have added significant content beyond the template. The template should be sufficient for all production agents.
