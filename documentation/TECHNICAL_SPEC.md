# Saros - Technical Specification

## Architecture Overview

Saros is built with a **distributed architecture** separating concerns into three primary components:

1. **Game Engine** (Rust + Bevy + SpacetimeDB): Core game logic, world simulation, and data persistence
2. **Game UI** (React + CSS): Player-facing interface overlays and menus
3. **Devtools** (Next.js + Tailwind): Content creation and management interface

## Technology Stack

### Game Engine (`/game`)

#### Core Technologies
- **Rust**: Systems programming language for performance and safety
- **Bevy**: Entity Component System (ECS) game engine
  - Rendering pipeline for isometric 3D voxels
  - Physics and collision detection
  - Input handling and event systems
- **SpacetimeDB**: Distributed database for game state persistence
  - Schema definitions for all game entities
  - Real-time synchronization
  - Query and mutation APIs

#### Key Libraries
- **bevy_ecs_tilemap** or custom voxel renderer: Isometric tile rendering
- **noise-rs**: Procedural terrain generation (Perlin/Simplex noise)
- **serde**: Serialization/deserialization for data interchange
- **tokio**: Async runtime for network operations
- **axum** or **actix-web**: HTTP server for devtools API

### Game UI (`/game/ui`)
- **React**: Component-based UI framework
- **CSS**: Styling for game overlays
- **WebAssembly bridge**: Communication with Rust game engine

### Devtools (`/devtools`)
- **Next.js 14+**: React framework with App Router
- **TypeScript**: Type-safe development
- **Tailwind CSS**: Utility-first styling
- **React Query**: Data fetching and caching
- **Zod**: Schema validation matching game engine types

## System Architecture

### Data Flow

```
┌─────────────────┐
│  SpacetimeDB    │ ← Persistent game state
│   (Schemas)     │
└────────┬────────┘
         │
         ↓
┌─────────────────┐      ┌──────────────┐
│  Game Engine    │ ←──→ │   Game UI    │
│  (Rust + Bevy)  │      │   (React)    │
└────────┬────────┘      └──────────────┘
         │
         │ HTTP API
         ↓
┌─────────────────┐
│   Devtools      │
│ (Next.js + TW)  │
└─────────────────┘
```

### Schema Synchronization

Game engine defines schemas in SpacetimeDB that are:
1. **Exported via HTTP API** to devtools
2. **Automatically reflected** in devtools UI
3. **Validated on both sides** using shared type definitions

## Core Systems

### 1. World Generation System

#### Terrain Generation Pipeline
1. **Noise Generation**: Multi-octave Perlin/Simplex noise for height maps
2. **Biome Assignment**: Temperature and moisture maps determine biome types
3. **Elevation Mapping**: Height values translated to voxel columns
4. **Water Placement**: Fill areas below sea level
5. **Structure Generation**: Place trees, rocks, and natural features
6. **Cave Carving**: 3D noise for underground cavern systems

#### Chunk Management
- **Chunk size**: 16x16x256 voxels (configurable)
- **Loading radius**: Configurable distance around player
- **Unloading strategy**: LRU cache with dirty chunk persistence
- **Serialization**: Compressed chunk data stored in SpacetimeDB

### 2. Rendering System

#### Isometric Projection
- **Camera angle**: Fixed isometric perspective (typically 30° or 45°)
- **Voxel rendering**: Mesh generation from visible block faces
- **Culling**: Frustum culling and occlusion culling for performance
- **Batching**: Combine meshes per chunk for reduced draw calls

#### Visual Features
- **Texture atlas**: Single texture for all block types
- **Lighting**: Ambient occlusion and simple directional lighting
- **Water rendering**: Transparent blocks with simple animation
- **Particle effects**: For interactions and environmental effects

### 3. Turn-Based Movement System

#### Grid System
- **Movement grid**: Overlay on voxel world for valid positions
- **Turn structure**:
  - Player selects destination (mouse click)
  - Movement points calculated based on distance
  - Character moves along path
  - Turn ends when movement complete or player confirms
- **Pathfinding**: A* algorithm for obstacle avoidance
- **Movement constraints**: Elevation limits, obstacle detection

### 4. Combat System (Dofus-inspired)

#### Turn-Based Combat Flow
1. **Initiative order**: Determine turn sequence
2. **Action points (AP)**: Each action costs AP
3. **Movement points (MP)**: Movement costs MP
4. **Abilities**: Skills with AP costs, ranges, and effects
5. **Positioning**: Tactical advantage based on grid position
6. **Turn end**: Player confirms, next combatant's turn

