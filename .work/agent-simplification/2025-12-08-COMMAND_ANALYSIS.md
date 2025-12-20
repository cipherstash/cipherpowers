# Command Analysis: Retention vs Migration vs New Skills

Analysis of all commands in `/plugin/commands/` to identify:
1. Content that should be RETAINED (unique instructions not in skill)
2. Content that should be MOVED to existing skill
3. Content that needs NEW skill created

Reference pattern (from brainstorm.md and code-review.md):
- Frontmatter with description + argument-hint (if args)
- Brief description
- Usage section with positional args (if args)
- Instructions section referencing skill

---

## SUMMARY

| Command | Lines | Status | Action | Priority |
|---------|-------|--------|--------|----------|
| commit.md | 82 | MINIMIZABLE | Can minimize now | HIGH |
| execute.md | 36 | GOOD | Keep as-is | - |
| plan.md | 32 | GOOD | Keep as-is | - |
| revise.md | 69 | MINIMIZABLE | Can minimize now | HIGH |
| summarise.md | 31 | GOOD | Keep as-is | - |
| test-paths.md | 41 | REFERENCE ONLY | Agent reference, keep as-is | - |
| verify.md | 36 | MINIMIZED | Already correct model | - |

---

## DETAILED ANALYSIS

### 1. commit.md (82 lines)

**Current line count:** 82

**Frontmatter:** ✅ CORRECT
```yaml
description: Systematic git commit with atomic commits and conventional messages
```

**Content Structure:**
- Lines 1-16: Frontmatter + title + brief description ✅
- Lines 17-32: MANDATORY skill activation section
- Lines 35-81: Algorithmic dispatch + agent info

**Assessment:**

**DUPLICATE CONTENT (lines 17-32):**
- "MANDATORY: Skill Activation" section duplicates skill activation instructions
- Skill (commit-workflow/SKILL.md) already has all this information
- Redundant explanation of skill activation workflow
- **Can be removed:** Yes, this is boilerplate that appears in other commands

**UNIQUE CONTENT (lines 35-81):**
- Algorithmic dispatch decision tree (lines 37-68)
- Agent dispatch instructions with Task tool syntax
- Explanation of why algorithmic dispatch (lines 62-67)
- What the agent does summary (lines 69-81)
- **Cannot be removed:** This is command-specific context about dispatching to commit-agent

**Recommended Action:** CAN MINIMIZE NOW

**Proposed minimal version:**
```markdown
---
description: Systematic git commit with atomic commits and conventional messages
---

# Commit

Systematic git commit with atomic commits and conventional messages.

## Usage

/cipherpowers:commit

No arguments required.

## Instructions

Activate the commit-workflow skill:

Skill(skill: "cipherpowers:commit-workflow")

Follow the skill exactly.

After skill completion, dispatch to commit-agent:

Use Task tool with:
  subagent_type: "cipherpowers:commit-agent"
  description: "Commit workflow"
  prompt: "[User's original request]"
```

**Savings:** ~50 lines (39% reduction)

---

### 2. execute.md (36 lines)

**Current line count:** 36

**Frontmatter:** ✅ CORRECT
```yaml
description: Execute implementation plans in batches with specialised agents
argument-hint: [plan-file] [model]
```

**Content Structure:**
- Lines 1-4: Frontmatter ✅
- Lines 6-9: Title + description ✅
- Lines 11-17: Usage with arguments ✅
- Lines 19-29: Agent defaults table (UNIQUE) ✅
- Lines 31-36: Workflow reference to skill ✅

**Assessment:**

**UNIQUE CONTENT:**
- Agent defaults table (lines 19-29): Maps agents to model defaults
  - This information is NOT in the executing-plans skill
  - This is command-specific context about `/cipherpowers:execute` dispatch
  - **Must be retained:** Yes, essential for understanding model override behavior

**SKILL REFERENCE:**
- Workflow section (lines 31-36) correctly references skill
- Minimal, focused instruction

**Recommended Action:** KEEP AS-IS

This command is already minimized and follows the reference pattern perfectly. The agent defaults table is unique command context that users need.

---

### 3. plan.md (32 lines)

**Current line count:** 32

**Frontmatter:** ✅ CORRECT
```yaml
description: Create detailed implementation plans with bite-sized tasks
```

