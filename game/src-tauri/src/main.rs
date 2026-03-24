use saros_game_lib::shared_state::SharedGameState;
use saros_game_lib::state::WorldStore;

fn main() {
    let shared = SharedGameState::default();
    let shared_clone = shared.clone();

    // Axum HTTP server for devtools
    std::thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let app = saros_game_lib::api::router();
            let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    });

    std::thread::spawn(move || run_bevy(shared_clone));

    tauri::Builder::default()
        .manage(WorldStore::default())
        .manage(shared)
        .invoke_handler(tauri::generate_handler![
            saros_game_lib::commands::create_world::create_world,
            saros_game_lib::commands::load_world::load_world,
            saros_game_lib::commands::list_worlds::list_worlds,
            saros_game_lib::commands::get_hotbar::get_hotbar,
            saros_game_lib::commands::craft_item::get_recipes,
            saros_game_lib::commands::craft_item::craft_item,
            saros_game_lib::commands::get_combat_state::get_combat_state,
            saros_game_lib::commands::dialogue::talk_to_npc,
            saros_game_lib::commands::quests::get_quests,
            saros_game_lib::commands::quests::accept_quest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_bevy(shared: SharedGameState) {
    use bevy::prelude::*;
    use saros_game_lib::plugins::{CameraPlugin, CombatPlugin, NpcPlugin, PlayerPlugin, WorldPlugin};
    use saros_game_lib::systems::sync_inventory::{GameStateBridge, sync_inventory_to_shared};

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Saros".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameStateBridge(shared))
        .add_plugins((CameraPlugin, CombatPlugin, NpcPlugin, PlayerPlugin, WorldPlugin))
        .add_systems(Update, sync_inventory_to_shared)
        .run();
}
