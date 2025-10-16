---
name: Development Workflow
description: Sequence development work in small increments through analysis, planning, implementation, verification, review, and summarization with organized work directories.
when_to_use: when organizing and tracking development work from initial research through final documentation
applies_to: all projects
related_practices: git-guidelines.md, code-review.md
version: 1.0.0
---

# Development Workflow

## Overview

Development work is sequenced in small increments, roughly equivalent to an "epic" or "feature".

At a high level, the flow is:

- **analysis**: research, analyze and recommend potential approaches or solutions
- **planning**: create detailed task breakdown
- **implementation**: execute and implement tasks
- **verification**: test and verify the implementation
- **review**: code and documentation review
- **summarise**: document implementation, experiments and lessons (including discarded approaches)

The flow is not necessarily linear - we react and respond as new information, constraints and opportunities arise.
Analysis and planning may be merged into a single step if the work is well-understood or simple to implement. For example, updating dependencies or fixing an issue does not need the same level of research and analysis as adding a completely new feature.

In practice, implementation and verification are an iterative feedback loop. We implement and verify each task in turn.

## Work directory

Each unit of work is given a short, descriptive `name`.
The work `name` may also be used as the name of any associated git branch.
Each work unit has an associated work directory (customize location for your project).

Work directories are named with:
  - descriptive name
  - kebab-case
  - maximum of 5 words
  - date prefix (using ISO 8601) for clarity and ease of collation: `{YYYY-MM-DD}-{name}`

The outputs of each step of the development workflow is saved in the work directory.

Workflow step naming convention:

| Step            | Filename                 |
|-----------------|--------------------------|
| analysis        | analysis.md              |
| planning        | plan.md                  |
| implementation  | -                        |
| verification    | -                        |
| review          | `{YYYY-MM-DD}-review`    |
| summarise       | summary.md               |

The file list is not exhaustive, other files can be generated if appropriate.
The output of implementation and verification should generally be working code with tests.


<examples>
## Examples

### Example work directory names

  - 2025-10-03-user-authentication
  - 2025-03-15-password-reset-flow
  - 2025-03-16-user-profile-dashboard
  - 2025-03-17-api-rate-limiting


### Example work directory structure
```
docs/work
├── 2025-10-03-user-authentication
    └── 2025-10-03-review.md
    └── analysis.md
    └── plan.md
    └── summary.md
```

</examples>
