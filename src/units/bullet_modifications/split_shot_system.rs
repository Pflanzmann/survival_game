use bevy::prelude::{Commands, EventReader, EventWriter, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};
use rand::random;

use crate::{SpriteLayer, TextureHandles};
use crate::models::bullet::Bullet;
use crate::models::bundles::bullet_bundle::BulletBundle;
use crate::models::child_bullet::ChildBullet;
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider_type::ColliderType;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::move_direction::MoveDirection;
use crate::models::sprite_move_rotation::SpriteMoveRotation;
use crate::models::unit_size::UnitSize;

/// A system to split the [Bullet] that has [SplitShot] applied to it.
/// The shot gets split when the bullet stops.
pub fn split_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    mut bullet_stopped_events: EventReader<BulletStoppedEvent>,
    bullet_query: Query<(&Transform, &Bullet, &CollidedEntities), With<SplitShot>>,
) {
    for event in bullet_stopped_events.iter() {
        let (bullet_transform, bullet, collided_entities) = match bullet_query.get(event.bullet_entity) {
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
            let bullet = command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(bullet_transform.translation.x, bullet_transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z()),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handle.bullet_fireball.clone(),
                ..Default::default()
            }).insert_bundle(BulletBundle {
                bullet: *bullet,
                unit_size: UnitSize { collider_size: Vec2::new(128.0, 128.0) },
                facing_direction: MoveDirection { direction },
                collider_entities: collided_entities.clone(),
            })
                .insert(Name::new("Bullet"))
                .insert(ChildBullet)
                .insert(SpriteMoveRotation)
                .insert(ColliderType::Circle(128.0))
                .id();

            bullet_shot_event_writer.send(BulletShotEvent { entity: bullet });
        }
    }
}