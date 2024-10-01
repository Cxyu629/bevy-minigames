use bevy::prelude::*;

use crate::{
    alien::AlienPlugin, player::PlayerPlugin, projectile::ProjectilePlugin,
    resolution::ResolutionPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup_game)
            .add_plugins((
                ResolutionPlugin,
                ProjectilePlugin,
                PlayerPlugin,
                AlienPlugin,
            ));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    InGame,
    GameOver,
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
