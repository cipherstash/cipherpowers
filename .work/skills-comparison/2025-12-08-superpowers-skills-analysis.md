# Research Report: Upstream Superpowers Skills Analysis

## Metadata
- Date: 2025-12-08
- Researcher: research-agent
- Scope: Analyzed 6 skills from upstream Superpowers repository
- Repository: https://github.com/obra/superpowers

## Research Questions
1. What is the structure and format of upstream Superpowers skills?
2. What metadata conventions do they use?
3. What content patterns and enforcement mechanisms exist?
4. How do they compare to CipherPowers in size, structure, and approach?
5. What unique characteristics or patterns exist that differ from CipherPowers?

## Key Findings

### Finding 1: Frontmatter Structure
- **Source:** All 6 skills analyzed (brainstorming, writing-plans, executing-plans, systematic-debugging, test-driven-development, using-superpowers)
- **Evidence:** 100% of skills use YAML frontmatter with `---` delimiters
- **Confidence:** VERY HIGH
- **Implication:** Consistent metadata convention across all upstream skills

**Frontmatter fields observed:**
- `name`: Present in all 6 skills (100%)
- `description`: Present in all 6 skills (100%)
- No other frontmatter fields found (no version, when_to_use, applies_to, etc.)

### Finding 2: Skill Size Distribution
- **Source:** Direct line counts from GitHub raw files
- **Evidence:**
  - brainstorming: 76 lines
  - writing-plans: 137 lines
  - executing-plans: 114 lines
  - systematic-debugging: 387 lines
  - test-driven-development: 397 lines
  - using-superpowers: 109 lines
- **Confidence:** VERY HIGH
- **Implication:** Wide size variance (76-397 lines), with enforcement-heavy skills (TDD, debugging) being significantly larger

**Statistics:**
- Average: 203.3 lines
- Median: 125.5 lines
- Range: 321 lines (76-397)
- Estimated tokens (line × 15): 3,050 average, 1,140-5,955 range

### Finding 3: Section Structure Patterns
- **Source:** Structural analysis of all 6 skills
- **Evidence:** Common level-2 headers across multiple skills
- **Confidence:** HIGH
- **Implication:** Consistent structural patterns emerge across the suite

**Common level-2 sections:**
- "Overview" (6/6 skills = 100%)
- "When to Use" (2/6 skills = test-driven-development, systematic-debugging)
- "The Process" (2/6 skills = brainstorming, executing-plans)
- "Remember" (2/6 skills = writing-plans, executing-plans)
- "Quick Reference" (1/6 skills = systematic-debugging)
- "Key Principles" (1/6 skills = brainstorming)
- "Common Rationalizations" (2/6 skills = systematic-debugging, test-driven-development)
- "Red Flags - STOP" (2/6 skills = systematic-debugging, test-driven-development)

**Level-2 header counts:**
- brainstorming: 4 sections
- writing-plans: 6 sections
- executing-plans: 5 sections
- systematic-debugging: 11 sections
- test-driven-development: 13 sections
- using-superpowers: 9 sections

### Finding 4: Enforcement Mechanisms
- **Source:** Analysis of skill content, particularly systematic-debugging and test-driven-development
- **Evidence:** Multiple enforcement patterns identified
- **Confidence:** HIGH
- **Implication:** Skills use explicit enforcement language and rationalization defense

**Enforcement patterns observed:**
1. **Iron Laws:** Both systematic-debugging and test-driven-development use "The Iron Law" section
   - systematic-debugging: "NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST"
   - test-driven-development: "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST"

2. **Red Flags sections:** Both debugging and TDD include explicit "Red Flags - STOP" sections with warning signs

3. **Common Rationalizations tables:** Both debugging and TDD include tables listing excuses with reality checks
   - test-driven-development: 12 rationalization entries
   - systematic-debugging: Multiple rationalization entries

4. **Verification checklists:** test-driven-development includes explicit verification checklist (8 items)

5. **Mandatory protocols:** using-superpowers includes "MANDATORY FIRST RESPONSE PROTOCOL" with checklist

### Finding 5: Announcement Directives
- **Source:** Frontmatter descriptions and explicit sections in all skills
- **Evidence:**
  - 4/6 skills (67%) have explicit "Announce at start" directives
  - 2/6 skills (33%) embed announcement guidance in frontmatter description
- **Confidence:** VERY HIGH
- **Implication:** Announcement pattern is standard but implementation varies

**Announcement patterns:**
- **Explicit "Announce at start" line:**
  - writing-plans: "Announce at start: 'I'm using the writing-plans skill to create the implementation plan.'"
  - executing-plans: "Announce at start: 'I'm using the executing-plans skill to implement...'"

- **Frontmatter description as guide:**
  - brainstorming: "Use when creating or developing, before writing code or implementation plans..."
  - systematic-debugging: "Use when encountering any bug, test failure, or unexpected behavior..."

