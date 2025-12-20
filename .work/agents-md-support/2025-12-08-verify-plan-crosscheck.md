# Cross-Check: Exclusive Plan Review Issues

**Date:** 2025-12-08
**Source:** Collation report `.work/2025-12-08-verify-plan-collated.md`
**Cross-check methodology:** Verified each exclusive issue by:
1. Locating the specific content in best practices document (agents-md-best-practices.md)
2. Checking whether the plan addresses this content
3. Assigning validation status based on evidence

## Summary

- **Total exclusive issues:** 7
- **VALIDATED:** 5 (confirmed gaps, should address)
- **INVALIDATED:** 2 (plan already covers or deliberate architectural choice)
- **UNCERTAIN:** 0 (all issues could be definitively classified)

## Reviewer #1 Exclusive Issues

### Issue: Hierarchical Approaches - Directory Choice
- **Source:** Reviewer #1
- **Validation:** INVALIDATED
- **Evidence:**
  - Best practice (lines 89-91): Suggests `agent_docs/` folder for supplementary files
  - Plan (line 7): "Architecture: New skill references existing docs/ structure (BUILD/FIX/UNDERSTAND/LOOKUP) rather than creating parallel hierarchies."
  - Plan AGENTS.md template (lines 87-92): References `docs/BUILD/00-START/`, `docs/LOOKUP/`, etc.
  - **Conclusion:** This is an intentional architectural decision documented in the plan header. CipherPowers deliberately uses its existing docs/ structure instead of creating a new agent_docs/ directory. This is a valid design choice for their architecture.
- **Recommendation:** SKIP - This is not a gap but a deliberate adaptation to CipherPowers' established architecture. The plan explicitly acknowledges this choice.

## Reviewer #2 Exclusive Issues

### Issue: Progressive Disclosure Pattern Explanation
- **Source:** Reviewer #2
- **Validation:** VALIDATED
- **Evidence:**
  - Best practice (lines 86-101): Extensive explanation of progressive disclosure pattern:
    - "progressive disclosure" (line 87)
    - "One approach is to maintain a primary AGENTS.md at the root with the universal info, and then have supplementary files for specific domains or components" (lines 88-90)
    - "Your main AGENTS.md would then refer the agent to these resources when appropriate" (lines 91-92)
    - "the detailed instructions are available to the AI but not loaded unless they're relevant to the task at hand" (lines 93-94)
    - Full paragraph on how to structure and use supplementary files
  - Plan: Uses progressive disclosure via docs/ references BUT does not explain the PATTERN itself
    - Template shows HOW to reference (`See [docs/UNDERSTAND/architecture.md] for detailed architecture documentation`)
    - Skill shows WHEN to use references (Section 4: "Reference, Don't Include")
    - MISSING: WHY this works (AI fetches on-demand, keeps main context light) and explicit naming of the pattern
- **Recommendation:** ADD - Include explicit explanation of progressive disclosure pattern in the skill's Core Principles section. This would help users understand the concept, not just follow the mechanics.

### Issue: Platform Capabilities / On-Demand Knowledge Pattern
- **Source:** Reviewer #2
- **Validation:** VALIDATED
- **Evidence:**
  - Best practice (lines 102-124): Extensive section "Tools and Skills to Maintain Context"
    - "Modern AI coding assistants often support external tools, skills, or hooks" (lines 102-103)
    - "Leveraging these can keep your AGENTS.md lean" (line 103)
    - Examples: skills, MCP, slash commands that supply info on-demand (lines 111-117)
    - "On-demand knowledge: Use skills or commands to fetch context (documentation, examples, etc.) as needed" (lines 119)
    - Specific pattern of using tools to provide context only when relevant
  - Plan: Mentions skills references but lacks the broader "on-demand knowledge" pattern
    - Skill section "Tool-First Content" (lines 249-264) covers linters/formatters
    - Template section "Related" (line 153) mentions skill reference
    - MISSING: Explicit guidance on using platform capabilities (hooks, MCP, slash commands) to provide info on-demand instead of front-loading everything
- **Recommendation:** ADD - Add subsection to skill explaining how to leverage platform tools for on-demand knowledge fetching. This extends the "tool-first" principle beyond just linters.

