mod alien;
mod game;
mod player;
mod projectile;
mod resolution;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Invaders".to_string(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: Vec2::new(512.0, 512.0).into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
        .add_plugins(GamePlugin)
        .run();
}
