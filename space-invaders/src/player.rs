use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{game, projectile::Projectile, resolution::Resolution};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(
            Update,
            (update_player).run_if(in_state(game::GameState::InGame)),
        );
    }
}

const SPEED: f32 = 200.0;
const BULLET_SPEED: f32 = 400.0;
const SHOOT_COOLDOWN: f32 = 0.5;
pub const PLAYER_SIZE: Vec2 = Vec2::new(13.0, 9.0);

#[derive(Component)]
pub struct Player {
    pub shoot_timer: f32,
}

fn setup_player(
    mut commands: Commands,
    resolution: Res<Resolution>,
    player_texture: Res<game::PlayerTexture>,
) {
    let player_texture = player_texture.0.clone();

    commands.spawn((
        Player { shoot_timer: 0.0 },
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_xyz(
                0.0,
                -resolution.size.y * 0.5 + (PLAYER_SIZE.y * 0.5 + 2.0) * resolution.pixel_ratio,
                0.0,
            )
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ..default()
        },
    ));
}

fn update_player(
    mut commands: Commands,
    bullet_texture: Res<game::BulletTexture>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<Resolution>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut horizontal = 0.0;

    if keys.pressed(KeyCode::KeyA) {
        horizontal -= 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        horizontal += 1.0;
    }

    transform.translation.x += horizontal * time.delta_seconds() * SPEED;

    let left_bound = resolution.size.x * (-0.5);
    let right_bound = resolution.size.x * (0.5);

    if transform.translation.x < left_bound {
        transform.translation.x = left_bound;
    }
    if transform.translation.x > right_bound {
        transform.translation.x = right_bound;
    }

    player.shoot_timer -= time.delta_seconds();

    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0.0 {
        player.shoot_timer = SHOOT_COOLDOWN;

        let bullet_texture: Handle<Image> = bullet_texture.0.clone();
        let velocity = Vec3::new(horizontal * SPEED, BULLET_SPEED, 0.0);

        commands.spawn((
            Projectile { velocity },
            SpriteBundle {
                texture: bullet_texture,
                transform: Transform::from_translation(transform.translation)
                    .with_rotation(Quat::from_axis_angle(
                        Vec3::Z,
                        velocity.xy().to_angle() - PI / 2.0,
                    ))
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                ..default()
            },
        ));
    }
}
