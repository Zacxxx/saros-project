# Saros Code Generation Rules

## Architecture: Structure of Arrays (SoA)

Prefer SoA over AoS for all game data. Group data by type, not by entity.

```rust
// ❌ AoS
struct Entities { positions: Vec<Vec3>, healths: Vec<f32>, names: Vec<String> }

// ✅ SoA
struct Positions(Vec<Vec3>);
struct Healths(Vec<f32>);
struct Names(Vec<String>);
```

This aligns with Bevy's ECS model: each component type is its own contiguous array.

---

## DRY — Don't Repeat Yourself

- Every piece of logic exists in exactly one place.
- If the same logic appears twice, extract it.
- Shared types live in a dedicated `types/` or `shared/` module.
- Constants are named and defined once at the top of their scope.

```rust
// ❌ Repeated
if hp <= 0.0 { die() }  // in combat.rs
if hp <= 0.0 { die() }  // in poison.rs

// ✅ Extracted
fn is_dead(hp: f32) -> bool { hp <= 0.0 }
```

---

## Separation of Concerns

Each module, file, and function has one responsibility. Do not mix:

- **Data** (schemas, structs) with **logic** (systems, reducers)
- **Rendering** with **game state**
- **API layer** with **business logic**
- **Game engine** with **devtools**

Directory structure enforces this:
```
systems/     ← logic only
components/  ← data only
api/         ← HTTP layer only
schemas/     ← type definitions only
```

---

## Atomicity

Every function does one thing and does it completely.

- A function either fully succeeds or fully fails — no partial state.
- Side effects are explicit and isolated.
- Reducers (SpacetimeDB) are transactional by nature — treat all mutations this way.
- Prefer returning `Result<T, E>` over panicking or silent failure.

```rust
// ❌ Does too much
fn update_player(id: u64, damage: f32) {
    apply_damage(id, damage);
    check_death(id);
    notify_ui(id);
    log_event(id);
}

// ✅ Atomic, composable
fn apply_damage(id: u64, damage: f32) -> Result<f32, GameError>
fn check_death(hp: f32) -> bool
fn notify_ui(id: u64, event: GameEvent)
```

---

## One Function Per File

Each file exports exactly one primary function, system, component, or type.

- File name matches the exported symbol.
- No barrel files that define logic.
- Index/mod files only re-export, never define.

```
systems/
  apply_damage.rs      → pub fn apply_damage(...)
  move_entity.rs       → pub fn move_entity(...)
  generate_terrain.rs  → pub fn generate_terrain(...)

components/
  position.rs          → pub struct Position(Vec3)
  health.rs            → pub struct Health(f32)
```

```ts
// devtools
app/blocks/page.tsx          → default export BlocksPage
components/BlockCard.tsx     → default export BlockCard
hooks/useBlocks.ts           → export function useBlocks()
api/fetchBlocks.ts           → export function fetchBlocks()
```

---

## Summary Table

| Rule | What it prevents |
|---|---|
| SoA | Cache misses, mixed-concern structs |
| DRY | Drift, duplication bugs |
| Separation of Concerns | Coupling, untestable code |
| Atomicity | Partial state, hidden side effects |
| One function per file | Bloated modules, unclear ownership |
