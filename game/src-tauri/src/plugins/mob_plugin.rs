use crate::components::Player;
use crate::components::mob::{LootEntry, MobBehavior, MobComponent, MobType};
use bevy::prelude::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_mobs)
            .add_systems(Update, (mob_ai_behavior, mob_wander));
    }
}

fn spawn_initial_mobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a few passive mobs (sheep-like)
    let mob_configs: Vec<(u64, &str, MobType, [f32; 3], Color, i32, Vec<LootEntry>)> = vec![
        (
            100,
            "Sheep",
            MobType::Passive,
            [8.0, 1.0, 8.0],
            Color::rgb(0.9, 0.9, 0.9),
            8,
            vec![LootEntry {
                item_id: 20,
                quantity_min: 1,
                quantity_max: 2,
                chance: 1.0,
            }],
        ),
        (
            101,
            "Cow",
            MobType::Passive,
            [12.0, 1.0, 5.0],
            Color::rgb(0.55, 0.35, 0.2),
            10,
            vec![
                LootEntry {
                    item_id: 18,
                    quantity_min: 1,
                    quantity_max: 3,
                    chance: 1.0,
                },
                LootEntry {
                    item_id: 20,
                    quantity_min: 0,
                    quantity_max: 2,
                    chance: 0.5,
                },
            ],
        ),
        (
            102,
            "Pig",
            MobType::Passive,
            [-8.0, 1.0, 10.0],
            Color::rgb(0.9, 0.7, 0.6),
            8,
            vec![LootEntry {
                item_id: 18,
                quantity_min: 1,
                quantity_max: 3,
                chance: 1.0,
            }],
        ),
        (
            103,
            "Zombie",
            MobType::Hostile,
            [20.0, 1.0, 20.0],
            Color::rgb(0.3, 0.5, 0.3),
            20,
            vec![
                LootEntry {
                    item_id: 18,
                    quantity_min: 0,
                    quantity_max: 1,
                    chance: 0.3,
                },
                LootEntry {
                    item_id: 12,
                    quantity_min: 0,
                    quantity_max: 1,
                    chance: 0.1,
                },
            ],
        ),
        (
            104,
            "Skeleton",
            MobType::Hostile,
            [-15.0, 1.0, -15.0],
            Color::rgb(0.85, 0.85, 0.8),
            20,
            vec![
                LootEntry {
                    item_id: 20,
                    quantity_min: 0,
                    quantity_max: 2,
                    chance: 0.5,
                },
                LootEntry {
                    item_id: 11,
                    quantity_min: 1,
                    quantity_max: 2,
                    chance: 0.8,
                },
            ],
        ),
        (
            105,
            "Wolf",
            MobType::Neutral,
            [15.0, 1.0, -10.0],
            Color::rgb(0.75, 0.72, 0.68),
            12,
            vec![],
        ),
    ];

    for (id, name, mob_type, pos, color, hp, loot) in mob_configs {
        let size = if mob_type == MobType::Hostile {
            (0.6, 1.8, 0.6)
        } else {
            (0.8, 0.8, 1.2)
        };
        commands.spawn((
            MobComponent {
                id,
                name: name.into(),
                mob_type,
                can_talk: mob_type == MobType::Neutral,
                ai_thoughts: "...".into(),
                thought_timer: 0.0,
                loot_table: loot,
                hp,
                max_hp: hp,
                behavior: MobBehavior::Idle,
                wander_timer: 0.0,
                wander_target: None,
            },
            PbrBundle {
                mesh: meshes.add(Cuboid::new(size.0, size.1, size.2)),
                material: materials.add(color),
                transform: Transform::from_xyz(pos[0], pos[1], pos[2]),
                ..default()
            },
        ));
    }
}

fn mob_ai_behavior(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut mobs: Query<(&mut MobComponent, &Transform), Without<Player>>,
) {
    let Ok(player_tf) = player.get_single() else {
        return;
    };
    let dt = time.delta_seconds();

    for (mut mob, mob_tf) in mobs.iter_mut() {
        if !mob.is_alive() {
            continue;
        }

        let dist = mob_tf.translation.distance(player_tf.translation);

        // AI thought generation timer
        mob.thought_timer += dt;
        if mob.thought_timer > 5.0 {
            mob.thought_timer = 0.0;
            mob.ai_thoughts = match mob.mob_type {
                MobType::Passive => {
                    if dist < 5.0 {
                        "A player is nearby... should I run?".into()
                    } else {
                        "Grazing peacefully...".into()
                    }
                }
                MobType::Hostile => {
                    if dist < 15.0 {
                        "I sense prey nearby...".into()
                    } else {
                        "Patrolling my territory...".into()
                    }
                }
                MobType::Neutral => {
                    if dist < 3.0 {
                        "This one seems friendly...".into()
                    } else {
                        "Watching from a distance...".into()
                    }
                }
            };
        }

        // Behavior state transitions
        match mob.mob_type {
            MobType::Passive => {
                mob.behavior = if dist < 4.0 {
                    MobBehavior::Flee
                } else {
                    MobBehavior::Wander
                };
            }
            MobType::Hostile => {
                mob.behavior = if dist < 16.0 {
                    MobBehavior::Chase
                } else {
                    MobBehavior::Wander
                };
            }
            MobType::Neutral => {
                mob.behavior = MobBehavior::Wander;
            }
        }
    }
}

fn mob_wander(time: Res<Time>, mut mobs: Query<(&mut MobComponent, &mut Transform)>) {
    let dt = time.delta_seconds();

    for (mut mob, mut tf) in mobs.iter_mut() {
        if !mob.is_alive() {
            continue;
        }

        match mob.behavior {
            MobBehavior::Wander => {
                mob.wander_timer += dt;
                if mob.wander_timer > 3.0 || mob.wander_target.is_none() {
                    mob.wander_timer = 0.0;
                    let angle = (mob.id as f32 + tf.translation.x) * 0.7;
                    let target =
                        tf.translation + Vec3::new(angle.cos() * 3.0, 0.0, angle.sin() * 3.0);
                    mob.wander_target = Some(target);
                }
                if let Some(target) = mob.wander_target {
                    let dir = (target - tf.translation).normalize_or_zero();
                    tf.translation += dir * 1.5 * dt;
                }
            }
            MobBehavior::Idle => {}
            MobBehavior::Chase | MobBehavior::Flee => {
                // Chase/flee handled by mob_ai_behavior setting wander_target
            }
        }
    }
}
