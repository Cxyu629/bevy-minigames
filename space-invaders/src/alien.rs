use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{game, player, resolution::Resolution};

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AlienManager>()
            .add_systems(Startup, setup_aliens)
            .add_systems(
                Update,
                (
                    update_aliens_death,
                    update_aliens_movement,
                    update_player_interaction,
                )
                    .chain()
                    .run_if(in_state(game::GameState::InGame)),
            );
    }
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.0;
const SPEED: f32 = 100.0;
const ALIEN_SHIFT_AMOUNT: f32 = 24.0;
pub const ALIEN_SIZE: Vec2 = Vec2::new(9.0, 9.0);

#[derive(Component)]
pub struct Alien;
#[derive(Component)]
pub struct Dead;

#[derive(Resource)]
struct AlienManager {
    pub velocity: Vec3,
}

impl FromWorld for AlienManager {
    fn from_world(_world: &mut World) -> Self {
        Self {
            velocity: Vec3::X * SPEED,
        }
    }
}

fn setup_aliens(
    mut commands: Commands,
    resolution: Res<Resolution>,
    alien_texture: Res<game::AlienTexture>,
) {
    let alien_texture = alien_texture.0.clone();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new((x as f32 + 0.5) * SPACING, (y as f32 + 0.5) * SPACING, 0.0)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * SPACING * 1.0)
                + (Vec3::Y * resolution.size.y * 0.45);

            commands.spawn((
                Alien {
                    // spawn_position: position,
                },
                SpriteBundle {
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    texture: alien_texture.clone(),
                    ..default()
                },
            ));
        }
    }
}

fn update_aliens_movement(
    mut alien_query: Query<&mut Transform, (With<Alien>, Without<Dead>)>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<Resolution>,
    time: Res<Time>,
) {
    let mut out_of_bounds_flag = false;

    for mut alien_transform in &mut alien_query {
        alien_transform.translation += time.delta_seconds() * alien_manager.velocity;

        if alien_transform.translation.x.abs() + ALIEN_SIZE.x / 2.0 > resolution.size.x * 0.5 {
            out_of_bounds_flag = true;
        }
    }

    if out_of_bounds_flag {
        alien_manager.velocity *= -1.0 * Vec3::X;

        for mut alien_transform in &mut alien_query {
            alien_transform.translation += (time.delta_seconds() + f32::EPSILON)
                * alien_manager.velocity
                - ALIEN_SHIFT_AMOUNT * Vec3::Y;
        }
    }
}

fn update_aliens_death(mut alien_query: Query<&mut Visibility, (With<Alien>, With<Dead>)>) {
    for mut visibility in &mut alien_query {
        *visibility = Visibility::Hidden;
    }
}

fn update_player_interaction(
    mut next_state: ResMut<NextState<game::GameState>>,
    resolution: Res<Resolution>,
    player: Query<&Transform, With<player::Player>>,
    aliens: Query<&Transform, With<Alien>>,
) {
    let player_bounding_box =
        Aabb2d::new(player.single().translation.xy(), player::PLAYER_SIZE / 2.0);

    for alien_transform in &aliens {
        let alien_bounding_box = Aabb2d::new(alien_transform.translation.xy(), ALIEN_SIZE / 2.0);

        if player_bounding_box.intersects(&alien_bounding_box)
            || alien_transform.translation.y - ALIEN_SIZE.y / 2.0 <= -resolution.size.y / 2.0
        {
            next_state.set(game::GameState::GameOver)
        }
    }
}
