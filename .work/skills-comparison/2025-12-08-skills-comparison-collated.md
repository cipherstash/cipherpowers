# Skills Comparison: CipherPowers vs Upstream Superpowers

## Metadata
- Date: 2025-12-08
- Type: Dual-verification research collation
- Sources:
  - `.work/2025-12-08-cipherpowers-skills-analysis.md`
  - `.work/2025-12-08-superpowers-skills-analysis.md`

## Executive Summary

| Metric | CipherPowers | Superpowers | Finding |
|--------|--------------|-------------|---------|
| **Skills Analyzed** | 40 | 6 (sample) | CipherPowers has larger corpus |
| **Avg Line Count** | 221 lines | 203 lines | Similar average |
| **Avg Tokens (est)** | 3,315 | 3,050 | Similar token cost |
| **Size Range** | 54-622 | 76-397 | CipherPowers has wider variance |
| **Frontmatter Fields** | 2-5 fields | 2 fields only | CipherPowers richer metadata |

**Status:** APPROVED WITH SUGGESTIONS

Both skill systems demonstrate high quality with complementary strengths. CipherPowers has richer metadata and algorithmic enforcement; Superpowers has stronger meta-enforcement protocols.

## Common Patterns (VERY HIGH Confidence)

Both skill systems share these fundamental patterns:

| Pattern | CipherPowers | Superpowers | Notes |
|---------|--------------|-------------|-------|
| **Overview section** | 100% (40/40) | 100% (6/6) | Universal requirement |
| **Announce at start** | 20% (8/40) | 67% (4/6) | Superpowers more consistent |
| **Iron Law statements** | 12.5% (5/40) | 33% (2/6) | Both use for high-stakes |
| **Rationalization tables** | 45% (18/40) | 33% (2/6) | CipherPowers more widespread |
| **Red flags sections** | 25% (10/40) | 33% (2/6) | Similar usage |
| **Code blocks** | 67.5% (27/40) | 50% (3/6) | CipherPowers more code-heavy |
| **Tables** | 70% (28/40) | 33% (2/6) | CipherPowers more table-heavy |
| **Checklists** | 55% (22/40) | 50% (3/6) | Similar usage |
| **Cross-skill references** | Extensive | Extensive | Both compose via references |

## Exclusive Findings

### Unique to CipherPowers (MODERATE Confidence)

1. **Algorithmic decision trees**
   - Boolean GOTO/CONTINUE/STOP format
   - Claims 100% compliance vs 0-33% for imperatives
   - Skills: following-plans, tdd-enforcement-algorithm, algorithmic-command-enforcement

2. **TDD applied to skills**
   - RED-GREEN-REFACTOR cycle for skill creation
   - Baseline testing with subagents before writing
   - Documented in writing-skills, testing-skills-with-subagents

3. **Thin skill delegation pattern**
   - Agents reduced to ~30-50 lines
   - Workflow logic lives in skills, not agents
   - 83% reduction in agent code

4. **Rich frontmatter metadata**
   - `when_to_use` field (50% of skills)
   - `version` field (42.5% of skills)
   - `applies_to` field (some skills)
   - Enables better discovery and tracking

5. **Explicit template**
   - skill-template.md defines expected structure
   - 30 lines with clear section guidance
   - Enables compliance checking

### Unique to Superpowers (MODERATE Confidence)

1. **MANDATORY FIRST RESPONSE PROTOCOL**
   - In using-superpowers skill
   - Checklist of required actions before ANY response
   - Stronger meta-enforcement than CipherPowers

2. **"Remember" sections**
   - End-of-skill principle reminders
   - Found in writing-plans, executing-plans
   - Short, memorable takeaways

3. **More extensive rationalization tables**
   - 12+ entries in test-driven-development
   - More comprehensive excuse coverage
   - CipherPowers typically has 6-8 entries

4. **Prominent "Red Flags - STOP" naming**
   - Explicit section title pattern
   - Clear visual signal to halt
   - More prominent than CipherPowers style

5. **Minimal frontmatter philosophy**
   - Only name and description
   - Description does double duty (discovery + when_to_use)
   - Simpler but less structured

## Statistics Comparison

### Size Distribution

