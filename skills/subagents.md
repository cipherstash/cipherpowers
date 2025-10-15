Always use the right sub-agent for the task

| Task                | Agent           | Instructions
| ------------------- | --------------- | -------------------------------- |
| Rust engineering    | rust-engineer   |  @claude/commands/code-rust.md
| Git operations      | code-committer  |  @claude/commands/commit.md
| Code review         | code-reviewer   |  @claude/commands/code-review.md
| Summary             | doc-writer      |  @claude/commands/summarise.md
| Doc review          | doc-writer      |  @claude/commands/doc-review.md

The subagent must:
 - use read tool to read the provided instructions
 - read entire file
 - follow the instructions exactly
 - find & use any skills relevant to the task for additional context