**Content Structure:**
- Lines 1-4: Frontmatter ✅
- Lines 6-8: Title + description ✅
- Lines 10-15: Usage ✅
- Lines 17-28: Instructions with skill activation ✅

**Assessment:**

**CONTENT QUALITY:**
- Brief, focused instruction
- Direct skill activation
- No duplicate content
- Follows reference pattern

**Recommended Action:** KEEP AS-IS

This command is already minimal and correct.

---

### 4. revise.md (69 lines)

**Current line count:** 69

**Frontmatter:** ✅ CORRECT
```yaml
description: Implement findings from verification reports
argument-hint: [scope] [collation-file]
```

**Content Structure:**
- Lines 1-8: Frontmatter + title + description + core principle ✅
- Lines 10-20: Usage with arguments ✅
- Lines 22-53: Algorithmic workflow (DUPLICATE)
- Lines 55-62: Related commands/skills ✅
- Lines 64-69: Remember section ✅

**Assessment:**

**DUPLICATE CONTENT (lines 22-53):**
- Lines 22-27: Decision tree step 1 (trivial)
- Lines 29-44: "MANDATORY: Skill Activation" section
  - This is identical boilerplate to commit.md
  - Duplicates skill activation instructions already in revising-findings skill
  - Same "@${CLAUDE_PLUGIN_ROOT}skills/revising-findings/SKILL.md" reference

- Lines 45-53: "FOLLOW THE SKILL EXACTLY" section
  - Just bullet points summarizing skill content
  - Duplicates skill workflow overview
  - **Can be removed:** Yes, skill already has this

**UNIQUE CONTENT:**
- Lines 10-20: Usage with scope explanations (UNIQUE)
  - Three scopes (common, exclusive, all) with different behaviors
  - Different prerequisites
  - Cannot be in skill (command-specific argument pattern)

- Lines 55-62: Related commands/skills + Remember (MOSTLY UNIQUE)
  - Related commands section references `/verify` (command-specific)
  - Remember section has practical tips on scope availability

**Recommended Action:** CAN MINIMIZE NOW

**Proposed minimal version:**
```markdown
---
description: Implement findings from verification reports
argument-hint: [scope] [collation-file]
---

# Revise

Implement findings from verification reports. Works with collation reports produced by `/verify`.

**Core principle:** Separate what to fix (verify) from how to fix it (revise).

## Usage

/cipherpowers:revise [scope] [collation-file]

- `$1` - scope: `common`, `exclusive`, `all` (default: all)
- `$2` - collation file path (optional)

| Scope | What's Implemented | When Available |
|-------|-------------------|----------------|
| `common` | Issues both reviewers found (VERY HIGH confidence) | Immediately after collation |
| `exclusive` | VALIDATED exclusive issues only | After cross-check completes |
| `all` | Common + VALIDATED exclusive | After cross-check completes |

## Instructions

Activate the revising-findings skill:

Skill(skill: "cipherpowers:revising-findings")

Follow the skill exactly. It handles:
- Locating collation report
- Checking cross-check status (for exclusive/all)
- Building implementation list
- Handling UNCERTAIN issues
- Dispatching implementation agents
- Verifying implementation

## Related Commands

- `/cipherpowers:verify` - Generate collation reports (prerequisite)

## Remember

- `/revise common` works immediately after collation
- `/revise exclusive` requires cross-check complete
- Skill contains detailed workflow - follow it exactly
```

**Savings:** ~35 lines (51% reduction)

---

### 5. summarise.md (31 lines)

**Current line count:** 31

**Frontmatter:** ✅ CORRECT
```yaml
description: Retrospective summary capturing decisions and lessons learned
```

**Content Structure:**
- Lines 1-4: Frontmatter ✅
- Lines 6-8: Title + description ✅
- Lines 10-15: Usage ✅
- Lines 17-31: Instructions ✅

**Assessment:**

**CONTENT QUALITY:**
- Minimal, focused instruction
- Clear skill activation
- Explains what skill provides (Step 1-3)
- Unique principle about "exhaustion after completion" (line 30)
- No duplicate content

**Recommended Action:** KEEP AS-IS

This command is already minimal and includes important context about when/why to use the skill.

---

### 6. test-paths.md (41 lines)

**Current line count:** 41

