use tauri::State;
use crate::commands::crafting::{Recipe, default_recipes};
use crate::shared_state::SharedGameState;

#[tauri::command]
pub async fn get_recipes() -> Result<Vec<Recipe>, String> {
    Ok(default_recipes())
}

#[tauri::command]
pub async fn craft_item(recipe_id: u32, state: State<'_, SharedGameState>) -> Result<bool, String> {
    let recipes = default_recipes();
    let Some(recipe) = recipes.iter().find(|r| r.id == recipe_id) else {
        return Err("Recipe not found".into());
    };

    // Check + consume ingredients from hotbar (simplified: checks first 9 slots)
    let mut hotbar = state.hotbar.write().unwrap();

    // Verify all inputs available
    for input in &recipe.inputs {
        let total: u32 = hotbar.iter()
            .filter(|s| s.item_id == input.item_id)
            .map(|s| s.quantity)
            .sum();
        if total < input.quantity {
            return Ok(false);
        }
    }

    // Consume inputs
    for input in &recipe.inputs {
        let mut remaining = input.quantity;
        for slot in hotbar.iter_mut() {
            if slot.item_id == input.item_id && remaining > 0 {
                let take = remaining.min(slot.quantity);
                slot.quantity -= take;
                remaining -= take;
                if slot.quantity == 0 { slot.item_id = 0; }
            }
        }
    }

    // Add output to first empty slot
    for slot in hotbar.iter_mut() {
        if slot.item_id == 0 {
            slot.item_id = recipe.output_item_id;
            slot.quantity = recipe.output_quantity;
            return Ok(true);
        }
    }

    Ok(false)
}
