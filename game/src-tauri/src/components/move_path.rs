use bevy::prelude::*;
use std::collections::VecDeque;

/// Queued grid positions the entity should walk through.
#[derive(Component, Default)]
pub struct MovePath {
    pub steps: VecDeque<(i32, i32)>,
    pub timer: f32,
}
