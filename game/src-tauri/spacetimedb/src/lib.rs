use spacetimedb::{ReducerContext, Table, Identity, Timestamp};

// ── World ────────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = world, public)]
pub struct World {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub seed: u64,
    pub size: u32,
    pub sea_level: u32,
    pub cave_density: f32,
    pub structure_frequency: f32,
    pub created_at: Timestamp,
}

// ── Block ────────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = block, public)]
pub struct Block {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub texture_id: String,
    pub is_solid: bool,
    pub is_transparent: bool,
    pub hardness: f32,
}

// ── Item ─────────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = item, public)]
pub struct Item {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub texture_id: String,
    pub stack_size: u32,
    pub item_type: String,
}

// ── Recipe ───────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = recipe, public)]
pub struct Recipe {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub output_item_id: u64,
    pub output_quantity: u32,
    pub inputs_json: String, // JSON: [{item_id, quantity}]
}

// ── Chunk ────────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = chunk, public)]
pub struct Chunk {
    #[primary_key]
    pub key: String, // "world_id:cx:cz"
    pub world_id: u64,
    pub cx: i32,
    pub cz: i32,
    pub blocks: Vec<u8>, // compressed block data
}

// ── Player ───────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub world_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub ap: u32,
    pub mp: u32,
}

// ── NPC ──────────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = npc, public)]
pub struct Npc {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub world_id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub traits_json: String,   // JSON: [String]
    pub routine_json: String,  // JSON: [{time, waypoint}]
    pub memory_json: String,   // JSON: [{role, content}]
    pub current_thought: String,
}

// ── Quest ────────────────────────────────────────────────────────────────────

#[spacetimedb::table(name = quest, public)]
pub struct Quest {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub world_id: u64,
    pub title: String,
    pub description: String,
    pub giver_npc_id: u64,
    pub objectives_json: String, // JSON: [{type, target, quantity}]
    pub rewards_json: String,    // JSON: [{item_id, quantity}]
}

// ── Reducers ─────────────────────────────────────────────────────────────────

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {}

#[spacetimedb::reducer]
pub fn create_world(ctx: &ReducerContext, name: String, seed: u64, size: u32, sea_level: u32, cave_density: f32, structure_frequency: f32) {
    ctx.db.world().insert(World {
        id: 0,
        name,
        seed,
        size,
        sea_level,
        cave_density,
        structure_frequency,
        created_at: ctx.timestamp,
    });
}

#[spacetimedb::reducer]
pub fn upsert_chunk(ctx: &ReducerContext, world_id: u64, cx: i32, cz: i32, blocks: Vec<u8>) {
    let key = format!("{}:{}:{}", world_id, cx, cz);
    if ctx.db.chunk().key().find(&key).is_some() {
        ctx.db.chunk().key().update(Chunk { key, world_id, cx, cz, blocks });
    } else {
        ctx.db.chunk().insert(Chunk { key, world_id, cx, cz, blocks });
    }
}
