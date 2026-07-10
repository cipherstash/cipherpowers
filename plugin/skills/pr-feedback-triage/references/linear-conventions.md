# Linear conventions for PR feedback triage

Read this before Phase 4. It covers searching, the exact write calls, and callout
placement. The Linear MCP tools are deferred — load them with `ToolSearch` first:

```
select:mcp__claude_ai_Linear__list_issues,mcp__claude_ai_Linear__get_issue,mcp__claude_ai_Linear__save_issue,mcp__claude_ai_Linear__save_comment,mcp__claude_ai_Linear__list_issue_labels,mcp__claude_ai_Linear__list_issue_statuses,mcp__claude_ai_Linear__list_teams,mcp__claude_ai_Linear__list_projects
```

Tool names vary with how the Linear MCP server is registered. If that selector returns
nothing, search by keyword (`ToolSearch "+linear issue"`) rather than assuming the
server is absent.

## Searching for an existing ticket

`list_issues` matches `query` against title and description. It is a *substring-ish*
match, not semantic — so one query per concept, not one query per item.

Search on the substance of the problem, from three angles. For a finding like "twelve
allowlist rows referencing a type that no longer exists, in `ci/lint.sh`":

| Angle | Example query |
|---|---|
| Symbol | the removed type's name |
| Path | `lint` |
| Failure mode | `allowlist` |

Constrain with `updatedAt` to the triage window plus a generous margin (a month), and
`includeArchived: true` — a closed-then-archived ticket still means the work was tracked.
Widen if you get nothing; a false "no ticket exists" produces a duplicate, which is the
most annoying outcome for the team.

Do not filter by team on the first pass. Feedback about a repo often has tickets on more
than one team's board, and you need to see that to infer where the new one belongs.

Read candidates with `get_issue` before deciding. A title match is not a content match.

## Inferring the target team and project

Take the team and project from the tickets you matched for that item, or for its
neighbours in the same PR. Concretely: if `ENG-455` covers a sibling finding from the
same PR and sits in the Platform team under project "Q3 Hardening", file there too.

If nothing matched anywhere, ask the user. Do not default to the first team in
`list_teams`.

## Writes

All issue writes go through `save_issue` — it creates when `id` is absent and updates
when present.

**Create.** Pass `team`, `title`, `description`, and `project` only if you inferred one.
Set `labels` only from names returned by `list_issue_labels`. Omit `priority` unless the
feedback states urgency; an invented priority is noise.

```
save_issue(team: "<inferred>", title: "<assertion of the problem>",
           description: "<callout>\n\n<body>", project: "<inferred, optional>",
           parentId: "<the PR's own ticket, when filing its findings as children>")
```

`parentId` accepts an issue identifier like `ENG-455`. Sub-issues survive the parent
closing, so parenting under an in-flight ticket is safe.

**Update.** Fetch the current description with `get_issue`, then write the full new
description back — `save_issue` replaces it wholesale, so read-modify-write or you will
destroy the existing content. Pass **only** `id` and `description`: `labels` replaces the
entire label set, so naming it on an update silently strips any label you didn't list.
Omitting a field leaves it alone, which is what you want for assignee, milestone, parent,
and priority on someone else's ticket.

**Mark a duplicate.** Two calls. Set the status to the workspace's existing duplicate
state (look it up in `list_issue_statuses` — it is commonly named `Duplicate`), and leave
a comment pointing at the survivor.

```
save_issue(id: "<loser>", state: "Duplicate")
save_comment(issueId: "<loser>", body: "Merged into <survivor identifier> during PR feedback triage. ...")
```

Never delete an issue and never rewrite a human's description content — you may only
prepend the callout and append new sections.

## Never create metadata

No new projects, labels, statuses, teams, or initiatives. If nothing in
`list_issue_labels` fits, apply no label and note it in the plan as `no label matched`.
A wrong label is worse than none: it pollutes someone's saved view.

## Callout placement

The callout goes at the very top of the description, before all existing content.

**Ticket has no callout** — prepend it, then a blank line, then the original description
unchanged.

**Ticket already has a callout from a previous run** — do not stack a second one. Extend
the existing `Source feedback:` list with any new item links, deduplicating by URL.
Leave the rest of the block alone.

**Ticket is being enhanced with new information** — the callout summarises provenance,
not the finding. Put the actual technical content in the body below, under a heading
like `## Additional detail from PR review`, quoting the reviewer and linking the
permalink.

Format:

```markdown
> [!note]
> Created by Claude as a result of triaging **owner/repo** for unaddressed PR feedback,
> triggered by **Jane Doe**.
>
> Source feedback:
> - [PR #1234 — `ci/lint.sh`: allowlist rows for a type that no longer exists](https://github.com/...#discussion_r...)
```

Link text should say what the feedback was, so the list is readable without clicking.
`[PR #1234 — comment](url)` tells a reader nothing.
