use axum::routing::{get, put};
use axum::{
    Json, Router,
    extract::{Path, State},
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

struct AppState {
    blocks: Mutex<Vec<Value>>,
    textures: Mutex<Vec<Value>>,
    mobs: Mutex<Vec<Value>>,
}

pub fn router() -> Router {
    let state = Arc::new(AppState {
        blocks: Mutex::new(vec![
            json!({"id":1,"name":"Stone","texture_id":"stone.png","is_solid":true,"is_transparent":false,"hardness":1.5}),
            json!({"id":2,"name":"Dirt","texture_id":"dirt.png","is_solid":true,"is_transparent":false,"hardness":0.5}),
            json!({"id":3,"name":"Grass","texture_id":"grass_block_top.png","is_solid":true,"is_transparent":false,"hardness":0.6}),
            json!({"id":4,"name":"Water","texture_id":"water_still.png","is_solid":false,"is_transparent":true,"hardness":0.0}),
            json!({"id":5,"name":"Sand","texture_id":"sand.png","is_solid":true,"is_transparent":false,"hardness":0.5}),
            json!({"id":6,"name":"Oak Log","texture_id":"oak_log.png","is_solid":true,"is_transparent":false,"hardness":2.0}),
            json!({"id":7,"name":"Oak Leaves","texture_id":"oak_leaves.png","is_solid":false,"is_transparent":true,"hardness":0.2}),
            json!({"id":8,"name":"Oak Planks","texture_id":"oak_planks.png","is_solid":true,"is_transparent":false,"hardness":2.0}),
            json!({"id":9,"name":"Cobblestone","texture_id":"cobblestone.png","is_solid":true,"is_transparent":false,"hardness":2.0}),
            json!({"id":10,"name":"Iron Ore","texture_id":"iron_ore.png","is_solid":true,"is_transparent":false,"hardness":3.0}),
            json!({"id":11,"name":"Coal Ore","texture_id":"coal_ore.png","is_solid":true,"is_transparent":false,"hardness":3.0}),
            json!({"id":12,"name":"Gold Ore","texture_id":"gold_ore.png","is_solid":true,"is_transparent":false,"hardness":3.0}),
            json!({"id":13,"name":"Diamond Ore","texture_id":"diamond_ore.png","is_solid":true,"is_transparent":false,"hardness":3.0}),
            json!({"id":14,"name":"Gravel","texture_id":"gravel.png","is_solid":true,"is_transparent":false,"hardness":0.6}),
            json!({"id":15,"name":"Clay","texture_id":"clay.png","is_solid":true,"is_transparent":false,"hardness":0.6}),
            json!({"id":16,"name":"Snow","texture_id":"snow.png","is_solid":true,"is_transparent":false,"hardness":0.2}),
            json!({"id":17,"name":"Ice","texture_id":"ice.png","is_solid":true,"is_transparent":false,"hardness":0.5}),
            json!({"id":18,"name":"Obsidian","texture_id":"obsidian.png","is_solid":true,"is_transparent":false,"hardness":50.0}),
            json!({"id":19,"name":"Bedrock","texture_id":"bedrock.png","is_solid":true,"is_transparent":false,"hardness":-1.0}),
            json!({"id":20,"name":"Mossy Cobblestone","texture_id":"mossy_cobblestone.png","is_solid":true,"is_transparent":false,"hardness":2.0}),
            json!({"id":21,"name":"Bricks","texture_id":"bricks.png","is_solid":true,"is_transparent":false,"hardness":2.0}),
            json!({"id":22,"name":"Bookshelf","texture_id":"bookshelf.png","is_solid":true,"is_transparent":false,"hardness":1.5}),
            json!({"id":23,"name":"Glowstone","texture_id":"glowstone.png","is_solid":true,"is_transparent":false,"hardness":0.3}),
        ]),
        textures: Mutex::new(vec![]),
        mobs: Mutex::new(vec![
            json!({"id":100,"name":"Sheep","mob_type":"Passive","can_talk":false,"hp":8,"max_hp":8,"behavior":"Wander","ai_config":{"thought_interval":5.0},"loot_table":[{"item_id":20,"quantity_min":1,"quantity_max":2,"chance":1.0}]}),
            json!({"id":101,"name":"Cow","mob_type":"Passive","can_talk":false,"hp":10,"max_hp":10,"behavior":"Wander","ai_config":{"thought_interval":5.0},"loot_table":[{"item_id":18,"quantity_min":1,"quantity_max":3,"chance":1.0}]}),
            json!({"id":102,"name":"Pig","mob_type":"Passive","can_talk":false,"hp":8,"max_hp":8,"behavior":"Wander","ai_config":{"thought_interval":5.0},"loot_table":[{"item_id":18,"quantity_min":1,"quantity_max":3,"chance":1.0}]}),
            json!({"id":103,"name":"Zombie","mob_type":"Hostile","can_talk":false,"hp":20,"max_hp":20,"behavior":"Chase","ai_config":{"thought_interval":3.0,"aggro_range":16.0},"loot_table":[{"item_id":18,"quantity_min":0,"quantity_max":1,"chance":0.3},{"item_id":12,"quantity_min":0,"quantity_max":1,"chance":0.1}]}),
            json!({"id":104,"name":"Skeleton","mob_type":"Hostile","can_talk":false,"hp":20,"max_hp":20,"behavior":"Chase","ai_config":{"thought_interval":3.0,"aggro_range":16.0},"loot_table":[{"item_id":20,"quantity_min":0,"quantity_max":2,"chance":0.5},{"item_id":11,"quantity_min":1,"quantity_max":2,"chance":0.8}]}),
            json!({"id":105,"name":"Wolf","mob_type":"Neutral","can_talk":true,"hp":12,"max_hp":12,"behavior":"Wander","ai_config":{"thought_interval":4.0},"loot_table":[]}),
        ]),
    });

    Router::new()
        .route("/api/health", get(health))
        .route("/api/schemas", get(schemas))
        .route("/api/blocks", get(get_blocks).post(create_block))
        .route("/api/blocks/:id", put(update_block))
        .route("/api/items", get(items).post(create_item))
        .route("/api/recipes", get(recipes).post(create_recipe))
        .route("/api/npcs", get(npcs).post(create_npc))
        .route("/api/mobs", get(get_mobs).post(create_mob))
        .route("/api/mobs/:id", put(update_mob))
        .route("/api/quests", get(quests).post(create_quest))
        .route("/api/rules", get(rules))
        .route("/api/rules/:id", put(update_rule))
        .route("/api/textures", get(get_textures).post(upload_texture))
        .route("/api/structures", get(structures))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn schemas() -> Json<Value> {
    Json(json!({
        "block":     block_schema(),
        "item":      item_schema(),
        "recipe":    recipe_schema(),
        "npc":       npc_schema(),
        "mob":       mob_schema(),
        "quest":     quest_schema(),
        "structure": structure_schema(),
    }))
}

async fn get_blocks(State(state): State<Arc<AppState>>) -> Json<Value> {
    let blocks = state.blocks.lock().unwrap();
    Json(json!(*blocks))
}

async fn items() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Stone","description":"A block of stone","stack_size":64,"item_type":"block"},
        {"id":2,"name":"Dirt","description":"A block of dirt","stack_size":64,"item_type":"block"},
        {"id":3,"name":"Grass Block","description":"A grass-covered dirt block","stack_size":64,"item_type":"block"},
        {"id":4,"name":"Sand","description":"A block of sand","stack_size":64,"item_type":"block"},
        {"id":5,"name":"Oak Log","description":"A log from an oak tree","stack_size":64,"item_type":"block"},
        {"id":6,"name":"Oak Planks","description":"Planks crafted from oak logs","stack_size":64,"item_type":"block"},
        {"id":7,"name":"Cobblestone","description":"Rough stone blocks","stack_size":64,"item_type":"block"},
        {"id":8,"name":"Stone Brick","description":"Crafted from smooth stone","stack_size":64,"item_type":"block"},
        {"id":9,"name":"Wooden Sword","description":"A basic sword made of wood","stack_size":1,"item_type":"weapon"},
        {"id":10,"name":"Stone Pickaxe","description":"A pickaxe for mining stone","stack_size":1,"item_type":"tool"},
        {"id":11,"name":"Wooden Axe","description":"An axe for chopping wood","stack_size":1,"item_type":"tool"},
        {"id":12,"name":"Iron Ingot","description":"Smelted iron ore","stack_size":64,"item_type":"material"},
        {"id":13,"name":"Coal","description":"Fuel and crafting material","stack_size":64,"item_type":"material"},
        {"id":14,"name":"Gold Ingot","description":"Smelted gold ore","stack_size":64,"item_type":"material"},
        {"id":15,"name":"Diamond","description":"A precious gemstone","stack_size":64,"item_type":"material"},
        {"id":16,"name":"Apple","description":"A fresh apple that restores health","stack_size":64,"item_type":"food"},
        {"id":17,"name":"Bread","description":"Baked bread that restores hunger","stack_size":64,"item_type":"food"},
        {"id":18,"name":"Cooked Meat","description":"Cooked meat from animals","stack_size":64,"item_type":"food"},
        {"id":19,"name":"Torch","description":"Provides light in dark areas","stack_size":64,"item_type":"utility"},
        {"id":20,"name":"Stick","description":"A basic crafting material","stack_size":64,"item_type":"material"},
    ]))
}

