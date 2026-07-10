#!/usr/bin/env python3
"""Collect review feedback from merged PRs in a time window.

Emits normalised JSON on stdout so the triaging agent never has to hand-roll
GraphQL. Every feedback item gets a stable id (e.g. PR1234-T4) that downstream
steps use to refer to it in plans, tickets, and callouts.

Usage:
    fetch_pr_feedback.py --since 14d
    fetch_pr_feedback.py --since 2026-06-01 --until 2026-07-01
    fetch_pr_feedback.py --since 30d --repo owner/name --include-bots

Requires the `gh` CLI, authenticated.
"""

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timedelta, timezone

# Authors whose comments are almost never actionable review feedback. Matched
# exactly against the login, never as a substring -- a human login can share a
# prefix with a bot's (e.g. a user named `coder...` vs `coderabbitai`).
DEFAULT_BOT_LOGINS = {
    "copilot-pull-request-reviewer",
    "copilot",
    "github-actions",
    "dependabot",
    "changeset-bot",
    "coderabbitai",
    "sonarcloud",
    "codecov",
    "vercel",
    "netlify",
}

# Many teams prefix review comments with a severity. Capturing it here is a
# hint for the triager, not a verdict: an unprefixed comment can still be a
# blocker, and a "nit" prefix on a real bug is still a real bug.
SEVERITY_PATTERNS = [
    ("blocker", r"^\W*(blocking|blocker|must[- ]fix|required|p0)\b"),
    ("should-fix", r"^\W*(should[- ]fix|important|p1)\b"),
    ("minor", r"^\W*(nit|nitpick|minor|style|optional|suggestion|praise|typo|p3)\b"),
    ("question", r"^\W*(question|q:|curious)\b"),
]

PR_SEARCH = """
query($q: String!, $after: String) {
  search(query: $q, type: ISSUE, first: 50, after: $after) {
    pageInfo { hasNextPage endCursor }
    nodes {
      ... on PullRequest {
        number title url mergedAt baseRefName
        author { login __typename }
        mergeCommit { oid }
      }
    }
  }
}
"""

PR_DETAIL = """
query($owner: String!, $name: String!, $number: Int!, $tAfter: String) {
  repository(owner: $owner, name: $name) {
    pullRequest(number: $number) {
      reviews(first: 100) {
        nodes {
          author { login __typename }
          state body url submittedAt
        }
      }
      comments(first: 100) {
        nodes { author { login __typename } body url createdAt }
      }
      reviewThreads(first: 100, after: $tAfter) {
        pageInfo { hasNextPage endCursor }
        nodes {
          isResolved isOutdated path line originalLine
          comments(first: 50) {
            nodes { author { login __typename } body url createdAt }
          }
        }
      }
    }
  }
}
"""


def gh_graphql(query, **variables):
    cmd = ["gh", "api", "graphql", "-f", f"query={query}"]
    for key, value in variables.items():
        if value is None:
            continue
        flag = "-F" if isinstance(value, int) else "-f"
        cmd += [flag, f"{key}={value}"]
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if proc.returncode != 0:
        sys.exit(f"gh graphql failed:\n{proc.stderr.strip()}")
    payload = json.loads(proc.stdout)
    if "errors" in payload:
        sys.exit("graphql errors: " + json.dumps(payload["errors"], indent=2))
    return payload["data"]


def resolve_repo(explicit):
    if explicit:
        return explicit
    proc = subprocess.run(
        ["gh", "repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"],
        capture_output=True, text=True,
    )
    if proc.returncode != 0:
        sys.exit("could not determine repo; pass --repo owner/name")
    return proc.stdout.strip()


def parse_when(value):
    """Accept an ISO date or a relative span like 14d / 3w / 6m."""
    if value is None:
        return None
    m = re.fullmatch(r"(\d+)\s*([dwmy])", value.strip(), re.I)
    if m:
        n, unit = int(m.group(1)), m.group(2).lower()
        days = {"d": 1, "w": 7, "m": 30, "y": 365}[unit] * n
        return (datetime.now(timezone.utc) - timedelta(days=days)).date().isoformat()
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00")).date().isoformat()
    except ValueError:
        sys.exit(f"could not parse date/span: {value!r} (try 14d or 2026-06-01)")


def severity_hint(body):
    first_line = (body or "").strip().splitlines()[0] if (body or "").strip() else ""
    probe = first_line.lower()
    for label, pattern in SEVERITY_PATTERNS:
        if re.search(pattern, probe, re.I):
            return label
    return None


def is_bot(author, include_bots):
    if include_bots:
        return False
    if not author:
        return True
    if author.get("__typename") == "Bot":
        return True
    return author.get("login", "").lower() in DEFAULT_BOT_LOGINS


def author_login(author):
    return (author or {}).get("login", "ghost")