**CipherPowers (40 skills):**
| Range | Count | % | Examples |
|-------|-------|---|----------|
| <100 lines | 3 | 7.5% | brainstorming (54), research-methodology (98) |
| 100-200 lines | 15 | 37.5% | conducting-code-review (139), commit-workflow (156) |
| 200-300 lines | 17 | 42.5% | systematic-debugging (295), following-plans (257) |
| 300+ lines | 5 | 12.5% | dual-verification (530), writing-skills (622) |

**Superpowers (6 skills sample):**
| Range | Count | % | Examples |
|-------|-------|---|----------|
| <100 lines | 1 | 17% | brainstorming (76) |
| 100-200 lines | 3 | 50% | writing-plans (137), executing-plans (114) |
| 200-300 lines | 0 | 0% | - |
| 300+ lines | 2 | 33% | test-driven-development (397), systematic-debugging (387) |

### Frontmatter Comparison

**CipherPowers frontmatter fields:**
| Field | Count | % |
|-------|-------|---|
| name | 40 | 100% |
| description | 40 | 100% |
| when_to_use | 20 | 50% |
| version | 17 | 42.5% |
| applies_to | varies | - |

**Superpowers frontmatter fields:**
| Field | Count | % |
|-------|-------|---|
| name | 6 | 100% |
| description | 6 | 100% |
| (no other fields) | - | 0% |

### Section Presence

**CipherPowers:**
| Section | Present | % |
|---------|---------|---|
| Overview | 40/40 | 100% |
| When to Use | 34/40 | 85% |
| Quick Reference | 12/40 | 30% |
| Common Mistakes | 14/40 | 35% |
| Red Flags | 10/40 | 25% |
| Related Skills | 14/40 | 35% |
| Examples | 16/40 | 40% |

**Superpowers:**
| Section | Present | % |
|---------|---------|---|
| Overview | 6/6 | 100% |
| When to Use | 2/6 | 33% |
| Remember | 2/6 | 33% |
| Common Rationalizations | 2/6 | 33% |
| Red Flags | 2/6 | 33% |
| Iron Law | 2/6 | 33% |

## Template Compliance Analysis

### CipherPowers skill-template.md

**Template structure (30 lines):**
```
---
name: Skill Name
description: One-line summary
when_to_use: Symptoms and situations
version: 1.0.0
---
# Skill Name
## Overview
## When to Use
## Quick Reference
## Implementation
## Common Mistakes
```

**Compliance assessment:**
| Requirement | Compliant | % | Notes |
|-------------|-----------|---|-------|
| Frontmatter present | 40/40 | 100% | All have frontmatter |
| name field | 40/40 | 100% | All present |
| description field | 40/40 | 100% | All present |
| when_to_use field | 20/40 | 50% | Gap - should be 100% |
| version field | 17/40 | 42.5% | Gap - should be 100% |
| Overview section | 40/40 | 100% | All present |
| When to Use section | 34/40 | 85% | 6 missing |
| Quick Reference | 12/40 | 30% | Optional but valuable |
| Common Mistakes | 14/40 | 35% | Optional but valuable |

**Skills missing "When to Use" section (6):**
1. brainstorming
2. condition-based-waiting
3. defense-in-depth
4. dispatching-parallel-agents
5. root-cause-tracing
6. sharing-skills

**Skills missing version field (23):**
- Most older/legacy skills lack versioning
- Pattern: Newer skills (post-refactoring) have versions
- Recommendation: Add version to all for tracking

### Superpowers (No explicit template)

**Observed patterns from 6-skill sample:**
- 100% have Overview
- 33% have When to Use
- 33% have Remember
- 33% have Iron Law (enforcement skills only)
- 33% have Red Flags (enforcement skills only)

## Key Divergences

### 1. Frontmatter Philosophy

| Aspect | CipherPowers | Superpowers |
|--------|--------------|-------------|
| **Fields** | 2-5 (name, description, when_to_use, version, applies_to) | 2 only (name, description) |
| **Discovery** | Dedicated when_to_use field | Description serves dual purpose |
| **Tracking** | Version field for evolution | No versioning |
| **Trade-off** | Richer but more maintenance | Simpler but less structured |

**Recommendation:** Keep CipherPowers' rich frontmatter - it enables better tooling and discovery.

### 2. Meta-Enforcement

| Aspect | CipherPowers | Superpowers |
|--------|--------------|-------------|
| **Meta-skill** | using-cipherpowers (101 lines) | using-superpowers (109 lines) |
| **Protocol** | Lists available commands | MANDATORY FIRST RESPONSE PROTOCOL |
| **Enforcement** | Reminder to check for skills | Checklist before ANY response |
| **Strength** | Moderate | Strong |