async fn recipes() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Oak Planks","inputs":[{"item_id":5,"quantity":1}],"output_item_id":6,"output_quantity":4},
        {"id":2,"name":"Stone Brick","inputs":[{"item_id":1,"quantity":4}],"output_item_id":8,"output_quantity":4},
        {"id":3,"name":"Stick","inputs":[{"item_id":6,"quantity":2}],"output_item_id":20,"output_quantity":4},
        {"id":4,"name":"Wooden Sword","inputs":[{"item_id":6,"quantity":2},{"item_id":20,"quantity":1}],"output_item_id":9,"output_quantity":1},
        {"id":5,"name":"Stone Pickaxe","inputs":[{"item_id":7,"quantity":3},{"item_id":20,"quantity":2}],"output_item_id":10,"output_quantity":1},
        {"id":6,"name":"Wooden Axe","inputs":[{"item_id":6,"quantity":3},{"item_id":20,"quantity":2}],"output_item_id":11,"output_quantity":1},
        {"id":7,"name":"Torch","inputs":[{"item_id":20,"quantity":1},{"item_id":13,"quantity":1}],"output_item_id":19,"output_quantity":4},
        {"id":8,"name":"Bread","inputs":[{"item_id":4,"quantity":3}],"output_item_id":17,"output_quantity":1},
    ]))
}

