---
name: research-methodology
description: Thorough multi-angle research exploration with evidence requirements and structured output
when_to_use: during dual-verification research, when exploring topics comprehensively, when evidence-based findings are required
version: 1.0.0
---

# Research Methodology

## Overview

Conduct thorough research by exploring topics from multiple angles with rigorous evidence standards.

**Announce at start:** "I'm using the research-methodology skill for comprehensive exploration."

## When to Use

- During dual-verification research tasks
- When exploring topics that require multiple perspectives
- When evidence-based findings with confidence levels are required
- As part of `/cipherpowers:verify research`

## Research Process

### Step 1: Multi-Angle Exploration

Approach the topic from at least 3 different angles:
- Different perspectives (user vs developer, novice vs expert)
- Different scopes (local vs global, immediate vs long-term)
- Different concerns (functionality, performance, maintainability)

### Step 2: Evidence Requirements

Every finding MUST include:

```
Finding: [clear statement]
Source: [file path, line number, or documentation reference]
Confidence: HIGH | MEDIUM | LOW
Evidence: [quote or specific reference supporting the finding]
```

**Confidence Levels:**
- **HIGH:** Direct evidence, explicitly stated, no interpretation needed
- **MEDIUM:** Reasonable inference from available evidence
- **LOW:** Circumstantial evidence, requires verification

### Step 3: Gap Identification

Explicitly document:
- What you searched for but couldn't find
- Areas where evidence is insufficient
- Questions that remain unanswered

### Step 4: Structured Output

Use consistent report format:

```markdown
# Research Findings - [Topic]

## Metadata
- Date: {YYYY-MM-DD HH:mm:ss}
- Topic: [description]
- Angles Explored: [list]

## Findings

### [Category 1]
[Findings with evidence]

### [Category 2]
[Findings with evidence]

## Gaps and Unanswered Questions
[What couldn't be determined]

## Summary
[Key takeaways]
```

## Save Workflow

Save findings to: `.work/{YYYY-MM-DD}-verify-research-{HHmmss}.md`
Announce file path in final response.

## Status Reporting

End with:
- `STATUS: COMPLETE` - Research finished, all angles explored, findings documented
- `STATUS: INCOMPLETE` - Research blocked, unable to complete exploration

## Remember

- Quality over quantity - fewer well-evidenced findings beat many unsupported claims
- Explicit gaps are valuable - knowing what we don't know matters
- Confidence levels enable informed decisions
- Save report before completing