#### Combat Mechanics
- **Range system**: Abilities have min/max range
- **Line of sight**: Obstacles block targeting
- **Area effects**: Multi-tile damage/buff zones
- **Status effects**: Buffs, debuffs, damage over time

### 5. Building & Crafting System

#### Block Placement
- **Placement rules**: Validate position and support
- **Block types**: Stored in SpacetimeDB schema
- **Inventory management**: Player block inventory
- **Modification tracking**: Update chunk dirty state

#### Crafting System
- **Recipe definitions**: Input items → output items
- **Crafting stations**: Some recipes require specific blocks
- **Recipe discovery**: Unlock recipes through gameplay
- **Validation**: Check inventory for required materials

### 6. NPC System

#### NPC Schema
```rust
struct NPC {
    id: u64,
    name: String,
    position: Vec3,
    personality_traits: Vec<String>,
    routine: Schedule,
    memory: Vec<Interaction>,
    current_thought: Option<String>,
}
```

#### AI Integration
- **Dialogue generation**: LLM API calls with NPC context
- **Thought generation**: Periodic AI-generated thoughts based on:
  - Current environment
  - Recent interactions
  - Personality traits
  - Time of day
- **Behavior modification**: Thoughts influence routine and reactions
- **Memory system**: Store player interactions for context

#### Dialogue System
- **Natural language input**: Player types messages
- **Context building**: Include NPC personality, memory, location
- **Response generation**: LLM generates in-character response
- **Action triggers**: Dialogue can trigger quests, trades, or events

### 7. Quest System

#### Quest Schema
```rust
struct Quest {
    id: u64,
    title: String,
    description: String,
    giver_npc_id: u64,
    objectives: Vec<Objective>,
    rewards: Vec<Item>,
    prerequisites: Vec<u64>, // Quest IDs
}

enum Objective {
    CollectItems { item_id: u64, quantity: u32 },
    DefeatEnemies { enemy_type: String, count: u32 },
    ReachLocation { position: Vec3 },
    TalkToNPC { npc_id: u64 },
}
```

#### Quest Progression
- **Tracking**: Monitor objective completion
- **Updates**: Notify player of progress
- **Completion**: Validate all objectives met
- **Rewards**: Grant items/experience on completion

## Devtools Architecture

### HTTP API Endpoints

#### Schema Endpoints
- `GET /api/schemas` - Retrieve all schema definitions
- `GET /api/schemas/{type}` - Get specific schema (blocks, items, npcs, quests, recipes)

#### Block Management
- `GET /api/blocks` - List all block types
- `POST /api/blocks` - Create new block type
- `PUT /api/blocks/{id}` - Update block definition
- `POST /api/blocks/{id}/texture` - Upload texture file

#### Item Management
- `GET /api/items` - List all items
- `POST /api/items` - Create new item
- `PUT /api/items/{id}` - Update item definition
- `POST /api/items/{id}/texture` - Upload item texture

#### Recipe Management
- `GET /api/recipes` - List all recipes
- `POST /api/recipes` - Create new recipe
- `PUT /api/recipes/{id}` - Update recipe
- `DELETE /api/recipes/{id}` - Remove recipe

#### NPC Management
- `GET /api/npcs` - List all NPCs
- `POST /api/npcs` - Create new NPC
- `PUT /api/npcs/{id}` - Update NPC definition
- `DELETE /api/npcs/{id}` - Remove NPC

#### Quest Management
- `GET /api/quests` - List all quests
- `POST /api/quests` - Create new quest
- `PUT /api/quests/{id}` - Update quest
- `DELETE /api/quests/{id}` - Remove quest

#### Game Rules
- `GET /api/rules` - List all configurable game rules
- `PUT /api/rules/{id}` - Update rule value

#### UI Definitions
- `GET /api/ui` - List all UI definitions
- `POST /api/ui` - Create new UI definition
- `PUT /api/ui/{id}` - Update UI definition

### Devtools Features

#### 1. Texture Management
- **Upload interface**: Drag-and-drop texture files
- **Preview**: Real-time texture preview on models
- **Atlas generation**: Automatic texture atlas creation
- **Format validation**: Ensure correct image formats and dimensions

#### 2. Interface Builder
- **Visual editor**: Drag-and-drop UI component placement
- **Component library**: Reusable UI elements
- **Layout system**: Responsive positioning
- **Export**: Generate UI definitions for game engine

