use bevy::app::EventWriter;
use bevy::prelude::{Commands, Res, Sprite, SpriteBundle, Time, Vec2};

use crate::{Damage, Entity, Query, TextureHandles, Transform, With};
use crate::models::bullet_components::{Bullet, BulletRange, HitLimit};
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider::Collider;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::gun_components::{Reloadable, StraightBasicShot, WeaponSlot};
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_stats_components::{FacingDirection, MoveSpeed, UnitSize};

pub fn straight_basic_shot_system(
    mut command: Commands,
    time: Res<Time>,
    texture_handle: Res<TextureHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    weapon_holder: Query<(&Transform, &FacingDirection, &WeaponSlot)>,
    mut gun_query: Query<(Entity, &mut Reloadable), With<StraightBasicShot>>,
) {
    for (holder_transform, holder_facing_direction, holder_weapon_slot) in weapon_holder.iter() {
        let (gun_entity, mut gun_reloadable) = match gun_query.get_mut(holder_weapon_slot.weapon_entity) {
            Ok(gun) => gun,
            Err(_) => continue,
        };

        gun_reloadable.reload_timer -= time.delta().as_secs_f32();
        if gun_reloadable.reload_timer > 0.0 {
            continue;
        }
        gun_reloadable.reload_timer = gun_reloadable.base_reloading_time;

        let bullet = command.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(holder_transform.translation.x, holder_transform.translation.y, SpriteLayer::LowGroundLevel.get_layer_z()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            texture: texture_handle.bullet_fireball.clone(),
            ..Default::default()
        })
            .insert(Bullet { source_entity: gun_entity })
            .insert(Collider)
            .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
            .insert(FacingDirection { facing_direction: holder_facing_direction.facing_direction })
            .insert(MoveSpeed { move_speed: 15.0 })
            .insert(Damage::new(5.0))
            .insert(BulletRange::new(2048.0))
            .insert(CollidedEntities { collisions: Vec::new() })
            .insert(HitLimit { hit_limit: 1 })
            .id();

        bullet_shot_event_writer.send(BulletShotEvent { entity: bullet })
    }
}