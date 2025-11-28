---
name: Research Package Template
description: Self-contained module for complex domain knowledge with multiple entry points
when_to_use: when documenting complex research topics, domain knowledge, or technical deep dives that need multiple reading paths
---

# [Topic] Research Package

**Status:** [✅ Verified | ⚠️ Draft | 🎯 Ready for Review]
**Last Verified:** [YYYY-MM-DD]
**Version:** [e.g., Bevy 0.15.x, API v2.0]

---

## TL;DR

[2-3 sentences capturing the essential insight. What would someone need to know if they only had 30 seconds?]

---

## Reading Paths

Choose based on your time budget:

### Option A: "Just tell me the essentials" (5 minutes)
1. Read this TL;DR section
2. Skim [key-findings.md]
3. Done

### Option B: "I need to understand the context" (20 minutes)
1. Read README.md (this file)
2. Read [design-decisions.md]
3. Reference [QUICK-REFERENCE.md] as needed

### Option C: "I need to implement something" (45 minutes)
1. Read README.md
2. Read [implementation-guide.md]
3. Work through [examples/]
4. Use [QUICK-REFERENCE.md] during implementation

### Option D: "I need to fully understand the domain" (2+ hours)
1. Read all documents in sequence
2. Cross-reference with [external-resources.md]
3. Complete exercises if provided

---

## Role-Based Paths

### If You're a [Role 1, e.g., Frontend Developer]
**Goal:** [What they need to accomplish]
**Reading order:** [file1.md] → [file2.md] → [file3.md]
**Time:** ~[X] minutes
**Takeaway:** [Key insight for this role]

### If You're a [Role 2, e.g., Backend Engineer]
**Goal:** [What they need to accomplish]
**Reading order:** [file1.md] → [file2.md] → [file3.md]
**Time:** ~[X] minutes
**Takeaway:** [Key insight for this role]

---

## Package Contents

```
[topic]/
├── 00-START-HERE.md           # Entry point (you are here or see this)
├── README.md                   # Package overview + TL;DR
├── how-to-use-this.md         # Detailed navigation guide
├── [core-topic-1].md          # Main research content
├── [core-topic-2].md          # Additional research
├── design-decisions.md        # Why decisions were made
├── QUICK-REFERENCE.md         # One-page summary
├── VERIFICATION-REVIEW.md      # Accuracy audit (if applicable)
└── examples/                   # Working examples
```

---

## Key Concepts

### [Concept 1]

[Brief explanation with link to detailed doc]

**See:** [detailed-concept-1.md]

### [Concept 2]

[Brief explanation with link to detailed doc]

**See:** [detailed-concept-2.md]

---

## Verification Status

| Document | Status | Last Verified | Notes |
|----------|--------|---------------|-------|
| [document-1.md] | ✅ Verified | YYYY-MM-DD | [Any notes] |
| [document-2.md] | ⚠️ Needs Review | YYYY-MM-DD | [What needs updating] |

---

## Related Resources

- **Internal:** [Link to related internal docs]
- **External:** [Link to authoritative external sources]
- **Code:** [Link to relevant code if applicable]

---

## Maintenance Notes

**Last major update:** [YYYY-MM-DD]
**Next review scheduled:** [YYYY-MM-DD or "As needed"]
**Owner:** [Team or individual responsible]