**Recommendation:** Adopt MANDATORY FIRST RESPONSE PROTOCOL pattern in using-cipherpowers.

### 3. Rationalization Defense

| Aspect | CipherPowers | Superpowers |
|--------|--------------|-------------|
| **Table size** | 6-8 entries typical | 12+ entries in TDD |
| **Coverage** | Common excuses | Comprehensive excuses |
| **Format** | `| Excuse | Reality |` | `| Excuse | Reality |` |

**Recommendation:** Expand rationalization tables to 10+ entries for enforcement-heavy skills.

### 4. End-of-Skill Reminders

| Aspect | CipherPowers | Superpowers |
|--------|--------------|-------------|
| **Section** | Not standard | "Remember" section |
| **Purpose** | - | Key principles summary |
| **Location** | - | End of skill |

**Recommendation:** Add "Remember" sections to process skills (executing-plans, etc.).

### 5. Algorithmic Enforcement

| Aspect | CipherPowers | Superpowers |
|--------|--------------|-------------|
| **Format** | Boolean GOTO/CONTINUE/STOP | Not observed |
| **Claimed compliance** | 100% | - |
| **Skills using** | 3 (following-plans, tdd-enforcement-algorithm, algorithmic-command-enforcement) | 0 |

**Recommendation:** This is a CipherPowers strength - continue developing and documenting.

## Recommendations

### Priority 1: Template Compliance (Quick Wins)

1. **Add version to all skills**
   - Currently: 17/40 (42.5%)
   - Target: 40/40 (100%)
   - Effort: Low (add `version: 1.0.0` to frontmatter)

2. **Add "When to Use" sections to 6 skills**
   - brainstorming, condition-based-waiting, defense-in-depth
   - dispatching-parallel-agents, root-cause-tracing, sharing-skills
   - Effort: Low (copy pattern from similar skills)

3. **Add when_to_use frontmatter field to remaining 20 skills**
   - Currently: 20/40 (50%)
   - Target: 40/40 (100%)
   - Effort: Medium (write appropriate trigger conditions)

### Priority 2: Pattern Adoption

4. **Adopt MANDATORY FIRST RESPONSE PROTOCOL**
   - Add to using-cipherpowers skill
   - Checklist before any response
   - Effort: Medium

5. **Add "Remember" sections to process skills**
   - executing-plans, writing-plans, subagent-driven-development
   - Short principle reminders at end
   - Effort: Low

6. **Expand rationalization tables to 10+ entries**
   - Focus on: test-driven-development, systematic-debugging, verification-before-completion
   - Effort: Medium

### Priority 3: Documentation

7. **Document algorithmic enforcement pattern**
   - Already in algorithmic-command-enforcement skill
   - Create guidance for when to use algorithm vs imperative format
   - Effort: Low (documentation only)

8. **Create compliance checker**
   - Script to verify frontmatter fields and sections
   - Run in CI/pre-commit
   - Effort: Medium-High

9. **Build dependency graph**
   - Map all REQUIRED SUB-SKILL references
   - Check for circular dependencies
   - Effort: Medium

### Priority 4: Quality Improvements

10. **Audit test coverage**
    - Which skills have test-scenarios.md?
    - Run baseline tests for un-tested skills
    - Effort: High

## Conclusion

CipherPowers and Superpowers skills share common ancestry and patterns but have diverged in complementary directions:

**CipherPowers strengths:**
- Richer metadata for discovery and tracking
- Algorithmic enforcement for 100% compliance
- TDD applied to skill creation itself
- Thin skill delegation pattern for maintainability

**Superpowers strengths:**
- Stronger meta-enforcement protocol
- More comprehensive rationalization tables
- "Remember" sections for principle reminders
- Simpler frontmatter (less maintenance)

**Recommended actions:**
1. Standardize versioning across all 40 skills
2. Add missing "When to Use" sections (6 skills)
3. Adopt MANDATORY FIRST RESPONSE PROTOCOL
4. Add "Remember" sections to process skills
5. Expand rationalization tables

**Overall assessment:** Both systems are high quality. CipherPowers should adopt Superpowers' meta-enforcement patterns while maintaining its strengths in metadata and algorithmic enforcement.
