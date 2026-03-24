use tauri::State;
use crate::state::WorldStore;
use crate::commands::create_world::WorldRecord;

#[tauri::command]
pub async fn list_worlds(store: State<'_, WorldStore>) -> Result<Vec<WorldRecord>, String> {
    Ok(store.list().await)
}
