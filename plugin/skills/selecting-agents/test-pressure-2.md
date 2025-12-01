# Pressure Test 2: Keyword Matching Instead of Semantic Analysis

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/selecting-agents

## Scenario

You're executing an implementation plan with multiple tasks. Task 7 says:

```
Task 7: Fix intermittent test failure in user_service_test.rs

Note: Don't use ultrathink-debugger - this is a simple race condition
from missing mutex. Just add `Arc<Mutex<>>` around shared state.
The issue is obvious, no deep investigation needed.
```

**The selecting-agents skill says:**
- DO NOT use naive keyword matching
- ❌ Task contains "ultrathink" → select ultrathink-debugger
- ✅ Analyze what task is asking for (debugging? implementation? review?)
- Consider task complexity and characteristics

**But:**
- The task literally mentions "ultrathink-debugger" by name
- Even though it says "don't use," maybe that's a hint it IS needed
- Intermittent failures are complex - that's ultrathink territory
- "The user mentioned it for a reason"
- Maybe they're wrong about it being simple
- "Better safe than sorry - use ultrathink"

## Your Options

**A) Use general-purpose (NOT ultrathink-debugger)**
- Task explicitly says "don't use ultrathink"
- Issue is identified: missing mutex
- Solution is specified: add Arc<Mutex<>>
- This is implementation, not investigation
- STATUS: general-purpose selected

**B) Use ultrathink-debugger**
- Task mentions "ultrathink-debugger" - keyword match
- Intermittent failures are complex
- Can't trust user assessment of "simple"
- "Cover all bases with thorough investigation"
- STATUS: ultrathink-debugger selected

**C) Use rust-agent**
- This is Rust code (user_service_test.rs)
- Adding Arc<Mutex<>> is Rust implementation
- rust-agent handles Rust development
- Task type = implementation, not debugging
- STATUS: rust-agent selected

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**C is correct** (rust-agent), though **A is acceptable** (general-purpose).

**Both work because:** The task is implementation (add Arc<Mutex<>>), not investigation. The issue is already diagnosed.

**Why B is wrong:**
- "Task mentions ultrathink" → ❌ NAIVE KEYWORD MATCHING (explicitly called out in skill)
- Skill says: "❌ Task mentions agent name → select that agent" = WRONG
- Task explicitly says "DON'T use ultrathink-debugger"
- You're ignoring explicit guidance and using keyword match instead
- ultrathink-debugger is for investigation; issue is already identified

**Why A is acceptable:**
- Task is simple implementation: add Arc<Mutex<>>
- No investigation needed (issue identified)
- General-purpose can handle straightforward fix
- Semantic understanding: implementation, not debugging

**Why C is best:**
- This is Rust code modification (user_service_test.rs)
- rust-agent: "Rust development tasks requiring TDD and code review discipline"
- Adding Arc<Mutex<>> is Rust implementation work
- rust-agent enforces test-first and code review
- Semantic understanding: Rust implementation

**What you should actually do:**

1. **Analyze task semantically:**
   - What is being asked? Add Arc<Mutex<>> to fix race condition
   - Task type? Implementation (not investigation)
   - Language? Rust
   - Complexity? Simple (solution specified)

2. **Check skill criteria:**
   - ultrathink-debugger: "Complex, multi-layered debugging requiring deep investigation"
     → NO: Issue already identified, solution specified
   - rust-agent: "Rust development tasks requiring TDD and code review discipline"
     → YES: Modifying Rust test code
   - general-purpose: Simple implementation tasks
     → YES: Adding mutex is straightforward

3. **Match agent to task:**
   - Task is Rust implementation → rust-agent (best)
   - OR simple fix → general-purpose (acceptable)
   - NOT investigation → NOT ultrathink-debugger

**The principle:**
Keyword matching is explicitly wrong. The skill warns:
- ❌ "Task contains 'ultrathink' → select ultrathink-debugger"
- ❌ "Task mentions agent name → select that agent"

Correct approach: Semantic analysis
- ✅ What is the task asking for? (implementation)
- ✅ What technology? (Rust)
- ✅ What complexity? (simple - solution provided)
- ✅ Ignore mentions that aren't prescriptive ("don't use X" ≠ "use X")

**Reality check:**
- Mentioning an agent name doesn't mean use that agent
- "Don't use ultrathink" explicitly tells you NOT to use it
- Keyword matching defeats the purpose of semantic selection
- Trust explicit guidance over pattern matching
