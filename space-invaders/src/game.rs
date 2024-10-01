use bevy::prelude::*;

use crate::{
    alien::AlienPlugin, player::PlayerPlugin, projectile::ProjectilePlugin,
    resolution::ResolutionPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(PreStartup, setup_game)
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

#[derive(Resource)]
pub struct AlienTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct BulletTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct PlayerTexture(pub Handle<Image>);

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bullet_texture = asset_server.load::<Image>("bullet.png");
    let alien_texture = asset_server.load::<Image>("alien.png");
    let player_texture = asset_server.load::<Image>("player.png");

    commands.insert_resource(AlienTexture(alien_texture));
    commands.insert_resource(BulletTexture(bullet_texture));
    commands.insert_resource(PlayerTexture(player_texture));

    commands.spawn(Camera2dBundle { ..default() });
}
