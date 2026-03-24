use bevy::prelude::*;
use crate::components::{Player, Inventory, CombatStats, NpcComponent};
use crate::shared_state::{SharedGameState, HotbarSlot, CombatStateSnapshot, NpcSnapshot};
use crate::plugins::combat::TurnState;

/// Bevy resource holding the shared state handle.
#[derive(Resource, Clone)]
pub struct GameStateBridge(pub SharedGameState);

/// Syncs Bevy state → SharedGameState every frame.
pub fn sync_inventory_to_shared(
    player: Query<(&Inventory, &CombatStats), With<Player>>,
    turn: Res<TurnState>,
    npcs: Query<&NpcComponent>,
    bridge: Res<GameStateBridge>,
) {
    if let Ok((inv, stats)) = player.get_single() {
        let mut hotbar = bridge.0.hotbar.write().unwrap();
        for i in 0..9 {
            hotbar[i] = HotbarSlot { item_id: inv.item_ids[i], quantity: inv.quantities[i] };
        }
        *bridge.0.active_slot.write().unwrap() = inv.hotbar_slot;
        *bridge.0.combat.write().unwrap() = CombatStateSnapshot {
            hp: stats.hp, max_hp: stats.max_hp,
            ap: stats.ap, max_ap: stats.max_ap,
            mp: stats.mp, max_mp: stats.max_mp,
            round: turn.round,
        };
    }

    let mut npc_map = bridge.0.npcs.write().unwrap();
    for npc in &npcs {
        npc_map.insert(npc.id, NpcSnapshot {
            id: npc.id,
            name: npc.name.clone(),
            traits: npc.traits.clone(),
            memory: npc.memory.clone(),
            current_thought: npc.current_thought.clone(),
        });
    }
}