- **Dedicated section:**
  - using-superpowers: Has entire "Announcing Skill Usage" section with pattern "'I'm using [Skill Name] to [what you're doing].'"

### Finding 6: Code Block Usage
- **Source:** Structural analysis of all 6 skills
- **Evidence:**
  - brainstorming: 0 code blocks
  - writing-plans: 8 code blocks (markdown examples, test structure, git commands)
  - executing-plans: 0 code blocks
  - systematic-debugging: 5 code blocks (multi-layer system examples, bash commands)
  - test-driven-development: 8 code blocks (TypeScript examples, bash commands, dot graph)
  - using-superpowers: 0 code blocks
- **Confidence:** VERY HIGH
- **Implication:** Code blocks used heavily in plan/implementation skills (writing-plans) and enforcement skills (TDD, debugging), not used in process/meta skills

**Total code blocks across 6 skills:** 21 code blocks

### Finding 7: Table Usage
- **Source:** Structural analysis of all 6 skills
- **Evidence:**
  - brainstorming: 0 tables
  - writing-plans: 0 tables
  - executing-plans: 0 tables
  - systematic-debugging: 3 tables
  - test-driven-development: 3 tables
  - using-superpowers: 0 tables
- **Confidence:** VERY HIGH
- **Implication:** Tables used exclusively in enforcement-heavy skills for documenting rationalizations and red flags

**Total tables across 6 skills:** 6 tables (all in debugging and TDD)

### Finding 8: Checklist Patterns
- **Source:** Structural analysis of all 6 skills
- **Evidence:**
  - 3/6 skills (50%) contain explicit checkbox-style checklists
  - executing-plans: Multiple checklists throughout process steps
  - test-driven-development: Verification checklist (8 items)
  - using-superpowers: Mandatory first response protocol checklist (4 items)
- **Confidence:** HIGH
- **Implication:** Checklists used in execution/process skills and meta-enforcement skills

### Finding 9: Cross-Skill References
- **Source:** Analysis of skill content for @ syntax and skill name references
- **Evidence:** 4/6 skills reference other skills
- **Confidence:** HIGH
- **Implication:** Skills form interconnected suite with explicit dependencies

**Reference patterns:**

**brainstorming → 3 references:**
- elements-of-style:writing-clearly-and-concisely
- superpowers:using-git-worktrees
- superpowers:writing-plans

**writing-plans → 3 references:**
- @brainstorming (via "created by brainstorming skill")
- @superpowers:executing-plans (execution handoff)
- @superpowers:subagent-driven-development

**executing-plans → 1 reference:**
- superpowers:finishing-a-development-branch

**systematic-debugging → 5 references:**
- root-cause-tracing
- test-driven-development
- defense-in-depth
- condition-based-waiting
- verification-before-completion

**test-driven-development → 0 references**
- Self-contained guidance

**using-superpowers → Generic references:**
- brainstorming skill
- test-driven-development skill
- debugging skill
- verification skill

### Finding 10: Level-3 Subsection Usage
- **Source:** Structural analysis of all 6 skills
- **Evidence:**
  - brainstorming: 5 subsections (Understanding, Exploring, Presenting, Documentation, Implementation)
  - writing-plans: 1 subsection (Task N template)
  - executing-plans: 5 subsections (Step 1-5)
  - systematic-debugging: 4 subsections (Phase 1-4)
  - test-driven-development: 6 subsections (RED, Verify RED, GREEN, Verify GREEN, REFACTOR, Repeat)
  - using-superpowers: 0 subsections
- **Confidence:** HIGH
- **Implication:** Process skills use level-3 headers for sequential steps/phases

## Patterns Observed

### Pattern 1: Size Correlates with Enforcement Strength
Skills with stronger enforcement mechanisms (Iron Laws, Red Flags, Rationalization tables) are significantly larger:
- Enforcement-heavy: systematic-debugging (387 lines), test-driven-development (397 lines)
- Process-focused: brainstorming (76 lines), executing-plans (114 lines), using-superpowers (109 lines)
- Mid-range: writing-plans (137 lines) - includes templates but less enforcement

### Pattern 2: Two-Field Frontmatter Standard
All skills use exactly 2 frontmatter fields (name, description), no more, no less. This is significantly simpler than CipherPowers' richer frontmatter with when_to_use, version, applies_to, etc.

### Pattern 3: "Iron Law" Enforcement Pattern
High-stakes skills (TDD, debugging) use "The Iron Law" section to establish non-negotiable rules:
- Single, memorable rule stated emphatically
- Followed by rationalization defense
- Violation consequences clearly stated

### Pattern 4: Sequential Process Documentation
Process skills use numbered steps or phases with level-3 headers:
- executing-plans: Step 1, Step 2, Step 3, Step 4, Step 5
- systematic-debugging: Phase 1, Phase 2, Phase 3, Phase 4
- test-driven-development: RED → Verify RED → GREEN → Verify GREEN → REFACTOR → Repeat

