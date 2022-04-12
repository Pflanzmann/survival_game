use bevy::app::EventWriter;
use bevy::prelude::{Commands, Res, Sprite, SpriteBundle, Vec2};

use crate::{Entity, Query, TextureHandles, Transform, With};
use crate::assets_handling::preload_bullet_system::BulletConfigHandles;
use crate::models::bullet_components::Bullet;
use crate::models::bundles::bullet_bundle::BulletBundle;
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider::Collider;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::gun_components::{Reloadable, StraightBasicShot, WeaponSlot};
use crate::models::player_components::AimDirection;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_stats_components::{MoveDirection, UnitSize};

pub fn straight_basic_shot_system(
    mut command: Commands,
    texture_handle: Res<TextureHandles>,
    bullet_handle: Res<BulletConfigHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    weapon_holder: Query<(&Transform, &AimDirection, &WeaponSlot)>,
    mut gun_query: Query<(Entity, &mut Reloadable), With<StraightBasicShot>>,
) {
    for (holder_transform, holder_aim_direction, holder_weapon_slot) in weapon_holder.iter() {
        if holder_aim_direction.direction.length() == 0.0 {
            continue;
        }

        let (gun_entity, mut gun_reloadable) = match gun_query.get_mut(holder_weapon_slot.weapon_entity) {
            Ok(gun) => gun,
            Err(_) => continue,
        };

        if gun_reloadable.reload_timer > 0.0 {
            continue;
        }
        gun_reloadable.reload_timer = gun_reloadable.base_reloading_time;

        let bullet = command.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(holder_transform.translation.x, holder_transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(bullet_handle.basic_bullet.sprite_custom_size_x, bullet_handle.basic_bullet.sprite_custom_size_y)),
                ..Default::default()
            },
            texture: texture_handle.bullet_fireball.clone(),
            ..Default::default()
        })
            .insert_bundle(BulletBundle {
                bullet: Bullet { source_entity: gun_entity },
                unit_size: UnitSize { collider_size: Vec2::new(bullet_handle.basic_bullet.sprite_custom_size_x, bullet_handle.basic_bullet.sprite_custom_size_y) },
                facing_direction: MoveDirection { direction: holder_aim_direction.direction },
                move_speed: MoveSpeed::new(bullet_handle.basic_bullet.speed),
                damage: Damage::new(bullet_handle.basic_bullet.damage),
                travel_range: TravelRange::new(bullet_handle.basic_bullet.range),
                hit_limit: HitLimit::new(bullet_handle.basic_bullet.hit_limit),
                collider: Collider,
                collider_entities: CollidedEntities::default(),
            }).id();

        bullet_shot_event_writer.send(BulletShotEvent { entity: bullet })
    }
}