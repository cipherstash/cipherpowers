# Code Review Request

Review code changes for production readiness.

Your task:

1. Review implementation
2. Compare against plan or requirements
3. Check code quality, architecture, testing
4. Categorize issues by severity
5. Assess production readiness

## What Was Implemented

{DESCRIPTION}

## Requirements

{REQUIREMENTS}

## Git Range

**Base:** {BASE_SHA}
**Head:** {HEAD_SHA}

```bash
git diff --stat {BASE_SHA}..{HEAD_SHA}
git diff {BASE_SHA}..{HEAD_SHA}
```

## Instructions

Use and follow the conducting-code-review skill.

Path: `${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md`
Tool: `Skill(skill: "cipherpowers:conducting-code-review")`

Use and follow the code-review-template to structure and format the review.

Template: `${CLAUDE_PLUGIN_ROOT}templates/code-review-template.md`

