use bevy::{math::bounding::Aabb2d, prelude::*};

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Resolution>();
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub size: Vec2,
    pub pixel_ratio: f32,
    pub bounding_box: Aabb2d,
}

impl FromWorld for Resolution {
    fn from_world(world: &mut World) -> Self {
        let window = world.query::<&Window>().single(world);
        Self {
            size: window.size(),
            pixel_ratio: 2.0,
            bounding_box: Aabb2d::new(Vec2::ZERO, window.size() / 2.0),
        }
    }
}
