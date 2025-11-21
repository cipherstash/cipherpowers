## Project TDD Standards

**Location:** `.claude/context/test-driven-development-start.md`

**Triggered by:** When `test-driven-development` skill loads (SkillStart hook)

This project uses:

- **Test framework:** Vitest
- **Test location:** `src/**/__tests__/*.test.ts`
- **Coverage requirement:** 80% line coverage minimum
- **Property testing:** Use fast-check for algorithms

### File Structure
```
src/
  components/
    Button/
      Button.tsx
      __tests__/
        Button.test.tsx
```

### Naming Convention
- Use `describe/it` blocks (not `test()`)
- Test names: "should [behavior] when [condition]"
- File naming: `{Component}.test.ts`

### Mocking Strategy
- Mock external services (APIs, databases)
- Do NOT mock internal modules (test real behavior)
- Use MSW for HTTP mocking

### RED-GREEN-REFACTOR
1. Write failing test first
2. Run test (verify it fails for right reason)
3. Write minimal code to pass
4. Run test (verify it passes)
5. Refactor (if needed)
6. Commit
