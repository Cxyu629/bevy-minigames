use bevy::prelude::*;

pub fn coords_to_translation(board_size: Vec2, grid_size: Vec2, coords: Vec2) -> Vec3 {
    let position = ((coords + Vec2::splat(0.5)) - 0.5 * board_size) * grid_size;
    Vec3 {
        x: position.x,
        y: position.y,
        z: 0.0,
    }
}