### Issue: Context Window is Precious
- **Source:** Reviewer #2
- **Validation:** VALIDATED
- **Evidence:**
  - Best practice (line 85): "Remember, the context window is precious – fill it with high-value information rather than exhaustive minutiae."
  - Plan: Principle is implied throughout but never explicitly stated as "context window is precious"
    - Skill discusses size limits (lines 230-240)
    - Skill discusses universal relevance (lines 242-247)
    - Common Rationalizations section (lines 392-400) mentions "attention drops linearly" but not "precious context"
    - MISSING: The explicit framing of "context window is precious" as a core principle
- **Recommendation:** ADD - Add explicit statement "context window is precious" to Core Principles section. This memorable phrase reinforces why all the other principles matter.

### Issue: Security/Special Considerations Section
- **Source:** Reviewer #2
- **Validation:** VALIDATED
- **Evidence:**
  - Best practice (lines 52-53): "Security or Special Considerations: If applicable, mention anything the AI should be cautious about, such as secrets management, regulatory requirements, or performance-critical sections."
  - Plan template: Does NOT include this optional section
    - Template includes: Project Overview, Quick Start, Architecture, Critical Guidelines, Key Files, See Also
    - MISSING: Optional section for security/compliance/performance warnings
- **Recommendation:** ADD - Add optional "Security & Special Considerations" section to template. Mark it as optional but provide structure for teams that need it.

### Issue: File-Level Explanatory Blurb
- **Source:** Reviewer #2
- **Validation:** VALIDATED
- **Evidence:**
  - Best practice (lines 149-151): "In practice, teams that have transitioned to AGENTS.md often add a note in the file itself to clarify its purpose and interoperability – e.g. a blurb at the top: 'Note: This project uses the open AGENTS.md standard...'"
  - Plan template: Does NOT include this explanatory note
    - Template header goes directly into project description
    - MISSING: Explanatory blurb about AGENTS.md standard and multi-agent compatibility
- **Recommendation:** ADD - Add optional explanatory blurb to template top explaining AGENTS.md standard. This helps contributors understand the file's purpose.

### Issue: Team Consistency Guidance
- **Source:** Reviewer #2
- **Validation:** INVALIDATED
- **Evidence:**
  - Best practice (lines 150-151): "This communicates to contributors what's going on and avoids confusion. Finally, encourage consistency among team members: if different developers use different AI tools, ensure everyone is contributing to and reading from the same AGENTS.md so knowledge doesn't get siloed or duplicated in tool-specific files."
  - Plan: This is META-GUIDANCE about using AGENTS.md, not content that goes IN the skill/template
    - The skill ITSELF serves this purpose - it teaches teams how to maintain a single source of truth
    - Multi-Agent Compatibility section (lines 352-381) covers unified files and avoiding duplication
    - This guidance is FOR teams adopting the practice, not IN the practice content
  - **Conclusion:** The plan's skill and template already embody this principle. The best practice quote is advice to teams about how to use AGENTS.md, not something that needs to be explicitly documented in the skill.
- **Recommendation:** SKIP - This is meta-level advice already embodied in the skill's design and multi-agent compatibility section.

### Issue: Monorepo Directory Structure
- **Source:** Reviewer #2
- **Validation:** VALIDATED (but lower priority)
- **Evidence:**
  - Best practice (lines 97-99): "Another form of hierarchy is to use directory-level AGENTS.md files in a monorepo or multi-project repository. For example, after migrating to the AGENTS.md standard, you might have: one top-level AGENTS.md with general info, plus an apps/AGENTS.md for frontend-specific instructions, and a backend/AGENTS.md for backend-specific instructions"
  - Plan: Does not address monorepo pattern
    - Hierarchical references covered via docs/ structure
    - MISSING: Specific guidance on directory-level AGENTS.md files in monorepos
  - **Note:** This is a specialized use case. CipherPowers itself is not a monorepo, so this may be lower priority.
- **Recommendation:** CONSIDER - Add brief mention of monorepo pattern in Multi-Agent Compatibility section or Hierarchical Approaches. Lower priority since CipherPowers itself doesn't use this pattern, but could help users with monorepo projects.

## Detailed Validation Analysis

### Validation Process

For each exclusive issue, I:
1. **Located the specific content** in agents-md-best-practices.md using the line numbers provided
2. **Read the full context** to understand what the best practice actually recommends
3. **Searched the plan** for any coverage of this content
4. **Classified the finding:**
   - VALIDATED: Best practice contains this insight, plan lacks it → Should add
   - INVALIDATED: Plan already covers this OR deliberate design choice → Skip
   - UNCERTAIN: Cannot determine clearly → Escalate

### VALIDATED Issues (Should Address)

