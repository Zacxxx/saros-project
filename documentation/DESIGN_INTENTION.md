# Saros - Design Intention Document

## Project Overview

**Saros** is an isometric 3D voxel-based exploration and crafting game inspired by Minecraft's sandbox mechanics and Dofus's turn-based combat system. The project emphasizes procedural generation, AI-driven NPCs, and a comprehensive development toolchain for content creation.

## Core Vision

Create a rich, explorable voxel world where players can:
- Explore procedurally generated terrain with natural features
- Engage in turn-based tactical combat
- Build and craft within the world
- Interact with AI-powered NPCs through natural language
- Experience emergent gameplay through NPC behaviors and routines

## Game Design Pillars

### 1. Exploration
- **Isometric 3D perspective** for clear spatial awareness and tactical positioning
- **Procedural terrain generation** with varied biomes, elevation, and natural structures
- **Chunk-based world loading** for seamless exploration of large worlds
- **Underground systems** including caves and subterranean structures
- **Natural landmarks** such as forests, mountains, and water bodies

### 2. Turn-Based Gameplay
- **Movement system**: Grid-based turn-based movement inspired by tactical RPGs
- **Combat mechanics**: Dofus-style turn-based combat with positioning and abilities
- **Mouse-driven controls**: Click-to-move interface for intuitive interaction

### 3. Building & Crafting
- **Block placement and removal** for construction
- **Recipe-based crafting system** for item creation
- **Resource gathering** from the environment

### 4. AI-Driven NPCs
- **Natural language dialogue**: Converse with NPCs using text prompts
- **AI-generated responses**: Dynamic dialogue based on context and NPC personality
- **Autonomous behavior**: NPCs have routines and behaviors driven by AI-generated thoughts
- **Emergent interactions**: NPC thoughts influence their actions and reactions to the world

### 5. Quest System
- **Structured quests** provided by NPCs
- **Dynamic objectives** tied to exploration, combat, and crafting
- **Quest progression** tracked through the game engine

## World Generation

### Terrain Features
- **Elevation mapping**: Hills, mountains, valleys
- **Water systems**: Rivers, lakes, oceans
- **Biome diversity**: Forests, plains, deserts, tundra
- **Natural structures**: Tree clusters, rock formations
- **Underground networks**: Cave systems, mineral deposits

### Chunk System
- **Dynamic loading**: Load/unload chunks based on player position
- **Persistent state**: Save chunk modifications and structures
- **Generation pipeline**: Noise-based terrain → biome assignment → structure placement

## Content Creation Philosophy

The game is designed with a **data-driven architecture** where game content (blocks, items, NPCs, quests, recipes) is defined in schemas that can be:
- Modified through the development tools
- Automatically synchronized between game engine and devtools
- Extended without code changes

This approach enables rapid iteration and content expansion while maintaining consistency across the game and tooling ecosystem.

## Player Experience Goals

1. **Discovery**: Reward exploration with varied terrain, hidden structures, and unique NPCs
2. **Strategy**: Provide tactical depth through turn-based combat and positioning
3. **Creativity**: Enable player expression through building and crafting
4. **Immersion**: Create believable NPCs with personality and agency
5. **Accessibility**: Intuitive mouse-based controls and clear visual feedback

## Unique Selling Points

- **Isometric voxel aesthetic** combining retro charm with modern rendering
- **AI-powered NPCs** that feel alive and responsive
- **Integrated development tools** for rapid content creation
- **Turn-based tactical gameplay** in a sandbox environment
- **Natural language interaction** with game characters

## Target Audience

Players who enjoy:
- Sandbox exploration games (Minecraft, Terraria)
- Turn-based tactical RPGs (Dofus, Wakfu, Final Fantasy Tactics)
- Games with emergent AI behaviors
- Creative building and crafting systems

## Development Philosophy

- **Modular architecture**: Separate concerns between game engine, UI, and devtools
- **Schema-driven design**: Define content in data structures, not hardcoded logic
- **Tooling-first approach**: Build robust devtools to accelerate content creation
- **Iterative development**: Start with core systems, expand features incrementally
