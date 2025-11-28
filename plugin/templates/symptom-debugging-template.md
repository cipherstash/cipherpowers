---
name: Symptom-Based Debugging Template
description: Structure for debugging documentation organized by observable symptoms
when_to_use: when creating debugging guides, troubleshooting docs, or FIX/ section content
---

# [Symptom Category] Debugging Guide

**Last Updated:** [YYYY-MM-DD]

> Developers search by what they **see**, not by root cause.

---

## Quick Diagnosis Table

| Symptom | Likely Cause | Investigation | Priority |
|---------|--------------|---------------|----------|
| [Observable symptom 1] | [Root cause] | [Link to investigation] | ⚠️ High |
| [Observable symptom 2] | [Root cause] | [Link to investigation] | ☠️ Critical |
| [Observable symptom 3] | [Root cause] | [Link to investigation] | ✅ Low |

---

## [Symptom 1: Specific Observable Behavior]

### What You See

[Describe exactly what the developer observes - error messages, visual artifacts, unexpected behavior]

### Likely Causes

1. **[Cause A]** (most common)
   - [Brief explanation]
   - [How to verify this is the cause]

2. **[Cause B]**
   - [Brief explanation]
   - [How to verify]

### Investigation Steps

1. [ ] [First diagnostic step]
   ```bash
   [command to run]
   ```
   Expected output: [what you should see if this is the issue]

2. [ ] [Second diagnostic step]
   ```bash
   [command]
   ```

3. [ ] [Third diagnostic step]

### Solutions

**If Cause A:**
```[language]
[code fix or steps]
```

**If Cause B:**
[Steps to resolve]

### Prevention

- [How to prevent this in the future]
- [Related best practice]

---

## [Symptom 2: Another Observable Behavior]

### What You See

[Description]

### Likely Causes

1. **[Cause]**
   - [Explanation]

### Investigation Steps

1. [ ] [Step]

### Solutions

[Fix]

---

## General Debugging Workflow

When none of the specific symptoms match:

1. **Reproduce consistently**
   - [ ] Identify exact steps to reproduce
   - [ ] Note environment differences (dev vs prod)

2. **Isolate the problem**
   - [ ] Identify when it started (recent commits?)
   - [ ] Minimal reproduction case

3. **Gather evidence**
   - [ ] Collect logs
   - [ ] Check monitoring/metrics
   - [ ] Review recent changes

4. **Form hypothesis**
   - [ ] What could cause this behavior?
   - [ ] How can I verify?

5. **Test and fix**
   - [ ] Apply fix
   - [ ] Verify symptom is gone
   - [ ] Add regression test

---

## When to Escalate

**Escalate immediately if:**
- [ ] Data corruption suspected
- [ ] Security implications
- [ ] Production system down
- [ ] Multiple users affected
- [ ] You've spent > [X] hours without progress

**Escalation path:**
1. [Team/person to contact]
2. [Backup contact]

---

## Related Resources

- **Investigation strategies:** [Link to investigation/]
- **Known solutions catalog:** [Link to solutions/]
- **Architecture context:** [Link to UNDERSTAND/]