**Frontmatter:** ✅ CORRECT
```yaml
description: Test file path resolution in plugin agents
```

**Content Structure:**
- Lines 1-4: Frontmatter ✅
- Lines 6-8: Title + description ✅
- Lines 10-15: Test scenarios ✅
- Lines 17-26: Execution instructions ✅
- Lines 29-34: Expected outcome ✅

**Assessment:**

**PURPOSE:** This is a special command that tests file path resolution. It doesn't reference a skill because it IS a reference to test-agent behavior.

**UNIQUE CONTENT:**
- Test scenarios explanation (lines 10-15)
- Task tool dispatch syntax to path-test-agent
- Expected outcome analysis

**Note:** This command is not minimizable because:
1. It's a testing/diagnostic command, not a workflow command
2. It has no corresponding skill (it tests agent behavior)
3. All content is necessary for running the test

**Recommended Action:** KEEP AS-IS

---

### 7. verify.md (36 lines)

**Current line count:** 36

**Frontmatter:** ✅ CORRECT
```yaml
description: Dual-verification dispatcher for high-confidence verification
argument-hint: <type> [scope] [model]
```

**Content Structure:**
- Lines 1-4: Frontmatter ✅
- Lines 6-9: Title + brief ✅
- Lines 11-18: Usage with arguments ✅
- Lines 20-24: Minimal workflow reference ✅
- Lines 26-35: Dispatch defaults table ✅

**Assessment:**

**ALREADY MINIMIZED:** ✅
This command is already an ideal reference implementation:
- Minimal text (only essential context)
- Clear argument documentation
- Unique dispatch defaults table (command-specific)
- Direct skill reference
- Follows the reference pattern perfectly

**Recommended Action:** KEEP AS-IS (REFERENCE IMPLEMENTATION)

---

## RECOMMENDATIONS SUMMARY

### Immediate Actions (HIGH PRIORITY)

**1. Minimize commit.md** (82→32 lines)
- Remove redundant skill activation boilerplate
- Keep algorithmic dispatch instructions
- Move agent workflow explanation to agent context
- Save ~50 lines (39% reduction)

**2. Minimize revise.md** (69→34 lines)
- Remove duplicate "MANDATORY: Skill Activation" section
- Remove redundant "FOLLOW THE SKILL EXACTLY" bullet points
- Keep unique scope explanation table
- Keep related commands section
- Save ~35 lines (51% reduction)

### Reference Pattern Compliance

**Commands that ALREADY COMPLY:**
- execute.md ✅ (with unique agent defaults table)
- plan.md ✅ (minimal style)
- summarise.md ✅ (minimal style)
- verify.md ✅ (REFERENCE IMPLEMENTATION - use as template)

**Commands that NEED UPDATES:**
- commit.md (remove lines 17-32, 62-67)
- revise.md (remove lines 22-27, 29-53)

**Commands that DON'T APPLY:**
- test-paths.md (testing/diagnostic command, no skill)

### Pattern for Minimized Commands

All minimized commands should follow this pattern:

```markdown
---
description: [One line description]
argument-hint: [args] (if applicable)
---

# [Title]

[One sentence summary]

## Usage

/cipherpowers:[command] [args if any]

[Argument definitions if needed]

## Instructions

[Minimal reference to skill OR unique command context]

## [Optional unique sections]

[Tables, related commands, remember notes, agent defaults, etc.]
```

### Skills Assessment

No NEW skills needed. All commands reference appropriate existing skills:
- commit.md → commit-workflow ✅
- execute.md → executing-plans ✅
- plan.md → writing-plans ✅
- revise.md → revising-findings ✅
- summarise.md → capturing-learning ✅
- verify.md → dual-verification ✅

### Content Migration Opportunities

**Boilerplate to extract (appears in multiple commands):**

Currently duplicated in commit.md and revise.md:
- "MANDATORY: Skill Activation" section template
- Skill path reference pattern
- Step-by-step evaluation format

**Note:** This boilerplate should stay WHERE IT IS (in commands) because:
1. Commands are thin dispatchers - they should include activation instructions
2. Agents include activation in their prompts (different context)
3. Duplication here is intentional for clarity at command level
4. Removing it would make commands harder to use

The minimization is about REMOVING repetition within a single command, not about eliminating skill activation instructions entirely.
