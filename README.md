# Saros

An isometric 3D voxel RPG with turn-based combat, AI-powered NPCs, building, crafting, and an integrated devtools suite.

## Stack

| Layer | Technology |
|---|---|
| Native shell | Tauri v2 |
| Game rendering | Bevy 0.13 (background thread, Linux/X11) |
| UI overlay | React + Vite (transparent WebView) |
| Persistent state | SpacetimeDB |
| Devtools | Next.js + Tailwind |

## Features

- Isometric voxel world with procedural terrain (FBM heightmap, biomes, caves, trees)
- Turn-based combat with AP/MP system
- Click-to-move A* pathfinding
- Block interaction — mine and place voxels
- Inventory, hotbar, and crafting system
- AI NPCs with waypoint routines, LLM-powered dialogue, and periodic thought generation
- Quest system with objective tracking
- Devtools suite: block/item/recipe/NPC/quest editors, texture manager, game rules explorer

## Project Structure

```
saros-project/
├── game/               # Tauri v2 app (Bevy + React)
│   ├── src/            # React/Vite frontend overlay
│   └── src-tauri/      # Rust backend (Bevy, Axum, Tauri commands)
│       ├── spacetimedb/    # SpacetimeDB module
│       └── crates/saros-types/  # Shared serializable types
├── devtools/           # Next.js devtools (port 3001)
└── documentation/      # Design intention, technical spec, code rules
```

## Getting Started

### Prerequisites

- Rust (stable)
- Node.js / Bun
- Tauri v2 CLI (`cargo install tauri-cli`)
- SpacetimeDB CLI (`cargo install spacetimedb-cli`)

### Run

```bash
# Start SpacetimeDB
make spacetime-start

# Publish the SpacetimeDB module
make spacetime-publish

# Run the game (Tauri + Bevy + React)
make dev

# Run devtools
make devtools
```

### Environment

Copy `.env.example` to `.env` and fill in your values (LLM API key is optional — NPCs fall back to placeholder dialogue without it).

## Devtools

The devtools run at `http://localhost:3001` and connect to the Axum API at `http://localhost:8080`.

Sections: Blocks · Items · Recipes · NPCs · Quests · Rules · Textures

## Code Rules

See [`documentation/CODE_RULES.md`](documentation/CODE_RULES.md). Key principles:

- One function/type per file, file name = exported symbol
- SoA for all game data
- `Result<T, E>` everywhere — no panics
- DRY, separation of concerns, atomicity
