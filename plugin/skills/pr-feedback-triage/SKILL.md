---
name: pr-feedback-triage
description: Audit already-merged PRs for code review feedback that was never acted on: collect the comments, verify each concern against origin/main, then reconcile the survivors with Linear — filing new tickets, enhancing matched ones, merging duplicates. Use this whenever someone suspects review feedback got dropped, steamrolled, or forgotten, and asks things like "did we ever fix what X flagged", "did we ever action those comments", "which of those open threads are still real problems on main", "what review feedback fell through the cracks", "audit last sprint's merged PRs", or "turn leftover review comments into tickets". It applies to a single named PR just as much as to a whole time window, to any base branch, and whether or not the user mentions Linear or a tracker at all — wanting to know which concerns from a merged PR are still live is reason enough. Not for reviewing an unmerged diff yourself, listing threads on still-open PRs, making the changes a reviewer asked for on your own open PR, or tracker housekeeping unrelated to review comments.
when_to_use: when review feedback on merged PRs may have been dropped, when auditing whether reviewer concerns were actually fixed, when turning surviving review comments into tracker tickets
version: 1.0.0
---

# PR feedback triage

Review comments get written, the PR merges anyway, and the concern evaporates. This
skill finds those, checks which are genuinely still live, and makes sure each one has
exactly one Linear ticket.

The failure mode to guard against is a plausible-looking ticket for a problem that was
fixed three PRs ago. That wastes a human's time and erodes trust in the whole exercise
faster than missing an item does. **Verify before you file.** When you cannot verify,
say so in the plan rather than quietly assuming.

## Requirements

- `gh` CLI, authenticated. The collector script uses the GitHub GraphQL API.
- **The Linear MCP server must be connected.** Phases 1–3 work without it; Phase 4
  cannot. Its tools are usually deferred — load them with `ToolSearch` before Phase 4.
- Python 3.9+ for the collector.

## Workflow

Five phases: collect → triage → verify → reconcile → apply. Only the last one writes
anything. Track them with your todo tool; a stalled run should be obvious.

---

## Phase 1 — Collect

Resolve the window and the repo first. If the user said "the last sprint" or "since the
release", turn that into a date and **tell them what you resolved it to** — a silently
wrong window silently omits PRs.

```bash
python3 ${CLAUDE_PLUGIN_ROOT}skills/pr-feedback-triage/scripts/fetch_pr_feedback.py \
  --since 14d -o feedback.json
```

`--since` takes a span (`14d`, `3w`, `2m`) or an ISO date. Add `--until` for a closed
window, `--repo owner/name` to override the current repo, `--include-bots` if the user
explicitly wants Copilot's or CodeRabbit's comments.

**Sweep every base branch. Do not reach for `--base`.** When a user says "the PRs we
merged into main", they mean the work that reached main — not the subset whose merge
commit happened to target it. Repos that stage a release on a long-lived branch merge
their richest, most-reviewed PRs into *that* branch, and it lands on main later as one
squashed merge. Filtering by base drops exactly the feedback worth finding: on a real
run, `--base main` collapsed thirteen feedback-bearing PRs down to one, and the survivor
was the release commit. If you have a genuine reason to filter, run unfiltered first and
tell the user what the filter would have excluded.

Each item carries a stable id (`PR1234-T4`), an author, a permalink, and — for inline
threads — `path` and `line`. Use those ids everywhere downstream, so a plan line, a
ticket callout, and a verification note all refer to the same thing.

Two fields are traps:

- `severity_hint` is a regex guess from the comment's first line. Many teams don't
  prefix at all, so a `null` hint means nothing, and a comment prefixed `nit:` can still
  describe a real bug. Read the body; the hint only breaks ties.
- `is_resolved` and `is_outdated` are recorded but are **not** evidence. Plenty of teams
  never resolve threads, so a PR can merge with every thread still open — including the
  ones marked blocking. Judge from the code, never from the checkbox.

---

## Phase 2 — Triage

Keep an item if a reasonable engineer would want it tracked: **blockers, bugs, design
issues, documentation issues.** Drop praise, style bikeshedding, questions already
answered in a reply, and anything whose fix is smaller than the ticket describing it.

A `review_body` item is often a *summary review* bundling many distinct findings in
prose — "Second pass: 9 items not covered by the existing threads". Split it. Each
finding becomes its own item, id-suffixed (`PR1234-R2#3`), because they will get
different verification results and probably different tickets.

For each kept item record: a one-line title, a category (blocker / bug / design / docs),
the concrete claim being made, and where it says the problem lives.

When you're unsure whether something is minor, ask what happens if it's never fixed. If
the answer is "nothing", drop it.

---

## Phase 3 — Verify against origin/main

`git fetch origin` first — a stale ref invalidates the entire run.

An item survives only if the concern **still holds on `origin/main` today**. Read the
cited file there (search for the construct by name; code moves), and check
`git log origin/main -- <path>` since the PR's merge commit for a later fix. Note that a
long-lived PR often fixes its own review comments on later commits *before* merging, so
"no follow-up commit touched this file" does not mean "never fixed".

**Where the comment implies a check, run it.** This is what separates a trustworthy
verdict from a confident guess, and it is the step most easily skipped. "This test runs
in no CI job" → grep the workflow files. "The allowlist references a type that doesn't
exist" → grep for the type. "This task is broken" → run the task. Repo-native checks are
cheap and decisive. Prefer a command whose output you can quote over a paragraph of
reasoning about what the code probably does.