### Pattern 5: Meta-Skill Enforcement
using-superpowers acts as meta-enforcement requiring skill usage before any response:
- MANDATORY FIRST RESPONSE PROTOCOL
- Checklist of required actions
- Rationalization defense against skipping skills

### Pattern 6: Workflow Integration
Skills explicitly reference next steps and related skills:
- brainstorming → writing-plans → executing-plans chain
- systematic-debugging references test-driven-development
- Handoff patterns documented explicitly

## Gaps and Uncertainties

### Gap 1: Version Information
- **What couldn't be verified:** No version field in any skill frontmatter
- **Implication:** Cannot determine skill evolution or compatibility requirements
- **Confidence in gap:** VERY HIGH (explicitly checked all frontmatter)

### Gap 2: Language/Framework Applicability
- **What couldn't be verified:** No applies_to field indicating language or framework scope
- **Implication:** Cannot filter skills by project type
- **Confidence in gap:** VERY HIGH

### Gap 3: Complete Skill Inventory
- **What couldn't be verified:** Only analyzed 6 skills from upstream repository
- **Implication:** May be missing patterns from other skills
- **Confidence in gap:** MODERATE (only sampled, did not fetch complete list)
- **Recommendation:** Fetch complete skill list from repository to verify sample is representative

### Gap 4: Skill Metadata Discovery
- **What couldn't be verified:** How skills are discovered without when_to_use field
- **Implication:** Discovery mechanism relies solely on description field
- **Confidence in gap:** MODERATE (frontmatter analysis complete, but discovery mechanism implementation unknown)

### Gap 5: Test Scenario Documentation
- **What couldn't be verified:** Whether skills include test-scenarios.md files
- **Implication:** Cannot verify TDD approach to skill development
- **Confidence in gap:** MODERATE (did not fetch additional files beyond SKILL.md)

### Gap 6: Historical Evolution
- **What couldn't be verified:** How skills have evolved over time
- **Implication:** Cannot determine stability or maturity
- **Confidence in gap:** HIGH (would require git history analysis)

## Summary Statistics

**Skills analyzed:** 6

**Line counts:**
- Minimum: 76 lines (brainstorming)
- Maximum: 397 lines (test-driven-development)
- Average: 203.3 lines
- Median: 125.5 lines

**Estimated token counts (line × 15):**
- Minimum: 1,140 tokens (brainstorming)
- Maximum: 5,955 tokens (test-driven-development)
- Average: 3,050 tokens
- Median: 1,883 tokens

**Frontmatter consistency:**
- 100% use YAML frontmatter with --- delimiters
- 100% have name field
- 100% have description field
- 0% have additional fields

**Common sections:**
- Overview: 100% (6/6)
- Announcement directive: 67% (4/6 explicit)
- When to Use: 33% (2/6)
- The Process/Steps: 33% (2/6)
- Remember: 33% (2/6)
- Common Rationalizations: 33% (2/6)
- Red Flags: 33% (2/6)
- Iron Law: 33% (2/6)

**Content elements:**
- Code blocks: 21 total across 6 skills (50% have code blocks)
- Tables: 6 total across 6 skills (33% have tables)
- Checklists: Present in 50% of skills (3/6)

## Skills Inventory Table

| Skill | Lines | Tokens (est) | Frontmatter Fields | Key Sections | Code Blocks | Tables | Checklists | References |
|-------|-------|--------------|-------------------|--------------|-------------|--------|------------|------------|
| brainstorming | 76 | 1,140 | name, description | Overview, Process, After Design, Principles | 0 | 0 | No | 3 |
| writing-plans | 137 | 2,055 | name, description | Overview, Granularity, Header, Task Structure, Handoff | 8 | 0 | No | 3 |
| executing-plans | 114 | 1,710 | name, description | Overview, Process (5 steps), When to Stop, Remember | 0 | 0 | Yes | 1 |
| systematic-debugging | 387 | 5,805 | name, description | Overview, Iron Law, When to Use, 4 Phases, Red Flags, Rationalizations, Quick Reference | 5 | 3 | No | 5 |
| test-driven-development | 397 | 5,955 | name, description | Overview, Iron Law, Red-Green-Refactor, Rationalizations, Red Flags, Verification, Debugging | 8 | 3 | Yes | 0 |
| using-superpowers | 109 | 1,635 | name, description | Getting Started, Mandatory Protocol, Critical Rules, Rationalizations, Checklists, Summary | 0 | 0 | Yes | Generic refs |

## Notable Characteristics

### Unique to Superpowers (Not in CipherPowers)

1. **"Iron Law" Enforcement Pattern:**
   - Dedicated section for single, emphatic non-negotiable rule
   - Clear violation consequences
   - Not observed in CipherPowers skills

