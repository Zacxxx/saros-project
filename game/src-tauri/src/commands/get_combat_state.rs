use tauri::State;
use crate::shared_state::{SharedGameState, CombatStateSnapshot};

#[tauri::command]
pub async fn get_combat_state(state: State<'_, SharedGameState>) -> Result<CombatStateSnapshot, String> {
    Ok(state.combat.read().unwrap().clone())
}
