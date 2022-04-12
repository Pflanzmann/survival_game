use bevy::app::EventReader;
use bevy::prelude::{Commands, Query, Res, Sprite, SpriteBundle, Vec2, Vec3};
use rand::random;

use crate::{SpriteLayer, TextureHandles, Transform, With};
use crate::models::bullet_components::Bullet;
use crate::models::bundles::bullet_bundle::BulletBundle;
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider::Collider;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::modification_components::SplitShot;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_stats_components::{MoveDirection, UnitSize};

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
                move_speed: MoveSpeed::new(15.0),
                damage: Damage::new(5.0),
                travel_range: TravelRange::new(2048.0),
                hit_limit: HitLimit::new(1.0),
                collider: Collider,
                collider_entities: CollidedEntities::default(),
            });
        }
    }
}