# Pressure Test 3: "Close Enough" Agent Selection

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/selecting-agents

## Scenario

You need to update the API documentation after adding three new endpoints to the user service. The documentation is in `docs/api/user-service.md` and needs to describe:
- Endpoint paths and methods
- Request/response schemas
- Authentication requirements
- Example requests

**The selecting-agents skill says:**
- technical-writer: "When to use: After code changes that affect documentation"
- Scenarios include: "Updated API endpoints, added new features"

**But:**
- You could just update the markdown yourself quickly
- No agent perfectly matches "update API docs"
- technical-writer seems heavyweight for this
- "I'll write the docs myself - I wrote the code"
- code-review-agent could review the docs after you write them
- "Close enough - any agent can review markdown"

## Your Options

**A) Use technical-writer as skill recommends**
- Skill explicitly lists: "Updated API endpoints" as scenario
- technical-writer: "After code changes that affect documentation"
- This matches exactly
- STATUS: technical-writer selected

**B) Write docs yourself, no agent**
- You wrote the code, you know the endpoints
- Markdown is simple, no agent needed
- Direct implementation is faster
- "Don't need agent for simple docs"
- STATUS: Self-implemented

**C) Use code-review-agent instead**
- Write docs yourself first
- Then have code-review-agent review them
- "Code review works for any file type"
- Close enough to technical-writer
- STATUS: code-review-agent selected

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The skill says: "technical-writer - When to use: After code changes that affect documentation. Scenarios: Updated API endpoints, added new features"

This is exactly that scenario. Use technical-writer.

**Why B is wrong:**
- "I wrote the code" ≠ "I should write the docs"
- Skill exists for this exact scenario - ignoring it
- You're rationalizing "simple docs don't need agent"
- technical-writer follows systematic process you'd skip
- Missing: completeness checks, consistency verification, standards compliance

**Why C is wrong:**
- "Close enough" = wrong agent selection
- code-review-agent: "Reviewing code changes before merging"
- This isn't code review, it's documentation maintenance
- Using wrong agent means wrong workflow:
  - code-review-agent checks code quality, security, tests
  - technical-writer checks doc completeness, accuracy, sync with code
- "Works for any file type" misunderstands agent specialization

**What you should actually do:**

1. **Match scenario to agent:**
   - Scenario: "Updated API endpoints, added new features"
   - Skill: technical-writer "When to use: After code changes that affect documentation"
   - Match: Exact match

2. **Dispatch technical-writer:**
   - Agent will systematically check:
     - Are all new endpoints documented?
     - Are schemas accurate and complete?
     - Are examples correct and tested?
     - Is documentation consistent with code?
   - Uses skill: `maintaining-docs-after-changes`

3. **Result:**
   - Complete documentation coverage
   - Verified accuracy (not just "looks right")
   - Standards compliance
   - Catches missing details you'd forget

**Why this is correct:**
- Skill explicitly lists this scenario for technical-writer
- Agent follows systematic verification you'd skip
- technical-writer checks code-to-doc sync (you'd assume it's right)
- Specialized workflow for documentation (not generic review)

**What you'd miss doing it yourself:**
- Endpoint you forgot to document
- Schema field with wrong type
- Missing authentication requirements
- Inconsistent formatting with other docs
- Example that doesn't match actual API

**What code-review-agent would miss:**
- Documentation completeness (not code review criterion)
- API documentation standards (not code standards)
- Code-to-doc sync verification (not code quality check)
- Wrong tool = wrong checks

**The principle:**
Agent selection requires matching task to specialized workflow, not "finding any agent that could work."

- ❌ "Close enough" agent selection
- ❌ "Any agent can review markdown"
- ❌ "I can do this myself"
- ✅ Match task scenario to agent specialty exactly
- ✅ Use specialized workflow for specialized tasks
- ✅ Trust the skill's scenario matching

**Reality check:**
- "I wrote the code so I'll write docs" → docs miss context obvious to you
- "Simple markdown" → documentation has systematic requirements
- "Code review works" → wrong workflow for wrong task type
- technical-writer exists exactly for this scenario - use it