def collect_pr(owner, name, pr, include_bots):
    threads, cursor = [], None
    reviews = comments = None
    while True:
        data = gh_graphql(
            PR_DETAIL, owner=owner, name=name, number=pr["number"], tAfter=cursor
        )["repository"]["pullRequest"]
        if reviews is None:
            reviews, comments = data["reviews"]["nodes"], data["comments"]["nodes"]
        threads += data["reviewThreads"]["nodes"]
        page = data["reviewThreads"]["pageInfo"]
        if not page["hasNextPage"]:
            break
        cursor = page["endCursor"]

    items = []

    for idx, thread in enumerate(threads, 1):
        nodes = thread["comments"]["nodes"]
        if not nodes:
            continue
        head = nodes[0]
        if is_bot(head["author"], include_bots):
            continue
        items.append({
            "id": f"PR{pr['number']}-T{idx}",
            "kind": "review_thread",
            "author": author_login(head["author"]),
            "url": head["url"],
            "path": thread["path"],
            "line": thread["line"] or thread["originalLine"],
            # Recorded for context only. Do not treat as evidence the concern
            # was addressed -- many teams never resolve threads at all.
            "is_resolved": thread["isResolved"],
            "is_outdated": thread["isOutdated"],
            "severity_hint": severity_hint(head["body"]),
            "body": head["body"],
            "replies": [
                {"author": author_login(c["author"]), "body": c["body"]}
                for c in nodes[1:]
                if not is_bot(c["author"], include_bots)
            ],
        })

    for idx, review in enumerate(reviews, 1):
        if is_bot(review["author"], include_bots) or not (review["body"] or "").strip():
            continue
        items.append({
            "id": f"PR{pr['number']}-R{idx}",
            # A summary review body often bundles several distinct findings.
            # The triager is expected to split it into separate items.
            "kind": "review_body",
            "author": author_login(review["author"]),
            "url": review["url"],
            "path": None,
            "line": None,
            "state": review["state"],
            "severity_hint": severity_hint(review["body"]),
            "body": review["body"],
            "replies": [],
        })

    for idx, comment in enumerate(comments, 1):
        if is_bot(comment["author"], include_bots) or not (comment["body"] or "").strip():
            continue
        items.append({
            "id": f"PR{pr['number']}-C{idx}",
            "kind": "pr_comment",
            "author": author_login(comment["author"]),
            "url": comment["url"],
            "path": None,
            "line": None,
            "severity_hint": severity_hint(comment["body"]),
            "body": comment["body"],
            "replies": [],
        })

    return {
        "number": pr["number"],
        "title": pr["title"],
        "url": pr["url"],
        "merged_at": pr["mergedAt"],
        "base": pr["baseRefName"],
        "merge_commit": (pr.get("mergeCommit") or {}).get("oid"),
        "author": author_login(pr["author"]),
        "items": items,
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--since", required=True, help="ISO date or span (14d, 3w, 2m)")
    ap.add_argument("--until", help="ISO date; defaults to now")
    ap.add_argument("--repo", help="owner/name; defaults to the current repo")
    ap.add_argument("--base", help="only PRs merged into this branch, e.g. main")
    ap.add_argument("--include-bots", action="store_true")
    ap.add_argument("-o", "--out", help="write JSON here instead of stdout")
    args = ap.parse_args()

    repo = resolve_repo(args.repo)
    owner, name = repo.split("/", 1)
    since, until = parse_when(args.since), parse_when(args.until)

    merged = f"merged:>={since}" + (f" merged:<={until}" if until else "")
    query = f"repo:{repo} is:pr is:merged {merged}"
    if args.base:
        query += f" base:{args.base}"

    prs, cursor = [], None
    while True:
        page = gh_graphql(PR_SEARCH, q=query, after=cursor)["search"]
        prs += [n for n in page["nodes"] if n]
        if not page["pageInfo"]["hasNextPage"]:
            break
        cursor = page["pageInfo"]["endCursor"]

    collected = [collect_pr(owner, name, pr, args.include_bots) for pr in prs]
    collected = [pr for pr in collected if pr["items"]]
    collected.sort(key=lambda p: p["number"])

    result = {
        "repo": repo,
        "window": {"since": since, "until": until or "now"},
        "prs_in_window": len(prs),
        "prs_with_feedback": len(collected),
        "total_items": sum(len(pr["items"]) for pr in collected),
        "prs": collected,
    }

    text = json.dumps(result, indent=2)
    if args.out:
        with open(args.out, "w") as fh:
            fh.write(text + "\n")
        print(
            f"{result['total_items']} feedback items from "
            f"{result['prs_with_feedback']}/{result['prs_in_window']} merged PRs "
            f"→ {args.out}",
            file=sys.stderr,
        )
    else:
        print(text)


if __name__ == "__main__":
    main()
