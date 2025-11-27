# Verify

Generic dual-verification dispatcher for high-confidence verification across all verification types.

**Core principle:** Agents cannot be trusted. Two independent agents + systematic collation = confidence.

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
   - Phase 2: Dispatch review-collation-agent to compare findings
   - Phase 3: Present collated findings to user with confidence levels

4. **STOP when verification is complete.**

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

→ Dispatches 1 code-review-agent and 1 code-agent in parallel
  (with model parameter if specified, otherwise sonnet)
→ Each agent independently reviews:
  - Read code changes
  - Run tests and checks
  - Review against standards
→ Dispatches review-collation-agent (always haiku)
→ Produces collated report with confidence levels
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

→ Dispatches 1 plan-review-agent and 1 code-agent in parallel
  (with model parameter if specified, otherwise sonnet)
→ Each agent independently evaluates against criteria
→ Dispatches review-collation-agent (always haiku)
→ Produces collated report with confidence levels
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

→ Dispatches 2 execute-review-agent agents in parallel
  (with model parameter if specified, otherwise haiku)
→ Each agent independently verifies:
  - Read plan tasks for batch
  - Read implementation changes
  - Verify each task: COMPLETE / INCOMPLETE / DEVIATED
→ Dispatches review-collation-agent (always haiku)
→ Produces collated report with confidence levels
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

→ Dispatches 2 research-agent agents in parallel
  (with model parameter if specified, otherwise sonnet)
→ Each agent independently explores:
  - Different entry points
  - Multiple sources (codebase, web, docs)
  - Different perspectives
→ Dispatches review-collation-agent (always haiku)
→ Produces collated report:
  - Common findings (high confidence)
  - Unique insights (worth knowing)
  - Divergences (needs clarification)
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

→ Dispatches 1 technical-writer and 1 code-agent in parallel
  (with model parameter if specified, otherwise haiku)
→ Each agent independently verifies against codebase
→ Dispatches review-collation-agent (always haiku)
→ Produces collated report with confidence levels
```

## Why Dual Verification?

**Problem:** Single agent can miss issues, hallucinate, or confirm biases.

**Solution:** Two independent agents catch what one misses.

**Confidence levels:**
- **VERY HIGH:** Both agents found → Act on this
- **MODERATE:** One agent found → Consider carefully
- **INVESTIGATE:** Agents disagree → User decides

**Example (research):**
```
Agent #1: "Auth uses JWT with 1-hour expiry"
Agent #2: "Auth uses JWT with 24-hour refresh tokens"

→ Collation: Both partially correct (access vs refresh)
→ Higher confidence understanding than single agent
```

## Integration with Other Commands

Execute workflow uses verify for batch verification:

```
/execute workflow:
  → Batch 1 (3 tasks)
  → /verify code (quality/standards)
  → /verify execute (plan adherence)
  → Fix all BLOCKING issues
  → Repeat for next batch
```

## Related Commands

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
- Dispatch table determines which agents to use
- Collation agent is always the same (generic)
- Confidence levels guide user decisions
- Agents cannot be trusted - that's why we use two
