# Verify

Generic dual-verification dispatcher for high-confidence verification across all verification types.

**Core principle:** Agents cannot be trusted. Two independent agents + systematic collation + cross-check = confidence.

**Parallel workflow:** User can `/revise common` immediately after collation while cross-check validates exclusive issues in background.

## Usage

```
/cipherpowers:verify <type> [scope] [--model=<sonnet|opus|haiku>]
```

**Model guidance:**
- `opus` - Deep analysis, security-critical verification, complex codebases
- `sonnet` - Balanced quality/speed (default for most verification types)
- `haiku` - Quick checks, simple verifications, execute adherence checks

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. What verification type is requested?
   - code → Dispatch to code verification workflow
   - plan → Dispatch to plan verification workflow
   - execute → Dispatch to execute verification workflow
   - research → Dispatch to research verification workflow
   - docs → Dispatch to documentation verification workflow
   - OTHER → Error: Unknown verification type. Valid types: code, plan, execute, research, docs

2. **MANDATORY: Skill Activation**

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/dual-verification/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:dual-verification"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:dual-verification")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

3. **FOLLOW THE SKILL EXACTLY:**
   - Phase 1: Dispatch 2 specialized agents in parallel (see dispatch table)
   - Phase 2: Dispatch review-collation-agent to compare findings → present to user
   - Phase 3: Dispatch cross-check agent to validate exclusive issues (background)

4. **After Phase 2 (collation):**
   - Present collation results to user immediately
   - Announce: "Can `/revise common` now. Cross-check starting..."
   - Dispatch cross-check agent (see cross-check table)

5. **After Phase 3 (cross-check):**
   - Present cross-check results
   - Announce: "Cross-check complete. `/revise exclusive` or `/revise all` ready"

6. **STOP when cross-check is complete.**

## Dispatch Table

| Type | Agent | Focus | Default Model |
|------|-------|-------|---------------|
| code | cipherpowers:code-review-agent + cipherpowers:code-agent | Heterogeneous review (Standards + Engineering) | sonnet |
| plan | cipherpowers:plan-review-agent + cipherpowers:code-agent | Plan quality + Technical feasibility | sonnet |
| execute | cipherpowers:execute-review-agent ×2 | Plan adherence, implementation match | haiku |
| research | cipherpowers:research-agent ×2 | Information completeness, accuracy | sonnet |
| docs | cipherpowers:technical-writer + cipherpowers:code-agent | Docs structure + Code example accuracy | haiku |

**Model parameter rules:**
- If user specified `--model=X` → pass `model: X` to ALL dispatched agents
- If no model specified → use default model from table above
- Collation agent always uses `haiku` (simple comparison task)

## Cross-check Dispatch Table

| Type | Cross-check Agent | Purpose | Model |
|------|-------------------|---------|-------|
| code | cipherpowers:code-agent | Verify exclusive issues against codebase | haiku |
| plan | cipherpowers:plan-review-agent | Verify exclusive concerns against requirements | haiku |
| execute | cipherpowers:execute-review-agent | Verify exclusive deviations against plan | haiku |
| research | Explore | Verify exclusive findings against sources | haiku |
| docs | cipherpowers:code-agent | Verify exclusive claims against implementation | haiku |

**Agent selection rationale:**
- Cross-check uses a **different agent type** than original reviewers to get fresh perspective
- `code-agent` for docs/code: Can read actual files to verify claims
- `plan-review-agent` for plans: Applies same criteria as original review
- `Explore` for research: Fast codebase search to validate findings
- `haiku` model: Cross-check is verification, not deep analysis

**Cross-check validates exclusive issues with states:**
- **VALIDATED:** Issue confirmed to exist → implement via `/revise exclusive`
- **INVALIDATED:** Issue doesn't apply → skip (auto-excluded from `/revise`)
- **UNCERTAIN:** Cannot determine → user decides

## Verification Types

### Code Verification

**When to use:** Before merging, after significant implementation.

**What it checks:**
- Code quality and standards compliance
- Testing coverage and quality
- Security considerations
- Performance implications
- Maintainability

**Workflow:**
```
/verify code [scope] [--model=<sonnet|opus|haiku>]

Phase 1: Dual Review
→ Dispatches 1 code-review-agent and 1 code-agent in parallel
→ Each agent independently reviews code changes

Phase 2: Collate and Present
→ Dispatches review-collation-agent (haiku)
→ Presents collation to user: "Can `/revise common` now"

Phase 3: Cross-check (background)
→ Dispatches code-agent to validate exclusive issues
→ Updates collation with VALIDATED/INVALIDATED/UNCERTAIN
→ Announces: "Cross-check complete. `/revise exclusive` ready"
```

### Plan Verification

**When to use:** Before executing implementation plans.

**What it checks:**
- 35 quality criteria (security, testing, architecture, etc.)
- Blocking issues that must be fixed
- Non-blocking improvements to consider

**Workflow:**
```
/verify plan [plan-file] [--model=<sonnet|opus|haiku>]

Phase 1: Dual Review
→ Dispatches 1 plan-review-agent and 1 code-agent in parallel
→ Each agent independently evaluates against criteria

Phase 2: Collate and Present
→ Dispatches review-collation-agent (haiku)
→ Presents collation to user: "Can `/revise common` now"

Phase 3: Cross-check (background)
→ Dispatches plan-review-agent to validate exclusive concerns
→ Updates collation with VALIDATED/INVALIDATED/UNCERTAIN
→ Announces: "Cross-check complete. `/revise exclusive` ready"
```

### Execute Verification

**When to use:** After each batch during /execute workflow.

**What it checks:**
- Each task implemented exactly as plan specified
- No skipped requirements
- No unauthorized deviations
- No incomplete implementations