async fn npcs() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Aldric","traits":["wise","friendly"],"x":5.0,"y":1.0,"z":5.0},
        {"id":2,"name":"Mira","traits":["curious","cautious"],"x":-5.0,"y":1.0,"z":3.0},
    ]))
}

async fn get_mobs(State(state): State<Arc<AppState>>) -> Json<Value> {
    let mobs = state.mobs.lock().unwrap();
    Json(json!(*mobs))
}

async fn create_mob(State(state): State<Arc<AppState>>, Json(body): Json<Value>) -> Json<Value> {
    let mut mobs = state.mobs.lock().unwrap();
    let mut new_mob = body.clone();
    let next_id = mobs
        .iter()
        .filter_map(|m| m.get("id").and_then(|id| id.as_u64()))
        .max()
        .unwrap_or(99)
        + 1;
    new_mob["id"] = json!(next_id);
    mobs.push(new_mob.clone());
    Json(json!({"ok": true, "data": new_mob}))
}

async fn update_mob(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut mobs = state.mobs.lock().unwrap();
    if let Some(mob) = mobs
        .iter_mut()
        .find(|m| m.get("id").and_then(|i| i.as_u64()) == Some(id))
    {
        *mob = body;
        Json(json!({"ok": true, "data": mob}))
    } else {
        Json(json!({"ok": false, "error": "Mob not found"}))
    }
}

