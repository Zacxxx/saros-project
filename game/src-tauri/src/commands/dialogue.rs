use tauri::State;
use serde::{Deserialize, Serialize};
use crate::shared_state::SharedGameState;

#[derive(Serialize)]
struct ChatMessage { role: String, content: String }

#[derive(Serialize)]
struct ChatRequest { model: String, messages: Vec<ChatMessage> }

#[derive(Deserialize)]
struct ChatChoice { message: ChatMessageResponse }

#[derive(Deserialize)]
struct ChatMessageResponse { content: String }

#[derive(Deserialize)]
struct ChatResponse { choices: Vec<ChatChoice> }

#[tauri::command]
pub async fn talk_to_npc(
    npc_id: u64,
    message: String,
    state: State<'_, SharedGameState>,
) -> Result<String, String> {
    let npc = {
        let npcs = state.npcs.read().unwrap();
        npcs.get(&npc_id).cloned()
    };
    let Some(npc) = npc else { return Err("NPC not found".into()); };

    let api_key = std::env::var("AI_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        // Fallback when no API key configured
        let response = format!("*{}* nods thoughtfully. \"{}\"", npc.name, placeholder_response(&npc.traits));
        append_memory(&state, npc_id, "user", &message);
        append_memory(&state, npc_id, "assistant", &response);
        return Ok(response);
    }

    let system_prompt = format!(
        "You are {}, an NPC in a fantasy world. Traits: {}. Current thought: \"{}\". Stay in character. Be concise.",
        npc.name,
        npc.traits.join(", "),
        npc.current_thought,
    );

    let mut messages = vec![ChatMessage { role: "system".into(), content: system_prompt }];
    for (role, content) in &npc.memory {
        messages.push(ChatMessage { role: role.clone(), content: content.clone() });
    }
    messages.push(ChatMessage { role: "user".into(), content: message.clone() });

    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&ChatRequest { model: "gpt-4o-mini".into(), messages })
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<ChatResponse>()
        .await
        .map_err(|e| e.to_string())?;

    let response = resp.choices.into_iter().next()
        .map(|c| c.message.content)
        .unwrap_or_default();

    append_memory(&state, npc_id, "user", &message);
    append_memory(&state, npc_id, "assistant", &response);
    Ok(response)
}

fn append_memory(state: &SharedGameState, npc_id: u64, role: &str, content: &str) {
    let mut npcs = state.npcs.write().unwrap();
    if let Some(npc) = npcs.get_mut(&npc_id) {
        npc.memory.push((role.into(), content.into()));
        if npc.memory.len() > 20 { npc.memory.remove(0); }
    }
}

fn placeholder_response(traits: &[String]) -> &'static str {
    if traits.iter().any(|t| t == "wise") { "The path ahead holds many secrets." }
    else if traits.iter().any(|t| t == "curious") { "I wonder what lies beyond those hills..." }
    else { "Greetings, traveler." }
}
