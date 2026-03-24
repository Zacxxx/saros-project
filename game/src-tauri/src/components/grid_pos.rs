use bevy::prelude::*;

/// Integer grid position (x, z) on the voxel grid.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct GridPos(pub i32, pub i32);
