use tauri::State;
use crate::shared_state::{SharedGameState, HotbarSlot};

#[derive(serde::Serialize)]
pub struct HotbarState {
    pub slots: Vec<HotbarSlot>,
    pub active_slot: usize,
}

#[tauri::command]
pub async fn get_hotbar(state: State<'_, SharedGameState>) -> Result<HotbarState, String> {
    let slots = state.hotbar.read().unwrap().to_vec();
    let active_slot = *state.active_slot.read().unwrap();
    Ok(HotbarState { slots, active_slot })
}
