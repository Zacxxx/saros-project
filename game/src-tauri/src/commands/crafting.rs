use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub item_id: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub inputs: Vec<RecipeIngredient>,
    pub output_item_id: u32,
    pub output_quantity: u32,
}

/// Static recipe registry — in production these come from SpacetimeDB.
pub fn default_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            id: 1,
            name: "Wooden Plank".into(),
            inputs: vec![RecipeIngredient { item_id: 6, quantity: 2 }], // LOG × 2
            output_item_id: 8,
            output_quantity: 4,
        },
        Recipe {
            id: 2,
            name: "Stone Brick".into(),
            inputs: vec![RecipeIngredient { item_id: 1, quantity: 4 }], // STONE × 4
            output_item_id: 9,
            output_quantity: 2,
        },
    ]
}
