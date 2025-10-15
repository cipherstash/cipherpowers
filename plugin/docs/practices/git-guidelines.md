## Guidelines for git branch names

- **Descriptive**: Names should reflect the branch's purpose or task
- **Concise**: Keep names short and to the point
- **Consistent**: Separate words with hyphens for readability (kebab-case)

## Guidelines for commits

- **Verify before committing**: Ensure code is linted, builds correctly, and documentation is updated
- **Atomic commits**: Each commit should contain related changes that serve a single purpose
- **Split large changes**: If changes touch multiple concerns, split them into separate commits
- **Present tense, imperative mood**: Write commit messages as commands (e.g., "add feature" not "added feature")
- **Concise first line**: Keep the first line under 72 characters

## Guidelines for splitting commits

When analyzing the diff, consider splitting commits based on these criteria:

1. **Different concerns**: Changes to unrelated parts of the codebase
2. **Different types of changes**: Mixing features, fixes, refactoring, etc.
3. **File patterns**: Changes to different types of files (e.g., source code vs documentation)
4. **Logical grouping**: Changes that would be easier to understand or review separately
5. **Size**: Very large changes that would be clearer if broken down

## Important notes

- If specific files are already staged, only commit those files
- If no files are staged, stage all modified and new files
- Construct the commit message based on the changes detected
- Before committing, review the diff to identify if multiple commits would be more appropriate
- If suggesting multiple commits, stage and commit the changes separately
- Always review the commit diff to ensure the message matches the changes
