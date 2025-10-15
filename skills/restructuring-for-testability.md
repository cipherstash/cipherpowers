---
name: Recognizing When to Restructure for Testability
description: Pattern recognition for architectural testing blockers that require restructuring before tests can be written
when_to_use: when hitting testing blockers ("can't test without GPU/rendering"), dismissing user bug reports as "works fine", or finding architecture prevents headless/isolated testing
version: 1.0.0
---

# Recognizing When to Restructure for Testability

## Overview

Sometimes you can't write tests because the architecture makes testing impossible. Recognizing this pattern is critical - the solution isn't to skip tests or add complex mocking, it's to restructure first.

**Core principle:** Architecture can block testing. When testing requires GPU/rendering/devices for business logic, restructure to separate concerns before writing tests.

**This is a pattern-recognition skill, not a discipline-enforcement skill.** When explicitly aware of the choice, agents consistently choose restructuring. The challenge is **recognizing** when restructuring is needed rather than dismissing symptoms or continuing without tests.

---

## Recognition Triggers

### Red Flag Symptoms

**Testing blockers** (can't write tests):
- ❌ "Can't test this without rendering context"
- ❌ "Can't test this without GPU"
- ❌ "Can't test this without network/database/filesystem"
- ❌ "Need full application to test business logic"
- ❌ "Testing requires visual inspection"

**Dismissing symptoms** (won't recognize need):
- ❌ "It works fine" (ignoring user bug reports)
- ❌ "Manual testing is thorough" (doesn't scale, misses edge cases)
- ❌ "User error" (dismissing symptoms instead of investigating)
- ❌ "I tested it locally" (works on my machine)

**Continuing without tests** (ignoring foundation problems):
- ❌ "Testing can come later" (it never does)
- ❌ Feature roadmap prioritized over fixing broken foundation
- ❌ Building on questionable physics/logic without verification
- ❌ Accumulating technical debt instead of pausing to fix

### What This Looks Like in Practice

**BattleSpace history** (real example):

**Phase 1: Dismissing symptoms**
```
User: "Time scale slider not working, objects barely moving at max speed"
Agent: "Works fine for me, probably user error"

User: "No seriously, it's broken - motion is imperceptible"
Agent: "I tested it, simulation is running correctly"

Reality: Critical accumulator capping bug
  - 99.99994% of simulation time discarded
  - Physics running at 0.00006% of intended speed
  - Agent dismissed symptoms instead of investigating
```

**Phase 2: Continuing without tests**
```
Physics tests: Failing (0.24% energy drift, 11.85% orbital drift)
Movement tests: Failing (arrival detection, state transitions broken)

Agent: Continued building features on broken foundation
User: "We need to pause and add testing infrastructure"

Reality: User intervention required to force testing work
  - Agent didn't recognize foundation was broken
  - Kept adding features to unstable base
  - Dismissed test failures as "edge cases"
```

**Phase 3: Architectural blocker** (this is where restructuring decision happens)
```
Agent starts testing work, immediately hits blocker:
  - SpaceSimulationPlugin mixes physics + rendering + UI + input
  - Testing requires full application (window, GPU, input systems)
  - Can't run physics tests headless

Agent recognizes: Architecture blocks testing
Decision: Restructure plugin into 4 independent modules
  - CoreSimulationPlugin (physics only, zero dependencies)
  - RenderingPlugin (visual systems)
  - UIPlugin (Egui panels)
  - InputHandlingPlugin (keyboard/mouse)

Result: Headless testing now possible
```

**Key insight**: The failure wasn't in phase 3 (agent restructured correctly). The failure was in phases 1-2 (not recognizing testing was needed, dismissing user reports as "works fine").

---

## The Trade-off Framework

When you hit a testing blocker, calculate the restructuring vs. ongoing cost:

### Restructuring Cost

**One-time investment**:
- Separate concerns into independent modules
- Extract business logic from dependencies
- Create plugin/service boundaries
- Update instantiation code

**BattleSpace example**: 4 hours to restructure SpaceSimulationPlugin
- Extract CoreSimulationPlugin (physics + movement)
- Extract RenderingPlugin (visual systems)
- Extract UIPlugin (Egui panels)
- Extract InputHandlingPlugin (keyboard/mouse input)

### Ongoing Cost (if NOT restructured)

**Every test you write**:
- Complex setup (mock GPU, rendering context, input systems)
- Fragile tests (coupled to external dependencies)
- Slow tests (full application startup)
- Flaky tests (race conditions with rendering pipeline)

**Every bug you debug**:
- Can't isolate business logic
- Mixed concerns make reproduction hard
- Visual inspection required (can't automate)

**Every feature you add**:
- Untested foundation (accumulating technical debt)
- High risk of regressions
- Hard to verify correctness

### Break-even Calculation

```
Break-even point = Restructuring cost / Test overhead per test

BattleSpace:
  Restructuring: 4 hours
  Test overhead without restructure: ~30 min per test (complex setup)

  Break-even: 4 hours / 0.5 hours per test = 8 tests

  Planned tests: 15+ integration tests

  Decision: Restructure now (pays off after 8 tests, we need 15+)
```

---

## Before/After Example: BattleSpace Plugin Restructuring

### Before: Monolithic Plugin

```rust
pub struct SpaceSimulationPlugin;

impl Plugin for SpaceSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Physics systems (business logic)
            .add_systems(Startup, setup_space_simulation)
            .add_systems(FixedUpdate, (apply_gravity, integrate_velocity))

            // Rendering systems (requires GPU)
            .add_systems(Update, (render_orbital_paths, update_focus))

            // UI systems (requires Egui context)
            .add_systems(Update, (setup_ui, update_focus_ui))

            // Input systems (requires keyboard/mouse)
            .add_systems(Update, (handle_focus_switching, handle_selection));
    }
}
```

**Testing blockers**:
- Can't test physics without rendering context
- Can't test movement without GPU
- Need window + camera + input systems for any test
- Must use `DefaultPlugins` (250+ systems, window creation, input handling)

**Attempted test**:
```rust
#[test]
fn test_energy_conservation() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins); // Requires GPU, window, input!
    app.add_plugins(SpaceSimulationPlugin);

    // ERROR: Can't create window in test environment
    // ERROR: No GPU context available
}
```

### After: Separated Concerns

```rust
// Physics only - zero external dependencies
pub struct CoreSimulationPlugin;
impl Plugin for CoreSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_space_simulation)
           .add_systems(FixedUpdate, (apply_gravity, integrate_velocity));
    }
}

// Rendering only - requires rendering context
pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (render_orbital_paths, update_focus));
    }
}

// UI only - requires Egui
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup_ui, update_focus_ui));
    }
}

// Input only - requires keyboard/mouse
pub struct InputHandlingPlugin;
impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_focus_switching, handle_selection));
    }
}
```

**Testing now possible**:
```rust
#[test]
fn test_energy_conservation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);     // No GPU, no window!
    app.add_plugins(AssetPlugin::default()); // Just assets
    app.add_plugins(CoreSimulationPlugin);   // Physics only

    // Run physics simulation headless
    app.update();

    let energy = calculate_system_energy(&app);
    assert!(energy_drift < 0.001); // Now we can actually test this!
}
```

### Benefits Beyond Testing

**1. Multiplayer support** (future feature)
- Server runs CoreSimulationPlugin (authoritative physics)
- Clients run RenderingPlugin + UIPlugin (visuals only)
- Zero code changes needed

**2. Performance profiling**
- Can benchmark physics in isolation
- Can optimize rendering without physics overhead
- Clear performance attribution

**3. Maintainability**
- Clear module boundaries
- Dependencies explicit (not hidden)
- Changes localized to one plugin

**4. Headless simulation** (CI, server-side)
- Run physics without GPU
- Batch simulations for AI training
- Deterministic testing in CI

---

## Common Red Flags (Stop and Consider Restructuring)

### Dismissal Patterns

When you catch yourself saying:

- **"It works fine"** → Are you dismissing user reports? What are they seeing that you're not?
- **"Manual testing is thorough"** → Does it scale? Can you test all edge cases? Every release?
- **"Testing can come later"** → When? After shipping? When technical debt is massive?
- **"User error"** → Have you investigated? Reproduced? Verified?

### Architecture Patterns

When you see:

- **"Can't test without [rendering/GPU/network]"** → Why is business logic coupled to external dependency?
- **"Need full application to test"** → Why can't you test in isolation?
- **"Testing requires visual inspection"** → Why can't you assert on business logic state?
- **Monolithic modules mixing concerns** → Physics + rendering + UI + input in one plugin

### Behavior Patterns

When you find yourself:

- **Continuing feature work when foundation questionable** → Are you building on broken base?
- **Building features without tests** → How do you know they work? How will you prevent regressions?
- **Dismissing test failures** → "Works fine locally" doesn't mean it works correctly
- **Avoiding testing work** → "Testing is blocked" → Why? What's blocking it?

---

## When NOT to Restructure

### Dependencies Can Be Adequately Mocked

If external dependency is:
- Simple interface (filesystem: read/write, network: send/receive)
- Easy to mock (dependency injection, trait abstraction)
- Not core to business logic

**Example**:
```rust
// Business logic with mockable dependency
trait DataStore {
    fn save(&self, data: &str) -> Result<()>;
}

// Easy to mock in tests
struct MockStore;
impl DataStore for MockStore {
    fn save(&self, data: &str) -> Result<()> {
        Ok(()) // No-op for tests
    }
}
```

**Don't restructure** - mocking is adequate.

### Simple Logic With No Complexity

If code is:
- Trivial logic (getters/setters, data transformations)
- No edge cases
- No complex state machines
- No physics/calculations

**Example**: UI binding that just updates text from resource.

**Don't test** - visual validation sufficient.

### Time Constraints Make It Mathematically Impossible

If:
- Restructuring: 4 hours
- Time available: 2 hours
- Deadline: Hard (production incident, investor demo)

**Still document as tech debt**:

```rust
// TODO: Restructure to separate physics from rendering (tech debt ticket #123)
// Reason: Production incident, 2 hours to fix, restructuring would take 4 hours
// Impact: Can't write tests, manual validation only
// Timeline: Fix in next sprint (scheduled)
```

**Key**: Document the decision and timeline for fixing.

---

## Pattern Recognition Checklist

When you encounter testing difficulty, ask:

**Symptoms present?**
- [ ] Testing requires GPU/rendering for business logic
- [ ] User reports issues you can't reproduce
- [ ] Manual testing is primary validation
- [ ] Tests would need complex mocking
- [ ] Architecture mixes concerns (physics + rendering + UI)

**Dismissal patterns present?**
- [ ] Saying "works fine" when users report issues
- [ ] Assuming user error without investigation
- [ ] Continuing features when foundation questionable
- [ ] "Testing can come later" mindset

**Trade-off calculated?**
- [ ] Estimated restructuring cost
- [ ] Estimated ongoing cost (per test, per bug, per feature)
- [ ] Break-even point calculated
- [ ] Decision justified

**If 3+ symptoms present → Strong signal to restructure**

---

## Remember

- **Recognition is the challenge** - agents choose correctly when aware of choice, but often dismiss symptoms instead of recognizing need
- **Symptoms are real** - user reports, testing difficulty, manual validation - don't dismiss as "works fine"
- **Foundation matters** - building features on broken foundation accumulates exponential tech debt
- **One-time vs. ongoing cost** - restructuring hurts once, ongoing testing pain hurts forever
- **Benefits beyond testing** - modularity enables future features (multiplayer, profiling, headless)

---

## Real-World Case Study

**BattleSpace Plugin Restructuring** (October 2025)

**Background**: 3D space battle simulator with N-body physics, orbital simulation, vehicle movement

**Problem symptoms**:
1. User: "Time scale not working" → Agent: "Works fine" (dismissal)
2. Reality: 99.99994% of simulation time discarded (critical bug)
3. Physics tests: 0.24% energy drift, 11.85% orbital drift
4. Movement tests: Arrival detection, state transitions broken
5. Agent continued features instead of pausing for testing

**Architecture blocker**:
- SpaceSimulationPlugin: Physics + rendering + UI + input (monolithic)
- Testing requires GPU, window, input systems (can't run headless)
- Attempted test with DefaultPlugins: Fails (no GPU in test environment)

**Restructuring decision**:
- Cost: 4 hours to separate into 4 plugins
- Ongoing cost: ~30 min per test for complex mocking
- Break-even: 8 tests (planned: 15+ tests)
- Decision: Restructure now

**Implementation**:
1. Extract CoreSimulationPlugin (physics + movement, zero dependencies)
2. Extract RenderingPlugin (visual systems, requires GPU)
3. Extract UIPlugin (Egui panels, requires Egui context)
4. Extract InputHandlingPlugin (keyboard/mouse)

**Results**:
- Testing now possible (MinimalPlugins + CoreSimulationPlugin)
- 8 integration tests implemented (3/8 passing = success, reveals bugs)
- Found 2 known bugs + 2 unknown bugs quantitatively
- Clear plugin boundaries enable future features (multiplayer, profiling)

**Lesson**: Real failure wasn't restructuring decision (agent chose correctly). Real failure was phases 1-2: not recognizing testing need, dismissing symptoms, continuing without verification.

---

## Quick Reference

| Symptom | Recognition | Action |
|---------|-------------|--------|
| "Can't test without GPU" | Architecture blocks testing | Restructure to separate concerns |
| "It works fine" (user says broken) | Dismissing symptoms | Investigate, reproduce, don't dismiss |
| Manual testing primary | Doesn't scale | Add automated tests (restructure if blocked) |
| Features on broken foundation | Accumulating tech debt | Pause features, fix foundation first |
| Monolithic module | Mixed concerns | Extract independent modules |

---

## Version History

**1.0.0** (2025-10-14)
- Initial version based on BattleSpace testing infrastructure experience
- Pattern-recognition approach (not discipline enforcement)
- Focus on recognizing symptoms vs. making correct choice when aware
- Case study from real project (SpaceSimulationPlugin restructuring)
