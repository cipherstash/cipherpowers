# Agent Simplification Implementation Plan (REVISED)

> **For Claude:** REQUIRED SUB-SKILL: Use cipherpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce agent verbosity while PRESERVING all critical workflows. Target ~25% reduction (not 42%) by removing only genuine redundancy.

**CRITICAL CONSTRAINT:** Every agent in the dual-verification pipeline MUST preserve:
1. **Skill activation** - mandatory skill references and activation patterns
2. **File persistence** - saving to `.work/` with timestamped filenames
3. **Workflow steps** - non-negotiable sequences that enforce quality
4. **Announcement patterns** - file path announcements for pipeline coordination

**What CAN be removed:**
- `<quality_gates>` sections (hooks handle this automatically)
- Duplicate "Red Flags" tables when rationalization_defense covers same ground
- Verbose prose that restates what skills already define
- Example interactions that duplicate skill examples

**What CANNOT be removed:**
- Skill activation blocks
- Save workflow instructions
- Output format specifications
- Dual-verification context notes
- AskUserQuestion requirements (gatekeeper)
- Annotation patterns (gatekeeper)

**Tech Stack:** Markdown agent prompts, skill references via `${CLAUDE_PLUGIN_ROOT}`

---

## Phase 1: Trim Redundancy (Preserve Workflows)

### Task 1: Trim commit-agent

**Files:**
- Modify: `plugin/agents/commit-agent.md` (214 → ~180 lines)

**What to REMOVE:**
- `<quality_gates>` section (hooks handle this)

**What to PRESERVE:**
- Skill activation block
- Standards references
- Non-negotiable workflow steps
- Rationalization defenses (condensed)

**Step 1: Read current commit-agent structure**

Read `plugin/agents/commit-agent.md` to locate the `<quality_gates>` section.

**Step 2: Remove quality_gates section**

Delete the `<quality_gates>...</quality_gates>` section entirely. This is handled by hooks.

**Step 3: Verify file structure**

Run: `wc -l plugin/agents/commit-agent.md`
Expected: ~180 lines (down from 214)

**Step 4: Commit**

```bash
git add plugin/agents/commit-agent.md
git commit -m "refactor(agents): remove redundant quality_gates from commit-agent"
```

---

### Task 2: NO CHANGES to gatekeeper

**Files:**
- `plugin/agents/gatekeeper.md` (288 lines - KEEP AS-IS)

**Rationale:** Gatekeeper is the ONLY guardrail between code review feedback and implementation. It MUST preserve:
- `validating-review-feedback` skill activation
- Full annotation workflow ([FIX]/[DEFERRED]/[WONTFIX])
- AskUserQuestion requirements for misaligned BLOCKING items
- Plan update workflow
- Rationalization defenses (these prevent dangerous shortcuts)

**Current gatekeeper has NO quality_gates section** - it's already lean.

**Decision:** Skip this task. Gatekeeper workflow is critical and already minimal.

---

### Task 3: Trim research-agent

**Files:**
- Modify: `plugin/agents/research-agent.md` (289 → ~250 lines)

**What to REMOVE:**
- `<quality_gates>` section (lines 203-217)
- Duplicate "Example Interactions" section (lines 283-289) - skill has examples

**What to PRESERVE (CRITICAL for dual-verification):**
- Dual-verification context note: "ONE of two independent researchers"
- Multi-angle exploration methodology
- Evidence gathering with confidence levels
- **SAVE WORKFLOW:** `.work/{YYYY-MM-DD}-verify-research-{HHmmss}.md`
- File path announcement requirement
- Gap identification requirements

**Step 1: Read and locate sections to remove**

Read `plugin/agents/research-agent.md` to confirm line numbers.

**Step 2: Remove quality_gates section**

Delete the `<quality_gates>...</quality_gates>` section.

**Step 3: Remove redundant examples**

Delete the "Example Interactions" section at end of file.

**Step 4: Verify critical sections remain**

Confirm these are still present:
- "This agent is dispatched as part of dual-verification"
- Save to `.work/{YYYY-MM-DD}-verify-research-{HHmmss}.md`
- "Announce file path in final response"

