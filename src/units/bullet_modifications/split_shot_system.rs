use bevy::prelude::{Commands, EventReader, EventWriter, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};
use rand::random;

use crate::{SpriteLayer, TextureHandles};
use crate::models::bullet::Bullet;
use crate::models::bundles::bullet_bundle::BulletBundle;
use crate::models::child_bullet::ChildBullet;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collision::EnemyHitBoxCollision;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::damaged_entities::DamagedEntities;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::move_direction::MoveDirection;
use crate::models::sprite_move_rotation::SpriteMoveRotation;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::damage_interval::DamageInterval;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_size::UnitSize;

/// A system to split the [Bullet] that has [SplitShot] applied to it.
/// The shot gets split when the bullet stops.
pub fn split_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    mut bullet_stopped_events: EventReader<BulletStoppedEvent>,
    bullet_query: Query<(&Transform, &Bullet, &DamagedEntities), With<SplitShot>>,
) {
    for event in bullet_stopped_events.iter() {
        let (bullet_transform, bullet, collided_entities) = match bullet_query.get(event.bullet_entity) {
            Ok(transform) => transform,
            Err(_) => continue,
        };

        let random_rotation = random::<f32>();

        let directions = vec![
            Vec2::new(1.0 - random_rotation, random_rotation).normalize(),
            Vec2::new(-random_rotation, 1.0 - random_rotation).normalize(),
            Vec2::new(-1.0 + random_rotation, -random_rotation).normalize(),
            Vec2::new(random_rotation, -1.0 + random_rotation).normalize(),
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
                .insert(ChildBullet)
                .insert(Name::new("SplitShot Bullet"))
                .insert(MoveSpeed::default())
                .insert(Damage::default())
                .insert(HitLimit::new(1.0))
                .insert(TravelRange::new(2048.0))
                .insert(SpriteMoveRotation)
                .insert(DamageInterval::new(60.0))
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(128.0 / 2.0) }).insert(EnemyHitBoxCollision)
                .insert(EnemyHitBoxCollision)
                .id();

            bullet_shot_event_writer.send(BulletShotEvent { entity: bullet });
        }
    }
}