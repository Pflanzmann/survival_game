use bevy::app::EventReader;
use bevy::prelude::{Commands, Query, Res, Sprite, SpriteBundle, Vec2, Vec3};
use rand::random;

use crate::{Collider, FacingDirection, MoveSpeed, TextureHandles, Transform, UnitSize, With};
use crate::components::bullet_components::{Bullet, BulletRange};
use crate::components::collision_components::CollidedEntities;
use crate::components::event_components::BulletStoppedEvent;
use crate::components::modification_components::SplitShot;

pub fn split_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    mut bullet_stopped_events: EventReader<BulletStoppedEvent>,
    bullet_query: Query<(&Transform, &Bullet), With<SplitShot>>,
) {
    for event in bullet_stopped_events.iter() {
        let (bullet_transform, bullet) = match bullet_query.get(event.bullet_entity) {
            Ok(transform) => transform,
            Err(_) => continue,
        };

        let random_rotation = random::<f32>();

        let directions = vec![
            Vec3::new(1.0 - random_rotation, random_rotation, 0.0).normalize(),
            Vec3::new(-random_rotation, 1.0 - random_rotation, 0.0).normalize(),
            Vec3::new(-1.0 + random_rotation, -random_rotation, 0.0).normalize(),
            Vec3::new(random_rotation, -1.0 + random_rotation, 0.0).normalize(),
        ];

        for direction in directions {
            command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(bullet_transform.translation.x, bullet_transform.translation.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.bullet_fireball.clone(),
                ..Default::default()
            })
                .insert(bullet.clone())
                .insert(Collider)
                .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
                .insert(FacingDirection { facing_direction: direction })
                .insert(MoveSpeed { move_speed: 15.0 })
                .insert(BulletRange::new(2048.0))
                .insert(CollidedEntities { collisions: Vec::new() });
        }
    }
}