use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{alien, game, resolution::Resolution};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_projectiles, update_aliens_interaction)
                .run_if(in_state(game::GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
}

fn update_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &Projectile, &mut Transform)>,
    resolution: Res<Resolution>,
    time: Res<Time>,
) {
    for (entity, projectile, mut transform) in &mut projectiles {
        transform.translation += projectile.velocity * time.delta_seconds();

        let projectile_bounding_box = Aabb2d::new(transform.translation.xy(), Vec2::ZERO);

        if !resolution.bounding_box.contains(&projectile_bounding_box) {
            commands.entity(entity).despawn();
        }
    }
}

fn update_aliens_interaction(
    mut commands: Commands,
    aliens: Query<(Entity, &Transform), (With<alien::Alien>, Without<alien::Dead>)>,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (alien_entity, alien_transform) in &aliens {
        for (projectile_entity, projectile_transform) in &projectiles {
            let alien_bounding_box =
                Aabb2d::new(alien_transform.translation.xy(), alien::ALIEN_SIZE);
            let projectile_bounding_box =
                Aabb2d::new(projectile_transform.translation.xy(), Vec2::ZERO);

            if alien_bounding_box.intersects(&projectile_bounding_box) {
                commands.entity(projectile_entity).despawn();
                commands.entity(alien_entity).insert(alien::Dead);
            }
        }
    }
}