#### 3. Game Rules Editor
- **Auto-discovery**: Fetch rules from game engine
- **Type-aware inputs**: Appropriate controls for each rule type
- **Validation**: Ensure valid values before saving
- **Live preview**: See rule effects in test environment

#### 4. Item & Recipe Editor
- **Item creation**: Define properties, stats, textures
- **Recipe builder**: Visual interface for input/output items
- **Dependency graph**: Visualize crafting chains
- **Validation**: Ensure recipes reference valid items

#### 5. NPC Editor
- **Personality builder**: Define traits and behaviors
- **Routine scheduler**: Set time-based activities
- **Dialogue testing**: Test AI responses with sample inputs
- **Spawn configuration**: Set initial positions and conditions

#### 6. Quest Editor
- **Objective builder**: Add/remove quest objectives
- **Reward configuration**: Define quest rewards
- **Prerequisite chains**: Set quest dependencies
- **Testing tools**: Simulate quest progression

## Data Schemas

### Block Schema
```rust
#[derive(Serialize, Deserialize)]
struct Block {
    id: u64,
    name: String,
    texture_id: String,
    is_solid: bool,
    is_transparent: bool,
    hardness: f32,
    tool_required: Option<String>,
}
```

### Item Schema
```rust
#[derive(Serialize, Deserialize)]
struct Item {
    id: u64,
    name: String,
    description: String,
    texture_id: String,
    stack_size: u32,
    item_type: ItemType,
}

enum ItemType {
    Block { block_id: u64 },
    Tool { durability: u32, efficiency: f32 },
    Consumable { effect: String },
    Material,
}
```

### Recipe Schema
```rust
#[derive(Serialize, Deserialize)]
struct Recipe {
    id: u64,
    inputs: Vec<ItemStack>,
    output: ItemStack,
    crafting_station: Option<u64>, // Block ID
}

struct ItemStack {
    item_id: u64,
    quantity: u32,
}
```

## Performance Considerations

### Optimization Strategies
- **Chunk meshing**: Only regenerate meshes for modified chunks
- **LOD system**: Reduce detail for distant chunks
- **Entity pooling**: Reuse entity allocations
- **Spatial partitioning**: Octree or grid for entity queries
- **Async loading**: Load chunks and assets off main thread
- **Texture compression**: Use GPU-friendly formats

### Scalability
- **Horizontal scaling**: SpacetimeDB handles distributed state
- **Client-side prediction**: Reduce perceived latency
- **Delta compression**: Minimize network traffic
- **Lazy loading**: Load content on-demand in devtools

## Development Phases

### Phase 1: Foundation
- Project structure and build system
- Basic Bevy rendering setup
- SpacetimeDB schema definitions
- Simple chunk generation and loading

### Phase 2: Core Gameplay
- Isometric camera and rendering
- Turn-based movement system
- Block placement and removal
- Basic terrain generation

### Phase 3: Content Systems
- Item and inventory system
- Crafting and recipes
- NPC spawning and basic AI
- Quest framework

### Phase 4: Advanced Features
- Combat system implementation
- AI dialogue integration
- Structure generation (forests, caves)
- Water and fluid simulation

### Phase 5: Devtools
- HTTP API implementation
- Next.js devtools foundation
- Texture management interface
- Item and recipe editors

### Phase 6: Polish & Integration
- NPC and quest editors
- Game rules interface
- UI builder
- Performance optimization
- Testing and bug fixes

## Testing Strategy

### Unit Tests
- Terrain generation algorithms
- Pathfinding logic
- Recipe validation
- Schema serialization

### Integration Tests
- API endpoint functionality
- SpacetimeDB queries
- Devtools synchronization
- Combat system interactions

### Performance Tests
- Chunk loading benchmarks
- Rendering frame rates
- Memory usage profiling
- Network latency measurements

## Security Considerations

- **Input validation**: Sanitize all devtools inputs
- **Authentication**: Protect devtools API endpoints
- **Rate limiting**: Prevent API abuse
- **Texture validation**: Scan uploaded files for malicious content
- **Sandboxing**: Isolate AI API calls

## Future Enhancements

- **Multiplayer support**: Multiple players in shared world
- **Modding API**: Allow community content creation
- **Advanced AI**: More sophisticated NPC behaviors
- **Biome expansion**: Additional terrain types
- **Magic system**: Spells and abilities
- **Economy**: Trading and currency systems