**What it does NOT check:**
- Code quality (that's code verification)
- Testing strategy (that's code verification)
- Standards compliance (that's code verification)

**Workflow:**
```
/verify execute [batch-number] [plan-file] [--model=<sonnet|opus|haiku>]

Phase 1: Dual Review
→ Dispatches 2 execute-review-agent agents in parallel
→ Each agent verifies: COMPLETE / INCOMPLETE / DEVIATED

Phase 2: Collate and Present
→ Dispatches review-collation-agent (haiku)
→ Presents collation to user: "Can `/revise common` now"

Phase 3: Cross-check (background)
→ Dispatches execute-review-agent to validate exclusive deviations
→ Updates collation with VALIDATED/INVALIDATED/UNCERTAIN
→ Announces: "Cross-check complete. `/revise exclusive` ready"
```

### Research Verification

**When to use:** When exploring unfamiliar topics, APIs, patterns, or codebases.

**What it checks:**
- Information completeness (did we find everything relevant?)
- Accuracy (are findings correct?)
- Multiple perspectives (different angles covered?)
- Gaps identified (what's missing?)

**Examples:**
- "How does authentication work in this codebase?"
- "What are the patterns for Bevy 0.17 picking?"
- "How should we structure the API layer?"

**Workflow:**
```
/verify research [topic] [--model=<sonnet|opus|haiku>]

Phase 1: Dual Research
→ Dispatches 2 research-agent agents in parallel
→ Each agent explores from different perspectives

Phase 2: Collate and Present
→ Dispatches review-collation-agent (haiku)
→ Presents collation: common findings, exclusive insights, divergences
→ User informed: "Common findings ready. Cross-check starting..."

Phase 3: Cross-check (background)
→ Dispatches Explore agent to validate exclusive findings
→ Updates collation with VALIDATED/INVALIDATED/UNCERTAIN
→ Announces: "Cross-check complete. All findings validated."
```

### Documentation Verification

**When to use:** Auditing documentation accuracy.

**What it checks:**
- File paths exist
- Commands work
- Examples accurate
- Structure complete

**Workflow:**
```
/verify docs [files] [--model=<sonnet|opus|haiku>]

Phase 1: Dual Review
→ Dispatches 1 technical-writer and 1 code-agent in parallel
→ Each agent independently verifies against codebase

Phase 2: Collate and Present
→ Dispatches review-collation-agent (haiku)
→ Presents collation to user: "Can `/revise common` now"

Phase 3: Cross-check (background)
→ Dispatches code-agent to validate exclusive claims
→ Updates collation with VALIDATED/INVALIDATED/UNCERTAIN
→ Announces: "Cross-check complete. `/revise exclusive` ready"
```

## Why Dual Verification?

**Problem:** Single agent can miss issues, hallucinate, or confirm biases.

**Solution:** Two independent agents + systematic collation + cross-check = confidence.

**Confidence levels (after collation):**
- **VERY HIGH:** Both agents found → `/revise common` immediately
- **MODERATE:** One agent found → Pending cross-check validation
- **INVESTIGATE:** Agents disagree → Resolved during collation

**Exclusive issue states (after cross-check):**
- **VALIDATED:** Cross-check confirmed → `/revise exclusive`
- **INVALIDATED:** Doesn't apply → Auto-excluded from `/revise`
- **UNCERTAIN:** Cannot determine → User decides

**Example (research):**
```
Agent #1: "Auth uses JWT with 1-hour expiry"
Agent #2: "Auth uses JWT with 24-hour refresh tokens"

→ Collation: Both partially correct (divergence resolved)
→ Exclusive finding: Agent #2 found rate limiting
→ Cross-check: VALIDATED (rate limiting exists in codebase)
→ Higher confidence understanding than single agent
```

## Integration with Other Commands

**Verify → Revise workflow:**
```
/verify docs README.md CLAUDE.md
  → Phase 1: Dual review
  → Phase 2: Collation presented → Can `/revise common` now
  → Phase 3: Cross-check runs in background

/revise common
  → Implements common issues (VERY HIGH confidence)
  → Can run while cross-check is still running

[Cross-check completes]
  → "Cross-check complete. `/revise exclusive` ready"

/revise exclusive
  → Implements VALIDATED exclusive issues only
  → Skips INVALIDATED issues automatically
  → Prompts for UNCERTAIN issues
```

**Execute workflow uses verify for batch verification:**
```
/execute workflow:
  → Batch 1 (3 tasks)
  → /verify code (quality/standards)
  → /verify execute (plan adherence)
  → /revise common (fix high-confidence issues)
  → Repeat for next batch
```

## Related Commands

- `/cipherpowers:revise` - Implement findings from verification (supports scope: common, exclusive, all)
- `/cipherpowers:execute` - Plan execution workflow (uses /cipherpowers:verify for batch verification)

## Related Skills

- `dual-verification` - Core pattern for all dual-verification
- `executing-plans` - Plan execution workflow integrating verification

## Related Agents

- `code-review-agent` & `code-agent` - Code quality verification
- `plan-review-agent` & `code-agent` - Plan quality verification
- `execute-review-agent` - Plan adherence verification
- `research-agent` - Research verification
- `technical-writer` & `code-agent` - Documentation verification
- `review-collation-agent` - Generic collation (works for all types)

## Remember

- All verification types use dual-verification pattern
- Phase 1: Dual review → Phase 2: Collate and present → Phase 3: Cross-check
- User can `/revise common` immediately after Phase 2
- Cross-check runs in background (Phase 3) while user works
- Exclusive issues: VALIDATED (implement) / INVALIDATED (skip) / UNCERTAIN (user decides)
- Agents cannot be trusted - that's why we use two + cross-check
