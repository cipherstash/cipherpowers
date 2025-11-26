## Project-Specific Code Review Requirements

This file demonstrates convention-based context injection.

**Location:** `.claude/context/code-review-start.md`

**Triggered by:** Running `/cipherpowers:code-review` command (SlashCommandStart hook)

**Purpose:** Inject project-specific review requirements automatically.

---

### Additional Security Checks

For this project, code reviews MUST verify:

1. **Authentication:** All API endpoints require valid JWT
2. **Input Validation:** All user inputs use allowlist validation
3. **Rate Limiting:** Public endpoints have rate limits configured
4. **Logging:** No PII in application logs

### Performance Requirements

- Database queries: No N+1 patterns
- API response time: < 200ms for p95
- Memory usage: No leaks detected in tests

### Documentation

- Public APIs have JSDoc/TSDoc comments
- Complex algorithms have inline explanations
- Breaking changes noted in CHANGELOG.md

---

**To use:** Copy to `.claude/context/code-review-start.md` in your project.
