use bevy::prelude::*;
use crate::components::NpcComponent;
use crate::systems::sync_inventory::GameStateBridge;

const THOUGHT_INTERVAL: f32 = 30.0;

/// Periodically generates a new thought for each NPC via LLM API.
pub fn npc_thought_tick(
    time: Res<Time>,
    bridge: Res<GameStateBridge>,
    mut npcs: Query<&mut NpcComponent>,
) {
    let dt = time.delta_seconds();
    let api_key = std::env::var("AI_API_KEY").unwrap_or_default();

    for mut npc in &mut npcs {
        npc.thought_timer += dt;
        if npc.thought_timer < THOUGHT_INTERVAL { continue; }
        npc.thought_timer = 0.0;

        let name = npc.name.clone();
        let traits = npc.traits.clone();
        let npc_id = npc.id;
        let bridge_clone = bridge.0.clone();
        let key = api_key.clone();

        // Spawn OS thread for blocking LLM call
        std::thread::spawn(move || {
            let thought = if key.is_empty() {
                generate_placeholder_thought(&traits)
            } else {
                generate_thought_blocking(&name, &traits, &key)
            };
            // Write thought back to shared state; Bevy will read it next sync
            let mut npcs = bridge_clone.npcs.write().unwrap();
            if let Some(snap) = npcs.get_mut(&npc_id) {
                snap.current_thought = thought;
            }
        });
    }
}

fn generate_placeholder_thought(traits: &[String]) -> String {
    if traits.iter().any(|t| t == "wise") {
        "I should check on the eastern ruins today.".into()
    } else {
        "I wonder if there are mushrooms near the old oak.".into()
    }
}

fn generate_thought_blocking(name: &str, traits: &[String], api_key: &str) -> String {
    use std::collections::HashMap;

    let prompt = format!(
        "You are {}. Traits: {}. Generate one short internal thought (1 sentence) about what you want to do next.",
        name, traits.join(", ")
    );

    let mut body = HashMap::new();
    body.insert("model", "gpt-4o-mini".to_string());
    body.insert("messages", serde_json::to_string(&[
        serde_json::json!({"role": "user", "content": prompt})
    ]).unwrap_or_default());

    // Use blocking reqwest
    let client = reqwest::blocking::Client::new();
    let Ok(resp) = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 60
        }))
        .send() else { return generate_placeholder_thought(traits); };

    let Ok(json) = resp.json::<serde_json::Value>() else { return generate_placeholder_thought(traits); };
    json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_string()
}
