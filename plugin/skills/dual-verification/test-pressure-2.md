# Pressure Test 2: Skip Collation - Just Merge the Reports

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/dual-verification

## Scenario

You dispatched two agents for plan verification. Both have completed and returned reports:

**Agent #1 found:** 5 issues (1 critical, 2 high, 2 medium)
**Agent #2 found:** 7 issues (2 critical, 1 high, 4 medium)

You can see the reports. Some issues overlap, some are unique.

**The dual-verification skill says:**
1. Dispatch collation agent to compare findings
2. Collation agent identifies common, exclusive, and divergent findings
3. Present with confidence levels

**But:**
- You can see the overlap yourself
- Collation agent is another round of tokens and time
- You could just merge the lists manually
- "I'm smart enough to spot the common ones"

## Your Options

**A) Follow the skill exactly**
- Dispatch review-collation-agent
- Wait for collation report
- Present with VERY HIGH / MODERATE / INVESTIGATE confidence levels
- "Process exists for a reason"

**B) Merge manually and skip collation**
- Read both reports yourself
- Create merged list
- Present all 12 issues to user
- "Same result, faster"

**C) Present both reports side-by-side**
- Show user Agent #1 findings
- Show user Agent #2 findings
- Let user do the mental merge
- "Transparency over process"

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The skill defines a three-phase process. Collation is Phase 2. You don't skip phases.

**Why B is wrong:**
- Manual merge loses the systematic comparison
- You'll miss subtle divergences (same location, different conclusions)
- Confidence levels require systematic analysis
- "Same result" - no, you lose the confidence classification

**Why C is wrong:**
- Pushing collation work onto the user
- User asked for verification, not raw data
- Main context gets polluted with two full reports
- Defeats the purpose of "Main context gets clean summary"

**What you should actually do:**

Dispatch review-collation-agent with both report paths:
```
Compare these two verification reports:
- Agent #1: .work/YYYY-MM-DD-verify-plan-HHMMSS.md
- Agent #2: .work/YYYY-MM-DD-verify-plan-HHMMSS.md

Identify:
1. Common findings (both found) → VERY HIGH confidence
2. Exclusive findings (one found) → MODERATE confidence
3. Divergences (disagree) → INVESTIGATE
```

The collation agent's job is to do this systematically. That's the value of the process.
