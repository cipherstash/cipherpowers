# Commit

Create a commit following conventional commit standards.

<instructions>
  ## Instructions

  1. Run pre-commit checks per @docs/practices/testing.md
     - Use `mise run check` command
  2. Check staged files with `git status`
  3. If no files staged, stage relevant files with `git add`
  4. Review changes with `git diff --staged`
  5. Analyze for multiple logical changes
     - If multiple distinct changes, suggest splitting commits
  6. Create commit message following @docs/practices/conventional-commits.md
  7. Reference @docs/practices/git-guidelines.md for git best practices
</instructions>
