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
}

pub fn router() -> Router {
    let state = Arc::new(AppState {
        blocks: Mutex::new(vec![
            json!({"id":1,"name":"Stone","texture_id":"","is_solid":true,"is_transparent":false,"hardness":1.5}),
            json!({"id":2,"name":"Dirt","texture_id":"","is_solid":true,"is_transparent":false,"hardness":0.5}),
            json!({"id":3,"name":"Grass","texture_id":"","is_solid":true,"is_transparent":false,"hardness":0.6}),
            json!({"id":4,"name":"Water","texture_id":"","is_solid":false,"is_transparent":true,"hardness":0.0}),
            json!({"id":5,"name":"Sand","texture_id":"","is_solid":true,"is_transparent":false,"hardness":0.5}),
            json!({"id":6,"name":"Log","texture_id":"","is_solid":true,"is_transparent":false,"hardness":2.0}),
            json!({"id":7,"name":"Leaf","texture_id":"","is_solid":false,"is_transparent":true,"hardness":0.2}),
        ]),
        textures: Mutex::new(vec![]),
    });

    Router::new()
        .route("/api/health", get(health))
        .route("/api/schemas", get(schemas))
        .route("/api/blocks", get(get_blocks).post(create_block))
        .route("/api/blocks/:id", put(update_block))
        .route("/api/items", get(items).post(create_item))
        .route("/api/recipes", get(recipes).post(create_recipe))
        .route("/api/npcs", get(npcs).post(create_npc))
        .route("/api/quests", get(quests).post(create_quest))
        .route("/api/rules", get(rules))
        .route("/api/rules/:id", put(update_rule))
        .route("/api/textures", get(get_textures).post(upload_texture))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn schemas() -> Json<Value> {
    Json(json!({
        "block":  block_schema(),
        "item":   item_schema(),
        "recipe": recipe_schema(),
        "npc":    npc_schema(),
        "quest":  quest_schema(),
    }))
}

async fn get_blocks(State(state): State<Arc<AppState>>) -> Json<Value> {
    let blocks = state.blocks.lock().unwrap();
    Json(json!(*blocks))
}

async fn items() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Stone","description":"A piece of stone","texture_id":"","stack_size":64,"item_type":"block"},
        {"id":6,"name":"Log","description":"A wooden log","texture_id":"","stack_size":64,"item_type":"block"},
        {"id":8,"name":"Wooden Plank","description":"Crafted from logs","texture_id":"","stack_size":64,"item_type":"block"},
        {"id":9,"name":"Stone Brick","description":"Crafted from stone","texture_id":"","stack_size":64,"item_type":"block"},
    ]))
}

async fn recipes() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Wooden Plank","inputs":[{"item_id":6,"quantity":2}],"output_item_id":8,"output_quantity":4},
        {"id":2,"name":"Stone Brick","inputs":[{"item_id":1,"quantity":4}],"output_item_id":9,"output_quantity":2},
    ]))
}

async fn npcs() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Aldric","traits":["wise","friendly"],"x":5.0,"y":1.0,"z":5.0},
        {"id":2,"name":"Mira","traits":["curious","cautious"],"x":-5.0,"y":1.0,"z":3.0},
    ]))
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

    let new_texture = json!({
        "name": name,
        "url": url,
        "assignedTo": ""
    });

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
fn quest_schema() -> Value {
    json!({"fields":["id","title","description","giver","objectives","rewards"]})
}