**5 issues confirmed as legitimate gaps:**

1. **Progressive Disclosure Pattern Explanation** - Plan uses the pattern but doesn't explain the concept itself. Best practice has full explanation of why this works (AI fetches on-demand, keeps context light).

2. **Platform Capabilities / On-Demand Knowledge Pattern** - Best practice has extensive guidance on using skills, MCP, hooks for on-demand context. Plan covers tool-first for linters but misses the broader on-demand knowledge pattern.

3. **Context Window is Precious** - Best practice explicitly states this memorable principle (line 85). Plan implies it throughout but never states it directly.

4. **Security/Special Considerations Section** - Best practice includes this as optional template section (lines 52-53). Plan template omits it entirely.

5. **File-Level Explanatory Blurb** - Best practice recommends adding note explaining AGENTS.md standard (lines 149-151). Plan template doesn't include this.

### INVALIDATED Issues (Can Skip)

**2 issues that don't represent gaps:**

1. **Hierarchical Approaches - Directory Choice** - This is a deliberate architectural decision documented in the plan header. CipherPowers intentionally uses its existing docs/ structure instead of creating agent_docs/. This is valid design choice, not a gap.

2. **Team Consistency Guidance** - This is meta-advice about USING AGENTS.md, not content that goes IN the skill/template. The skill's design already embodies this principle through its multi-agent compatibility section.

## Recommendations

### High Priority (Clear Value)

Implement these 3 VALIDATED issues with clear benefits:

1. **Progressive Disclosure Pattern Explanation**
   - Add to: Skill Core Principles or Overview
   - Content: Explain what progressive disclosure is and why it works (AI fetches on-demand, reduces initial context load)
   - Why: Helps users understand the concept, not just follow mechanics

2. **Platform Capabilities / On-Demand Knowledge Pattern**
   - Add to: Skill Core Principles (extend "Tool-First Content" section)
   - Content: Guidance on using skills, hooks, MCP for on-demand context delivery
   - Why: Extends tool-first principle beyond linters to full platform capabilities

3. **Context Window is Precious**
   - Add to: Skill Core Principles (opening statement)
   - Content: Explicit statement "context window is precious" with brief explanation
   - Why: Memorable framing that reinforces all other principles

### Medium Priority (Template Enhancements)

Implement these 2 VALIDATED issues as template improvements:

4. **File-Level Explanatory Blurb**
   - Add to: Template top (optional comment)
   - Content: Note explaining AGENTS.md standard and multi-agent compatibility
   - Why: Helps contributors understand file's purpose

5. **Security/Special Considerations Section**
   - Add to: Template structure (mark as optional)
   - Content: Section for security, compliance, performance warnings
   - Why: Provides structure for teams with critical requirements

### Lower Priority (Specialized Use Case)

Consider for future enhancement:

6. **Monorepo Directory Structure** (VALIDATED but specialized)
   - Add to: Skill Multi-Agent Compatibility section
   - Content: Brief mention of directory-level AGENTS.md pattern in monorepos
   - Why: Useful for monorepo users but not core to CipherPowers' own architecture

### Skip (Already Covered)

Do NOT implement these 2 INVALIDATED issues:

7. **Hierarchical Approaches - Directory Choice** - Deliberate architectural choice
8. **Team Consistency Guidance** - Already embodied in skill design

## Summary by Confidence

| Validation | Count | Action |
|------------|-------|--------|
| VALIDATED | 5 | Implement via `/cipherpowers:revise exclusive` |
| INVALIDATED | 2 | Skip (auto-excluded) |
| UNCERTAIN | 0 | N/A |

## Next Steps

1. **Immediate:** `/cipherpowers:revise exclusive` will now implement 5 VALIDATED issues
2. **Optional:** User can choose to defer lower-priority issue (#6 monorepo) to future iteration
3. **Confirmed skip:** 2 INVALIDATED issues automatically excluded from revision workflow

---

## Cross-check Methodology

**Evidence-based validation:**
- Every validation includes specific line numbers from best practices document
- Every VALIDATED issue shows what's in best practices AND what's missing from plan
- Every INVALIDATED issue explains why it doesn't represent a gap

**Conservative approach:**
- When in doubt, validated (better to review than miss a gap)
- All 7 issues could be definitively classified (no UNCERTAIN)
- Clear reasoning provided for each decision

**Result:** High confidence in validation outcomes. Cross-check successfully separated legitimate gaps (5) from non-gaps (2).
