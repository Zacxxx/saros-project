use tauri::State;
use serde::{Deserialize, Serialize};
use crate::shared_state::SharedGameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub giver: String,
    pub objectives: Vec<QuestObjective>,
    pub accepted: bool,
    pub completed: bool,
}

fn default_quests() -> Vec<Quest> {
    vec![
        Quest {
            id: 1,
            title: "Gather Stone".into(),
            description: "Aldric needs stone to repair the village wall.".into(),
            giver: "Aldric".into(),
            objectives: vec![
                QuestObjective { description: "Collect 10 stone blocks".into(), completed: false },
            ],
            accepted: false,
            completed: false,
        },
        Quest {
            id: 2,
            title: "Explore the Cave".into(),
            description: "Mira is curious about the cave to the north.".into(),
            giver: "Mira".into(),
            objectives: vec![
                QuestObjective { description: "Enter the cave".into(), completed: false },
                QuestObjective { description: "Return to Mira".into(), completed: false },
            ],
            accepted: false,
            completed: false,
        },
    ]
}

#[tauri::command]
pub async fn get_quests() -> Result<Vec<Quest>, String> {
    Ok(default_quests())
}

#[tauri::command]
pub async fn accept_quest(_quest_id: u64) -> Result<bool, String> {
    Ok(true)
}
