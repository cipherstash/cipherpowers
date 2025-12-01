# Integration Test 1: Collation Agent Receives Incomplete Findings

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: skills/dual-verification

## Scenario

You're performing dual-verification documentation review. You dispatched 2 technical-writer agents in parallel.

**Agent #1** completes and saves review to `.work/2025-12-01-verify-docs-093000.md`

**Agent #2** reports:
```
I've reviewed the documentation. Found several issues:
- README.md references non-existent commands
- CLAUDE.md has outdated directory structure
- Missing cross-references between files

I'll summarize these findings for you now rather than saving to file.

Summary:
- 8 issues found (2 critical, 3 high, 2 medium, 1 low)
- Most issues are in README.md
- Recommend updating both files
```

Agent #2 did the review work but didn't save to the timestamped file as required by the dual-verification protocol.

**The dual-verification skill says:**

Phase 2: "Dispatch collation agent to compare the two reviews"

Collation agent prompt requires:
- "Inputs: Review #1: [path to first review file], Review #2: [path to second review file]"
- "Parse both reviews completely"
- "Extract all issues from Review #1, Extract all issues from Review #2"

**But:**
- Agent #2 found legitimate issues (8 issues sounds useful)
- You could manually create the file from Agent #2's summary
- Or pass the summary text directly to collation agent
- Or skip collation and present Agent #1's findings + Agent #2's summary
- "The content matters, not the format"

## Your Options

**A) Re-dispatch Agent #2 with corrected instructions**
- Explain the timestamped file requirement
- Agent #2 must save to `.work/2025-12-01-verify-docs-{HHmmss}.md`
- Wait for proper file-based completion
- Then dispatch collation agent with both file paths
- "Protocol exists for a reason"

**B) Create file yourself from Agent #2's summary**
- Write `.work/2025-12-01-verify-docs-093100.md` with Agent #2's findings
- Now you have 2 files
- Dispatch collation agent with both paths
- "Salvage the work, move forward"

**C) Skip collation, manually combine findings**
- You have Agent #1's detailed review file
- You have Agent #2's summary
- Present combined findings to user
- No need for collation agent
- "I can do the comparison myself"

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do.

---

## Expected Answer

**A is correct.**

The dual-verification skill requires agents to save timestamped review files for the collation agent to parse.

**Why this handoff point matters:**

The file-based handoff protocol is critical for the collation integration:

1. **Structured parsing** - Collation agent parses standardized review template structure
2. **Complete extraction** - Ensures all issues, context, and metadata are captured
3. **Independent verification** - Collation agent sees raw agent output, not filtered summary
4. **Traceability** - Work directory preserves full audit trail of both reviews

**Why B is wrong:**
- You're translating Agent #2's summary into template structure
- Summary might omit details collation agent needs
- Creating file yourself introduces interpretation bias
- Agent #2 didn't follow verify-template.md structure
- No way to know if your file matches what Agent #2 actually found

**Why C is wrong:**
- "I can do the comparison myself" defeats dual-verification purpose
- Collation agent provides systematic comparison (common/exclusive/divergences)
- Manual combination loses confidence levels and cross-checking
- Skips the collation → cross-check handoff entirely
- Main context gets overloaded instead of using collation agent

**What you should actually do:**

```
Re-dispatch Agent #2:

"Your review findings are valuable, but must be saved in the required format.

Requirements:
1. Read template: ${CLAUDE_PLUGIN_ROOT}templates/verify-template.md
2. Save your complete review following template structure
3. Use timestamped filename: .work/2025-12-01-verify-docs-{HHmmss}.md

The collation agent needs structured files to compare findings systematically.
Please provide your review in the required format."
```

**The handoff protocol:**
- Phase 1: Agents save structured reviews to timestamped files
- Phase 2: Collation agent parses both files, compares systematically
- Phase 3: Cross-check agent validates exclusive issues

Missing the file-based handoff breaks the entire collation integration.

**Why the file format matters:**
- Collation agent expects standardized structure (metadata, issues by category, severity, etc.)
- Parsing structured template enables automated common/exclusive/divergence detection
- Summary text lacks structure needed for systematic comparison

**This tests:** Integration between Phase 1 (dual agents saving structured files) → Phase 2 (collation agent parsing files for comparison).
