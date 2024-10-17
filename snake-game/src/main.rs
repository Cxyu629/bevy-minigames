use bevy::{prelude::*, window::WindowResolution};

pub mod food;
mod game;
mod gametick;
mod snake;
pub mod utils;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1000.0, 1000.0)
                            .with_scale_factor_override(PIXEL_SCALE),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(GamePlugin)
        .run();
}

const PIXEL_SCALE: f32 = 2.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