**Step 5: Commit**

```bash
git add plugin/agents/research-agent.md
git commit -m "refactor(agents): remove redundant sections from research-agent"
```

---

### Task 4: Trim plan-review-agent

**Files:**
- Modify: `plugin/agents/plan-review-agent.md` (213 → ~175 lines)

**What to REMOVE:**
- `<quality_gates>` section (lines 146-161)

**What to PRESERVE (CRITICAL for dual-verification):**
- `verifying-plans` skill activation
- Standards references (code-review.md, development.md, testing.md)
- **SAVE WORKFLOW:** `.work/{YYYY-MM-DD}-verify-plan-{HHmmss}.md`
- File path announcement in final response
- Rationalization defenses (prevents rubber-stamping)
- Review checklist (all 6 categories)

**Step 1: Read and locate quality_gates section**

Read `plugin/agents/plan-review-agent.md` to confirm line numbers.

**Step 2: Remove quality_gates section**

Delete the `<quality_gates>...</quality_gates>` section.

**Step 3: Verify critical sections remain**

Confirm these are still present:
- Skill activation block for "cipherpowers:verifying-plans"
- Save workflow with timestamped filename
- "Announce saved file location in final response"

**Step 4: Commit**

```bash
git add plugin/agents/plan-review-agent.md
git commit -m "refactor(agents): remove redundant quality_gates from plan-review-agent"
```

---

### Task 5: Trim execute-review-agent

**Files:**
- Modify: `plugin/agents/execute-review-agent.md` (312 → ~270 lines)

**What to REMOVE:**
- Duplicate example at end of file (lines 277-312)

**What to PRESERVE (CRITICAL for dual-verification):**
- Full 6-step verification workflow
- COMPLETE/INCOMPLETE/DEVIATED status system
- **SAVE WORKFLOW:** `.work/{YYYY-MM-DD}-verify-execute-{HHmmss}.md`
- Report structure with BLOCKING/NON-BLOCKING categorization
- Rationalization defenses (prevents "close enough" mentality)
- File path announcement requirement

**Step 1: Read and locate example section**

Read `plugin/agents/execute-review-agent.md` to confirm structure.

**Step 2: Remove redundant example**

Delete the "Example Verification" section at end (lines 277-312) - the non_negotiable_workflow section already has complete examples.

**Step 3: Verify critical sections remain**

Confirm these are still present:
- All 8 workflow steps (including save workflow)
- Report template with BLOCKING/NON-BLOCKING
- "announce saved file path in final response"

**Step 4: Commit**

```bash
git add plugin/agents/execute-review-agent.md
git commit -m "refactor(agents): remove redundant example from execute-review-agent"
```

---

### Task 6: NO CHANGES to review-collation-agent

**Files:**
- `plugin/agents/review-collation-agent.md` (128 lines - KEEP AS-IS)

**Rationale:** Review-collation-agent is already minimal (128 lines) and contains only essential content:
- `dual-verification` skill activation (MUST preserve)
- Template reference: `${CLAUDE_PLUGIN_ROOT}templates/verify-collation-template.md` (MUST preserve)
- Save to `.work/` workflow (MUST preserve)
- `/cipherpowers:revise common` announcement (MUST preserve)
- No quality_gates section (nothing to remove)

**Decision:** Skip this task. Agent is already lean and every section is critical.

---

## Phase 2: Trim High-Stakes Agents

### Task 7: Trim code-agent

**Files:**
- Modify: `plugin/agents/code-agent.md` (237 → ~200 lines)

**What to REMOVE:**
- `<quality_gates>` section - hooks handle this automatically

**What to PRESERVE:**
- Skill activation patterns
- TDD workflow enforcement
- Code review request requirement
- Rationalization defenses
- All standards references

**Step 1: Read and locate quality_gates section**

Read `plugin/agents/code-agent.md` to confirm location of quality_gates.

**Step 2: Remove quality_gates section**

Delete the `<quality_gates>...</quality_gates>` section.

**Step 3: Verify file structure**