Verification is per-item and independent, so **fan it out across subagents** when there
are more than a handful. Give each the item, the repo path, and instructions to return a
verdict plus its evidence. Twenty items verified serially is a slow, context-hungry run
for no benefit.

Assign one of three verdicts and carry the evidence forward — the evidence is what makes
the plan reviewable:

- `still-live` — reproduced the concern on `origin/main`.
- `addressed` — found the fix. Drop it, but name the commit in the summary so the user
  can spot a wrong call. A reply saying "fixed" or "will do" is not a commit.
- `unverifiable` — you genuinely cannot tell: it needs a database, credentials, or
  runtime behaviour you can't exercise. **Do not drop these and do not quietly file them
  as if confirmed.** Surface them as their own group, and if one is filed anyway, say so
  in the ticket itself. A ticket that overstates its own confidence is worse than no
  ticket, because the next person trusts it.

---

## Phase 4 — Reconcile with Linear

### Items are not tickets

Group first, then search. Several findings that share a **root cause** belong in one
ticket, because they will be fixed in one sitting by one person: four separate defects
in the same CI script are one "harden this script" ticket, not four. Findings that share
only a *theme* ("docs are stale") but live in unrelated files are still one ticket if a
single pass fixes them all, and separate tickets if not.

The test is whether a reader would open one PR or several. Err toward fewer, larger
tickets — a board with forty single-comment tickets is a board nobody triages. But keep
a genuinely load-bearing bug (a CI gate that fails open, a query that errors) as its own
ticket even when its neighbours are cleanup, or it will be invisible inside a checklist.

When many findings come from one PR that already has a Linear ticket, consider filing
them as **children of that ticket** rather than as top-level siblings. It keeps the board
tidy and gives the parent the content its author never wrote.

### Search before you file

Search Linear for each surviving group before concluding it needs a new ticket.
Constrain by `updatedAt` to the same window plus a margin — a ticket filed in response
to this feedback can't predate the PR — and search on the *substance*, not the
comment's wording. Try the symbol name, the file path, and the failure mode separately;
one query rarely finds it.

Then, per group:

**Several plausible tickets.** They may be duplicates of each other. Pick the survivor —
oldest, most complete, or the one with real discussion on it — and fold the others in.
Read the duplicates fully before merging: two tickets sharing a keyword often describe
genuinely different problems, and merging those destroys information. On the loser, set
status to the workspace's existing `Duplicate` state and comment a link to the survivor.
Never delete.

**Exactly one plausible ticket.** Enhance rather than duplicate. Add what the feedback
contributes that the ticket lacks — the file and line, the reviewer's reasoning, the
verification evidence. Correct metadata that's now wrong. If the ticket already says
everything, leave it alone and record it as `no-change`; a no-op edit is noise on
someone's board.

**No ticket.** File one. Title as an assertion of the problem, not a restatement of the
comment. Body: what's wrong, where, why it matters, and the reviewer's own words quoted
with a link.

Two constraints on all Linear writes:

- **Use existing metadata only.** Never create a project, label, status, or team. Read
  what exists (`list_issue_labels`, `list_issue_statuses`, `list_projects`) and pick from
  it. If nothing fits, use nothing and say so in the plan.
- **Infer the team and project from the tickets you matched.** File alongside the work
  the feedback belongs to. If no ticket matched and the target is genuinely ambiguous,
  ask the user rather than guessing at their board's conventions.

---

## Phase 5 — Plan, confirm, apply

Present the plan and **stop**. Linear is other people's shared workspace; a bad dedup
call is visible to the whole team and tedious to unpick. Do not write until the user
approves, unless they said something like "just do it".

The plan is a table the user can scan, grouped by action:

```
CREATE  (3)
  PR1234-T1  Blocking   integration test suite runs in no CI job
             → Platform · no label matched
  ...
UPDATE  (2)
  PR1234-T6  ENG-455    add file:line + verification evidence
DUPLICATE (1)
  ENG-461 → ENG-455     both describe the same stale allowlist rows
NO CHANGE (4)   already fully captured
UNVERIFIABLE (2)   needs a database to confirm — your call
DROPPED (31)    addressed on main (12) · minor (19)
```

Show the counts even for dropped items. A user glancing at "dropped 31" and thinking
"that's too many" is the cheapest bug-catch in this whole workflow.

On approval, apply the writes. If one fails, keep going and report the failures at the
end — a half-applied plan the user knows about beats an aborted one they don't.

---

## The callout

Every ticket this skill creates or modifies gets this at the **top of the description**,
above the existing content:

```markdown
> [!note]
> Created by Claude as a result of triaging **<owner/repo>** for unaddressed PR feedback,
> triggered by **<user>**.
>
> Source feedback:
> - [PR #1234 — `ci/lint.sh`: allowlist rows for a type that no longer exists](<permalink>)
> - [PR #1234 — review summary, item 3](<permalink>)
```

`<owner/repo>` comes from the fetch output's `repo`. `<user>` is the git identity
(`git config user.name`) — the person who ran the triage, not the reviewer who wrote the
comment.

Link every source item that fed the ticket, using the item's permalink from the fetch
output. When you update a ticket that already carries a callout from a previous run,
extend its source list rather than stacking a second callout block.

## Reference

`references/linear-conventions.md` — the exact MCP calls for each write, callout
placement when a description already has one, and how to search Linear effectively.
Read it before Phase 4.
