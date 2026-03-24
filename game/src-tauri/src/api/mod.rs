use axum::{Router, Json, extract::Path};
use axum::routing::{get, post, put};
use tower_http::cors::CorsLayer;
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/api/health",       get(health))
        .route("/api/schemas",      get(schemas))
        .route("/api/blocks",       get(blocks).post(create_block))
        .route("/api/items",        get(items).post(create_item))
        .route("/api/recipes",      get(recipes).post(create_recipe))
        .route("/api/npcs",         get(npcs).post(create_npc))
        .route("/api/quests",       get(quests).post(create_quest))
        .route("/api/rules",        get(rules))
        .route("/api/rules/:id",    put(update_rule))
        .route("/api/textures",     post(upload_texture))
        .layer(CorsLayer::permissive())
}

async fn health() -> Json<Value> { Json(json!({"status": "ok"})) }

async fn schemas() -> Json<Value> {
    Json(json!({
        "block":  block_schema(),
        "item":   item_schema(),
        "recipe": recipe_schema(),
        "npc":    npc_schema(),
        "quest":  quest_schema(),
    }))
}

async fn blocks() -> Json<Value> {
    Json(json!([
        {"id":1,"name":"Stone","texture_id":"","is_solid":true,"is_transparent":false,"hardness":1.5},
        {"id":2,"name":"Dirt","texture_id":"","is_solid":true,"is_transparent":false,"hardness":0.5},
        {"id":3,"name":"Grass","texture_id":"","is_solid":true,"is_transparent":false,"hardness":0.6},
        {"id":4,"name":"Water","texture_id":"","is_solid":false,"is_transparent":true,"hardness":0.0},
        {"id":5,"name":"Sand","texture_id":"","is_solid":true,"is_transparent":false,"hardness":0.5},
        {"id":6,"name":"Log","texture_id":"","is_solid":true,"is_transparent":false,"hardness":2.0},
        {"id":7,"name":"Leaf","texture_id":"","is_solid":false,"is_transparent":true,"hardness":0.2},
    ]))
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

async fn create_block(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"ok": true, "data": body}))
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

async fn upload_texture(Json(body): Json<Value>) -> Json<Value> {
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
    Json(json!({"ok": true, "texture_id": name}))
}

fn block_schema() -> Value { json!({"fields":["id","name","texture_id","is_solid","is_transparent","hardness"]}) }
fn item_schema()  -> Value { json!({"fields":["id","name","description","texture_id","stack_size","item_type"]}) }
fn recipe_schema()-> Value { json!({"fields":["id","name","inputs","output_item_id","output_quantity"]}) }
fn npc_schema()   -> Value { json!({"fields":["id","name","traits","x","y","z","routine","memory"]}) }
fn quest_schema() -> Value { json!({"fields":["id","title","description","giver","objectives","rewards"]}) }
