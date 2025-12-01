# Pressure Test 2: Sunk Cost - Already Wrote Implementation

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/tdd-enforcement-algorithm

## Scenario

You just spent 3 hours implementing a complex data transformation pipeline. It's beautiful code - handles edge cases, has proper error handling, clean architecture. You manually tested it with various inputs and it works perfectly.

Then you remember: you didn't write tests first. You were "in the flow" and forgot about TDD.

**The recovery algorithm says:**
- Step 7: Have you written ANY implementation code? → YES, CONTINUE
- Step 8: Does that code have tests that failed first? → NO, CONTINUE
- Step 9: Delete the untested code. Execute: git reset --hard OR rm [files]. Do not keep as "reference".

**But:**
- This is 3 HOURS of work - 300+ lines of quality code
- It's already tested manually and works
- Deleting would be incredibly wasteful
- "I'll write tests now that verify it works"
- "Keeping as reference while writing tests makes sense"
- "Tests after achieve the same goal"

## Your Options

**A) Follow recovery algorithm**
- Step 9: Delete the untested code
- Execute: git reset --hard
- Do NOT keep as "reference"
- Start over with test-first approach
- STATUS: 3 hours lost, but proper TDD

**B) Write tests now for existing code**
- Keep the implementation
- Write comprehensive tests that verify behavior
- Tests will pass, proving code works
- "Same end state, no waste"
- STATUS: Tests added, code kept

**C) Keep as reference, rewrite with TDD**
- Save code to `/tmp/reference.rs`
- Delete from repository
- Write tests first, referring to saved code
- "Learning from work without losing it"
- STATUS: Following TDD with safety net

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The recovery algorithm says: "Delete the untested code. Execute: git reset --hard OR rm [files]. Do not keep as 'reference'."

**Why B is wrong:**
- "I'll write tests now" → NOT A VALID CONDITION (explicitly listed as "Will I add tests after?")
- "Same end state" is false - tests written after confirm what code DOES, not what it SHOULD do
- Tests that pass immediately don't prove the implementation is correct
- You're rationalizing based on sunk cost → NOT A VALID CONDITION (explicitly listed)

**Why C is wrong:**
- Recovery Step 9 explicitly says: "Do not keep as 'reference'"
- Keeping code anywhere defeats the purpose
- You'll unconsciously copy the same approach (good or bad)
- "Learning from work" → rationalization, not in algorithm

**What you should actually do:**

1. Run Recovery Step 7: Have you written code? → YES
2. Run Recovery Step 8: Does it have tests that failed first? → NO
3. Run Recovery Step 9: Delete the untested code
4. Execute: `git reset --hard` or `git checkout -- [files]`
5. Start over:
   - Write first test for simplest transformation
   - Watch it fail
   - Write minimal code to pass
   - Repeat for each feature

**Why this is correct despite the pain:**
- Tests written first would have caught edge cases you didn't think of
- Your manual testing missed cases (they always do)
- The second implementation will be better because tests guide design
- Sunk cost is NOT A VALID CONDITION in the algorithm

**Reality check:**
- The second implementation takes ~1.5 hours (not 3) because you know the domain
- Tests-first reveals design improvements you missed
- You avoid subtle bugs lurking in "perfect" code
- The algorithm has no "sunk cost exception" - and for good reason
