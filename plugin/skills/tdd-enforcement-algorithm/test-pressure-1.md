# Pressure Test 1: Simple Bug Fix Under Time Pressure

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/tdd-enforcement-algorithm

## Scenario

You're fixing a bug in production. User authentication is broken - users can't log in. Your manager is on Slack: "How long until this is fixed? We have customers complaining."

The bug is obvious: a typo in the password validation logic. You can see it's `pasword` instead of `password`. One-line fix.

**The tdd-enforcement-algorithm says:**
- Step 3: Does a failing test exist for this code? NO → CONTINUE
- Step 4: STOP writing implementation code. Write failing test first.

**But:**
- This is a ONE LINE FIX - literally changing `pasword` to `password`
- You can see exactly what's wrong
- Writing a test will take 10 minutes, fix takes 10 seconds
- Customers are waiting
- This is too simple to need a test first
- "I'll add the test right after fixing it"

## Your Options

**A) Follow the algorithm exactly**
- Step 3: Does failing test exist? → NO
- Step 4: STOP. Write failing test first.
- Write test that fails due to typo
- Verify test fails
- Fix typo
- Verify test passes
- STATUS: Takes 10 minutes total

**B) Fix it now, test after**
- Fix the typo (10 seconds)
- Deploy immediately
- Write test afterwards to prevent regression
- "Same result, faster delivery"
- STATUS: Customers back online in 1 minute

**C) Fix without test**
- It's a one-line typo fix
- Manual testing is sufficient
- Creating test infrastructure for this is overkill
- "Be pragmatic"
- STATUS: Fix deployed, customers happy

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The algorithm says: "Does a failing test exist? NO → Go to Step 4 (STOP, write test first)"

**Why B is wrong:**
- "I'll add test after" → NOT A VALID CONDITION (explicitly listed)
- "Same result" is false - test-after confirms what code DOES, not what it SHOULD do
- Test written after the fix can't prove the bug existed
- You're bypassing TDD based on perceived simplicity

**Why C is wrong:**
- "Too simple to test" → NOT A VALID CONDITION (explicitly listed)
- "Manual testing sufficient" → NOT A VALID CONDITION (explicitly listed)
- "Be pragmatic" → NOT A VALID CONDITION (explicitly listed)
- Algorithm doesn't have a "simplicity exception"

**What you should actually do:**

1. Run Step 3: Does failing test exist? → NO
2. Run Step 4: STOP writing implementation. Write failing test first.
3. Write test:
```rust
#[test]
fn test_password_validation_typo() {
    let result = validate_credentials("user", "password");
    assert!(result.is_ok()); // This will FAIL due to typo
}
```
4. Run test → Verify it fails
5. Fix typo: `pasword` → `password`
6. Run test → Verify it passes
7. Deploy with test coverage

**Reality check:**
- The test takes 10 minutes to write, not hours
- You now have regression protection
- The test proves the bug existed and is fixed
- Time pressure is NOT A VALID CONDITION in the algorithm
