use tauri::State;
use crate::state::WorldStore;

#[tauri::command]
pub async fn load_world(id: String, store: State<'_, WorldStore>) -> Result<(), String> {
    store.set_active(id).await;
    Ok(())
}
