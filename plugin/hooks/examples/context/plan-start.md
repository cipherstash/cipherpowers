## Project Planning Template

**Location:** `.claude/context/plan-start.md`

**Triggered by:** Running `/cipherpowers:plan` command (SlashCommandStart hook)

Your implementation plan must include:

### Architecture Impact
- Which services/modules are affected?
- Any new dependencies introduced?
- Database schema changes required?

### API Surface
- New endpoints or breaking changes?
- Version bump needed?
- Backward compatibility strategy?

### Testing Strategy
- Unit test coverage target (80%+)
- Integration tests for new flows
- E2E tests for user-facing features

### Deployment Considerations
- Feature flags required?
- Migration scripts needed?
- Rollback strategy?

### Success Criteria
- What does "done" look like?
- How to verify it works?
- What metrics to monitor?
