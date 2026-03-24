use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnPhase {
    PlayerInput,
    PlayerMoving,
    EnemyTurn,
}

#[derive(Resource)]
pub struct TurnState {
    pub phase: TurnPhase,
    pub round: u32,
}

impl Default for TurnState {
    fn default() -> Self {
        Self { phase: TurnPhase::PlayerInput, round: 1 }
    }
}