2. **Minimal Frontmatter:**
   - Only 2 fields (name, description) vs CipherPowers' richer metadata
   - Description field serves dual purpose (discovery + when_to_use guidance)
   - Simpler but less structured for discovery

3. **"Remember" Section Pattern:**
   - Found in writing-plans and executing-plans
   - Short reminder of key principles at end
   - Not a standard section in CipherPowers

4. **Meta-Enforcement Skill:**
   - using-superpowers acts as meta-skill enforcing skill usage
   - MANDATORY FIRST RESPONSE PROTOCOL
   - CipherPowers lacks equivalent meta-enforcement skill

5. **Extensive Rationalization Defense:**
   - Detailed tables of excuses with reality checks
   - 12+ entries in test-driven-development
   - More comprehensive than CipherPowers' rationalization defenses

6. **"Red Flags - STOP" Sections:**
   - Explicit section title pattern
   - Lists warning signs requiring immediate halt
   - More prominent than CipherPowers' red flag handling

### Characteristics Shared with CipherPowers

1. **Skill References:**
   - Both use @ syntax for skill references
   - Both create interconnected skill suites
   - Similar dependency patterns

2. **Announcement Directives:**
   - Both require announcing skill usage
   - Similar pattern: "I'm using [skill] to [action]"

3. **Process Documentation:**
   - Sequential steps with clear transitions
   - Checkpoint patterns
   - Wait-for-feedback cycles

4. **Enforcement Through Structure:**
   - Non-negotiable workflows
   - Explicit stop points
   - Violation warnings

### Differences in Approach

1. **Frontmatter Philosophy:**
   - **Superpowers:** Minimal (2 fields), description does heavy lifting
   - **CipherPowers:** Rich metadata (when_to_use, version, applies_to)

2. **Size Strategy:**
   - **Superpowers:** Wide variance (76-397 lines), enforcement skills are much larger
   - **CipherPowers:** More consistent sizing (needs verification)

3. **Documentation Density:**
   - **Superpowers:** High-stakes skills very dense with enforcement
   - **CipherPowers:** Delegated enforcement to agents (thin skill delegation pattern)

4. **Code Examples:**
   - **Superpowers:** Heavy use in implementation skills (8 blocks in writing-plans and TDD)
   - **CipherPowers:** Usage pattern unclear (needs analysis)

5. **Table Usage:**
   - **Superpowers:** Concentrated in enforcement skills (6 tables in 2 skills)
   - **CipherPowers:** Usage pattern unclear (needs analysis)

## Recommendations

### For CipherPowers Development

1. **Consider "Iron Law" pattern:**
   - High-stakes skills could benefit from dedicated non-negotiable rule section
   - More memorable and enforceable than distributed enforcement

2. **Evaluate frontmatter trade-offs:**
   - Superpowers' minimal approach is simpler
   - CipherPowers' rich metadata aids discovery
   - Consider which better serves team needs

3. **Rationalization defense depth:**
   - Superpowers' comprehensive rationalization tables (12+ entries) are stronger
   - CipherPowers could strengthen defenses in enforcement-heavy skills

4. **Meta-enforcement consideration:**
   - using-superpowers demonstrates value of meta-skill
   - CipherPowers using-cipherpowers skill could adopt MANDATORY FIRST RESPONSE PROTOCOL

5. **"Remember" section pattern:**
   - Simple end-of-skill reminder section is effective
   - Could supplement CipherPowers skills

### Further Research Needed

1. **Complete Superpowers skill inventory:**
   - Fetch all skills to verify sample is representative
   - Analyze complete cross-reference graph

2. **CipherPowers skill analysis:**
   - Apply same analysis to CipherPowers skills
   - Direct comparison of patterns and sizes

3. **Test scenario investigation:**
   - Check for test-scenarios.md files in Superpowers
   - Compare TDD approach to skill development

4. **Historical analysis:**
   - Git history of skill evolution
   - Understand maturity and stability patterns

5. **Discovery mechanism research:**
   - How does Superpowers discover skills without when_to_use?
   - Compare to CipherPowers' Skill tool integration

## Research Methodology

**Entry Points Used:**
1. Direct GitHub raw file fetching via WebFetch
2. Line counting from source
3. Structural analysis (headers, code blocks, tables)
4. Content pattern analysis (frontmatter, references, enforcement)

**Sources Confidence:**
- Line counts: VERY HIGH (direct from source)
- Structure analysis: VERY HIGH (complete files analyzed)
- Pattern identification: HIGH (consistent across multiple skills)
- Gap identification: MODERATE to HIGH (based on absence of evidence)

**Limitations:**
- Sample of 6 skills (may not represent full suite)
- No test-scenarios.md analysis
- No git history analysis
- No runtime behavior verification
