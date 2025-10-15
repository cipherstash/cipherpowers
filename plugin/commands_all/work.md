# Work

1. Use the rust-engineer subagent to implement the plan.
   - follow the instructions @claude/commands/code-rust.md

2. Use the code-comitter subagent to commit work
   - follow the instructions in @claude/commands/commit.md

3. Use the code-review subagent to review the code
   - follow the instructions in @.claude/commands/code-review.md

4. Use the rust-engineer subagent to implement the code review recommendations
   - follow the instructions @claude/commands/fix.md

5. Use the code-review subagent to review the code review recomendation implementation
   - follow the instructions in .claude/commands/fix-review.md

6. Repeat steps 4 & 5 until all feedback has been addressed
   - clarify any ambiguity or ask the user for help if implementation issues arise
   - repeat a maximum of 1-2 times
   - do not get caught in a loop

7. Use the doc-writer subagent to review the completed work and create a summary
   - follow the instructions in .claude/commands/summarise.md

8. Use the doc-writer subagent to review the project documentation
   - follow the instructions in .claude/commands/doc-review.md

9. Use the code-comitter subagent to commit documentation changes
   - follow the instructions in @claude/commands/commit.md

Always use the rust-engineer subagent to make code changes when we are interactive debugging.

<important>
  - Always follow instructions in referenced files precisely & exactly.
  - Use todos to keep track of the high-level steps in this workflow
</important>