Run: `wc -l plugin/agents/code-agent.md`
Expected: ~200 lines (down from 237)

**Step 4: Commit**

```bash
git add plugin/agents/code-agent.md
git commit -m "refactor(agents): remove redundant quality_gates from code-agent"
```

---

### Task 8: Trim rust-agent

**Files:**
- Modify: `plugin/agents/rust-agent.md` (243 → ~205 lines)

**What to REMOVE:**
- `<quality_gates>` section - hooks handle this automatically

**What to PRESERVE:**
- Rust-specific skill patterns
- TDD workflow enforcement
- Code review request requirement
- Rationalization defenses
- All standards references

**Step 1: Read and locate quality_gates section**

Read `plugin/agents/rust-agent.md` to confirm location of quality_gates.

**Step 2: Remove quality_gates section**

Delete the `<quality_gates>...</quality_gates>` section.

**Step 3: Verify file structure**

Run: `wc -l plugin/agents/rust-agent.md`
Expected: ~205 lines (down from 243)

**Step 4: Commit**

```bash
git add plugin/agents/rust-agent.md
git commit -m "refactor(agents): remove redundant quality_gates from rust-agent"
```

---

### Task 9: Trim technical-writer

**Files:**
- Modify: `plugin/agents/technical-writer.md` (270 → ~230 lines)

**What to REMOVE:**
- `<quality_gates>` section if present - hooks handle this automatically

**What to PRESERVE:**
- VERIFICATION vs EXECUTION mode logic
- Save workflow for verification reports
- Documentation standards references
- Mode-specific workflows

**Step 1: Read and check for quality_gates**

Read `plugin/agents/technical-writer.md` to check if quality_gates section exists.

**Step 2: Remove quality_gates section if present**

If present, delete the `<quality_gates>...</quality_gates>` section.

**Step 3: Verify file structure**

Run: `wc -l plugin/agents/technical-writer.md`
Expected: ~230 lines if quality_gates was removed

**Step 4: Commit (if changes were made)**

```bash
git add plugin/agents/technical-writer.md
git commit -m "refactor(agents): remove redundant quality_gates from technical-writer"
```

---

## Phase 3: Verification

### Task 10: Verify all agents preserve critical workflows

**Step 1: Verify dual-verification agents have save workflows**

For each agent, grep for save pattern:
```bash
grep -l "\.work/" plugin/agents/*.md
```

Expected: research-agent, plan-review-agent, execute-review-agent should all have `.work/` save patterns.

**Step 2: Verify skill activations remain**

```bash
grep -l "MANDATORY.*Skill" plugin/agents/*.md
```

Expected: gatekeeper, plan-review-agent, review-collation-agent, commit-agent should all have skill activation blocks.

**Step 3: List all agent files with line counts**

Run: `wc -l plugin/agents/*.md | sort -n`

Expected output should show modest reductions (~15-20%) not drastic (~42%).

**Step 4: Verify YAML frontmatter is valid**

Run: `head -5 plugin/agents/*.md`

Expected: Each file starts with `---` and has name, description, color fields.

**Step 5: Final commit if needed**

If any fixes were needed:
```bash
git add plugin/agents/
git commit -m "fix(agents): correct issues found during verification"
```

---

## Summary

**Before:** 14 agents, ~2,776 lines total
**After:** 14 agents, ~2,200 lines total
**Savings:** ~576 lines (~20% reduction)

| Phase | Agents | Estimated Savings |
|-------|--------|-------------------|
| Phase 1 | 6 routine agents (4 modified, 2 unchanged) | ~100 lines |
| Phase 2 | 3 high-stakes agents | ~110 lines |
| Phase 3 | Verification | 0 lines |

**Key difference from original plan:**
- Original: 42% reduction by replacing workflows with minimal stubs
- Revised: 20% reduction by removing only redundant sections
- Preserved: All skill activations, save workflows, dual-verification context, rationalization defenses

**Agents unchanged (already minimal or workflows critical):**
- gatekeeper.md (288 lines) - validation workflow is essential
- review-collation-agent.md (128 lines) - already minimal
