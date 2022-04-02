use bevy::app::EventReader;
use bevy::prelude::{Commands, Query, Res, Sprite, SpriteBundle, Vec2, Vec3};
use rand::random;

use crate::{Collider, FacingDirection, MoveSpeed, TextureHandles, Transform, UnitSize, With};
use crate::components::bullet_components::{Bullet, BulletRange};
use crate::components::collision_components::CollidedEntities;
use crate::components::event_components::BulletShotEvent;
use crate::components::modification_components::{CurveShot, SplitShot};

pub fn apply_split_shot_system(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
) {
    for event in bullet_shot_event.iter() {
        commands.entity(event.entity).insert(SplitShot);
    }
}

pub fn split_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    mut bullet_query: Query<(&Transform, &BulletRange), With<SplitShot>>,
) {
    for (bullet_transform, bullet_range) in bullet_query.iter_mut() {
        if bullet_range.distance_traveled >= bullet_range.total_range {
            command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(bullet_transform.translation.x, bullet_transform.translation.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.bullet_fireball.clone(),
                ..Default::default()
            })
                .insert(Bullet)
                .insert(Collider)
                .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
                .insert(FacingDirection { facing_direction: Vec3::new(1.0, 0.0, 0.0) })
                .insert(MoveSpeed { move_speed: 15.0 })
                .insert(BulletRange::new(2048.0))
                .insert(CollidedEntities { collisions: Vec::new() });
        }

        if bullet_range.distance_traveled >= bullet_range.total_range {
            command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(bullet_transform.translation.x, bullet_transform.translation.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.bullet_fireball.clone(),
                ..Default::default()
            })
                .insert(Bullet)
                .insert(Collider)
                .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
                .insert(FacingDirection { facing_direction: Vec3::new(-1.0, 0.0, 0.0) })
                .insert(MoveSpeed { move_speed: 15.0 })
                .insert(BulletRange::new(2048.0))
                .insert(CollidedEntities { collisions: Vec::new() });
        }

        if bullet_range.distance_traveled >= bullet_range.total_range {
            command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(bullet_transform.translation.x, bullet_transform.translation.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.bullet_fireball.clone(),
                ..Default::default()
            })
                .insert(Bullet)
                .insert(Collider)
                .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
                .insert(FacingDirection { facing_direction: Vec3::new(0.0, 1.0, 0.0) })
                .insert(MoveSpeed { move_speed: 15.0 })
                .insert(BulletRange::new(2048.0))
                .insert(CollidedEntities { collisions: Vec::new() });
        }

        if bullet_range.distance_traveled >= bullet_range.total_range {
            command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(bullet_transform.translation.x, bullet_transform.translation.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.bullet_fireball.clone(),
                ..Default::default()
            })
                .insert(Bullet)
                .insert(Collider)
                .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
                .insert(FacingDirection { facing_direction: Vec3::new(0.0, -1.0, 0.0) })
                .insert(MoveSpeed { move_speed: 15.0 })
                .insert(BulletRange::new(2048.0))
                .insert(CollidedEntities { collisions: Vec::new() });
        }
    }
}