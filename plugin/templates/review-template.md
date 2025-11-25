---
name: Review Template
description: Structured format for individual reviews or research in dual-verification pattern
when_to_use: when conducting independent reviews or research as part of dual-verification
related_practices: code-review.md, development.md
version: 1.0.0
---

# Review - {Date}

## Metadata
- **Reviewer:** [Agent type, e.g., technical-writer, plan-review-agent, Explore]
- **Date:** {YYYY-MM-DD HH:mm:ss}
- **Subject:** [What is being reviewed/researched - file path(s), topic, question, etc.]
- **Ground Truth:** [What this is verified against - codebase, requirements, standards, etc.]
- **Context:** [Standalone / Independent review #[1/2] for dual-verification]
- **Mode:** [Review / Research]

## Summary
- **Subject:** [Brief description]
- **Scope:** [What was reviewed/researched]

---

## For Reviews (use BLOCKING/SUGGESTIONS)

## Status: [BLOCKED | APPROVED WITH SUGGESTIONS | APPROVED]

## BLOCKING (Must Address)

[Issues or "None"]

**[Issue title]:**
- Description: [what's wrong or missing]
- Location: [file:line or section]
- Impact: [why this blocks]
- Action: [what needs to be done]

## SUGGESTIONS (Would Improve Quality)

[Suggestions or "None"]

**[Suggestion title]:**
- Description: [what could be improved]
- Location: [file:line or section]
- Benefit: [how this would help]
- Action: [optional improvement]

---

## For Research (use FINDINGS)

## FINDINGS

[Key discoveries, observations, or answers]

**[Finding title]:**
- Description: [what was discovered]
- Location: [where this was found - file, source, etc.]
- Relevance: [how this relates to the research question]
- Confidence: [HIGH / MEDIUM / LOW]

**[Finding title]:**
- Description: [what was discovered]
- Location: [where this was found]
- Relevance: [how this relates to the research question]
- Confidence: [HIGH / MEDIUM / LOW]

## QUESTIONS (Unresolved or for further investigation)

[Open questions or "None"]

- [Question needing further investigation]
- [Question needing further investigation]

---

## Assessment

**Conclusion:**
[Brief explanation of findings or decision]

**Confidence in findings:**
[Any areas where verification was limited or uncertain]
