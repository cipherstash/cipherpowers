# Commit

Use the code-commiter subagent to commit the changes.

<instructions>
  ## Instructions for commits

  1. Run pre-commit checks:
    - Run linters and formatters
    - Run tests if appropriate
    - Ensure code builds without errors

  2. Check which files are staged with `git status`

  3. If 0 files are staged, automatically add all modified and new files with `git add`

  4. Perform a `git diff` to understand what changes are being committed

  5. Analyze the diff to determine if multiple distinct logical changes are present

  6. If multiple distinct changes are detected, suggest breaking the commit into multiple smaller commits

  7. For each commit (or the single commit if not split), create a commit message using the conventional commit format
    - Follow conventional commits: `docs/practices/conventional-commits.md`
    - Follow git guidelines: `docs/practices/git-guidelines.md`
</instructions>