async fn quests() -> Json<Value> {
    Json(json!([
        {"id":1,"title":"Gather Stone","giver":"Aldric","objectives":[{"description":"Collect 10 stone blocks","completed":false}]},
        {"id":2,"title":"Explore the Cave","giver":"Mira","objectives":[{"description":"Enter the cave","completed":false}]},
    ]))
}

async fn rules() -> Json<Value> {
    Json(json!([
        {"id":"sea_level","label":"Sea Level","type":"integer","value":32,"min":8,"max":128},
        {"id":"cave_density","label":"Cave Density","type":"float","value":0.4,"min":0.0,"max":1.0},
        {"id":"structure_frequency","label":"Structure Frequency","type":"float","value":0.3,"min":0.0,"max":1.0},
        {"id":"turn_ap","label":"Action Points per Turn","type":"integer","value":6,"min":1,"max":20},
        {"id":"turn_mp","label":"Move Points per Turn","type":"integer","value":3,"min":1,"max":20},
    ]))
}

async fn structures() -> Json<Value> {
    Json(json!([
        {"name":"oak_tree","size":[3,6,3],"block_count":14},
        {"name":"tall_oak_tree","size":[5,8,5],"block_count":30},
        {"name":"birch_tree","size":[3,6,3],"block_count":14},
        {"name":"boulder","size":[3,2,3],"block_count":12},
    ]))
}

async fn create_block(State(state): State<Arc<AppState>>, Json(body): Json<Value>) -> Json<Value> {
    let mut blocks = state.blocks.lock().unwrap();
    let mut new_block = body.clone();
    let next_id = blocks
        .iter()
        .filter_map(|b| b.get("id").and_then(|id| id.as_u64()))
        .max()
        .unwrap_or(0)
        + 1;
    new_block["id"] = json!(next_id);
    blocks.push(new_block.clone());
    Json(json!({"ok": true, "data": new_block}))
}

async fn update_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut blocks = state.blocks.lock().unwrap();
    if let Some(block) = blocks
        .iter_mut()
        .find(|b| b.get("id").and_then(|i| i.as_u64()) == Some(id))
    {
        *block = body;
        Json(json!({"ok": true, "data": block}))
    } else {
        Json(json!({"ok": false, "error": "Block not found"}))
    }
}

async fn create_item(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"ok": true, "data": body}))
}

async fn create_recipe(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"ok": true, "data": body}))
}

async fn create_npc(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"ok": true, "data": body}))
}

async fn create_quest(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"ok": true, "data": body}))
}

async fn update_rule(Path(id): Path<String>, Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"ok": true, "id": id, "data": body}))
}

async fn get_textures(State(state): State<Arc<AppState>>) -> Json<Value> {
    let textures = state.textures.lock().unwrap();
    Json(json!(*textures))
}

async fn upload_texture(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut textures = state.textures.lock().unwrap();
    let name = body
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let url = body.get("url").and_then(|v| v.as_str()).unwrap_or("");
    let new_texture = json!({ "name": name, "url": url, "assignedTo": "" });
    textures.push(new_texture.clone());
    Json(json!({"ok": true, "texture_id": name, "data": new_texture}))
}

fn block_schema() -> Value {
    json!({"fields":["id","name","texture_id","is_solid","is_transparent","hardness"]})
}
fn item_schema() -> Value {
    json!({"fields":["id","name","description","texture_id","stack_size","item_type"]})
}
fn recipe_schema() -> Value {
    json!({"fields":["id","name","inputs","output_item_id","output_quantity"]})
}
fn npc_schema() -> Value {
    json!({"fields":["id","name","traits","x","y","z","routine","memory"]})
}
fn mob_schema() -> Value {
    json!({"fields":["id","name","mob_type","can_talk","hp","max_hp","behavior","ai_config","loot_table"]})
}
fn quest_schema() -> Value {
    json!({"fields":["id","title","description","giver","objectives","rewards"]})
}
fn structure_schema() -> Value {
    json!({"fields":["name","size","blocks"]})
}
