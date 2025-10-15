# Work Documentation Structure Decision

**Date:** 2025-10-14
**Status:** Needs team discussion
**Context:** Migrating commands from previous project to CipherPowers plugin

## Background

While auditing commands from `/commands_all` for plugin integration, we discovered that many commands assume a "Current Active Work Directory" pattern (e.g., `mise run work:active`, `mise run work:recent`). This pattern doesn't fit our current workflow.

**Our current state:**
- Using Linear for work/issue tracking
- Want markdown files (analysis, plans, summaries) in the project for rapid context sharing
- No established pattern for where these docs should live

## The Core Question

**How should we organize project history and work documentation to balance:**
- Linear as source of truth for work tracking
- Markdown files for rich context (analysis, plans, summaries, decisions)
- Easy discoverability for team members
- Preservation of context over time

## Options Analysis

### Option A: Dated Documentation Directory

**Structure:**
```
docs/work/
  2025-10-14-user-auth-analysis.md
  2025-10-14-user-auth-plan.md
  2025-10-15-user-auth-summary.md
  2025-10-16-api-refactor-analysis.md
```

**Pros:**
- Simple chronological history
- Easy to find recent context (`ls -lt` shows newest first)
- No complex directory structure to maintain
- Natural fit for commands like `/summarise` that capture point-in-time work

**Cons:**
- No connection to code locations
- Hard to find "all docs about user auth" later without grep
- Filenames get verbose with date + topic
- No clear separation of active vs. completed work

**Best for teams that:** Work on many small changes, value recency over topical grouping

---

### Option B: Feature/Topic-Based Structure

**Structure:**
```
docs/work/
  user-authentication/
    analysis.md
    plan.md
    summary.md
  api-refactor/
    analysis.md
    plan.md
    summary-2025-10-14.md
```

**Pros:**
- All related docs in one place
- Clear topical organization
- Easy to navigate by feature
- Natural for longer-running initiatives

**Cons:**
- Need to decide "what's a feature" (naming/scope decisions)
- Mixing active and completed work in same directory
- Harder to see "what's recent" across all work
- May accumulate stale directories over time

**Best for teams that:** Work on larger features over time, frequently reference past decisions on a topic

---

### Option C: Co-located with Code (ADR-style)

**Structure:**
```
src/auth/
  docs/
    2025-10-14-analysis.md
    implementation-notes.md

docs/decisions/
  001-use-jwt-tokens.md
  002-password-hashing-strategy.md
```

**Pros:**
- Context lives near relevant code
- Survives refactoring better (moves with code)
- Follows ADR (Architecture Decision Record) pattern - proven approach
- Developers find context naturally when working in that area

**Cons:**
- Project-wide plans don't fit cleanly (where do they go?)
- Documentation scattered across codebase
- May clutter code directories
- Harder to get "all recent work" view

**Best for teams that:** Value context near code, make lots of architectural decisions, want docs to move with refactoring

---

### Option D: Hybrid - Linear ID + Project Docs

**Structure:**
```
docs/work/
  CIP-123-user-auth/
    analysis.md
    plan.md
    summary.md
  CIP-145-api-refactor/
    analysis.md
    plan.md
```

**Pros:**
- Direct link to Linear ticket (single source of truth)
- Organized by work item
- Easy cross-reference between systems
- Clear what work corresponds to what Linear issue

**Cons:**
- Requires Linear ticket upfront (may not fit exploratory work)
- Harder to browse without knowing ticket ID
- May have orphaned directories if tickets change/close
- Tight coupling between Linear and docs structure

**Best for teams that:** Always work from Linear tickets, want tight integration between tracking and documentation

---

## Related Command Audit Results

### Commands to Keep (need updates for plugin use)

| Command | Current State | Needs |
|---------|---------------|-------|
| `analyse.md` | References `mise run work:recent` | Update to flexible approach for plugin |
| `fix-review.md` | References `mise run review:active` | Update active review discovery |
| `fix.md` | References `mise run review:active` | Update active review discovery |
| `recommend.md` | References `mise run work:recent` | Update to flexible approach |

### Commands to Remove

- `code-rust.md` - Redundant with language-agnostic `execute.md`
- `prime.md` - Near-duplicate of `analyse.md`
- All 5 duplicates in `/commands_all` that exist in `/commands/`

### Commands to Convert to Skills

- `doc-review.md` → Reusable process knowledge about documentation review
- `summarise.md` → *Maybe* - could be skill (process) or command (workflow). Depends on chosen structure.

## Key Decisions Needed

### 1. Work Documentation Structure (Primary Decision)

**Question:** Which option (A, B, C, D, or hybrid) fits our team's workflow?

**Consider:**
- How do we typically work? (Many small changes vs. longer features)
- What do we reference more often? (Recent work vs. specific topics)
- Do we always have Linear tickets before starting work?
- How important is having docs near code vs. centralized?

### 2. Command Updates (Dependent on #1)

Once we decide on documentation structure:
- Update `analyse.md` to find recent work
- Update `fix.md` / `fix-review.md` to find active reviews
- Update `recommend.md` to suggest next work
- Decide if `summarise.md` is skill or command

### 3. Path Constants (Plugin Infrastructure)

- Establish `${CIPHERPOWERS_ROOT}` pattern (like superpowers uses `${SUPERPOWERS_SKILLS_ROOT}`)
- Update all command path references consistently
- Document plugin path resolution

### 4. Subagents Skill (Blocked by above)

After commands are organized, create the subagents skill that:
- Maps task types to specialized agents
- References clean command structure
- Uses consistent path constants
- Follows superpowers skill structure

## Questions for Team Discussion

1. **What's our typical work cadence?**
   - Many small changes daily? → Consider Option A (dated)
   - Longer features over days/weeks? → Consider Option B (topic-based)

2. **How do we reference past work?**
   - "What did we do last week?" → Option A
   - "What was our decision on auth?" → Option B or C

3. **Do we always create Linear tickets before starting work?**
   - Yes, always → Option D viable
   - No, sometimes exploratory → Option D problematic

4. **How important is context-near-code?**
   - Very important → Option C
   - Central docs fine → Options A, B, or D

5. **Who generates these docs?**
   - Commands (automated) → Favor simpler structures (A, D)
   - Developers (manual) → Can handle complexity (B, C)

## Next Steps

1. **Team discussion** on work documentation structure
2. **Decide** on option (A, B, C, D, or hybrid)
3. **Update** plugin documentation (CLAUDE.md) with chosen pattern
4. **Migrate** commands based on decisions
5. **Create** subagents skill with clean references

## References

- Original project commands: `/Users/tobyhede/src/cipherpowers/commands_all/`
- Current plugin commands: `/Users/tobyhede/src/cipherpowers/commands/`
- Superpowers skills pattern: `${SUPERPOWERS_SKILLS_ROOT}/skills/`
- ADR pattern: https://adr.github.io/
