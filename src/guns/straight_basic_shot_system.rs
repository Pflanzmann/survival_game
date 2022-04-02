use bevy::app::EventWriter;
use bevy::prelude::{AssetServer, Commands, Res, Sprite, SpriteBundle, Time, Vec2};

use crate::{Entity, Query, TextureHandles, Transform, With};
use crate::components::bullet_components::{Bullet, BulletRange};
use crate::components::collision_components::{CollidedEntities, Collider};
use crate::components::event_components::BulletShotEvent;
use crate::components::gun_components::{Reloadable, StraightBasicShot, WeaponSlot};
use crate::components::unit_stats_components::{FacingDirection, MoveSpeed, UnitSize};

pub fn straight_basic_shot_system(
    mut command: Commands,
    time: Res<Time>,
    texture_handle: Res<TextureHandles>,
    mut bullet_shot_event_writer: EventWriter<BulletShotEvent>,
    player_query: Query<(&Transform, &FacingDirection, &WeaponSlot)>,
    mut gun_query: Query<(Entity, &mut Reloadable), With<StraightBasicShot>>,
) {
    for (transform, direction, weapon_slot) in player_query.iter() {
        for (entity, mut reloadable) in gun_query.iter_mut() {
            if weapon_slot.entity != entity {
                continue;
            }

            reloadable.reload_timer -= time.delta().as_secs_f32();
            if reloadable.reload_timer > 0.0 {
                continue;
            }
            reloadable.reload_timer = reloadable.base_reloading_time;

            let bullet = command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
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
                .insert(FacingDirection { facing_direction: direction.facing_direction })
                .insert(MoveSpeed { move_speed: 15.0 })
                .insert(BulletRange::new(2048.0))
                .insert(CollidedEntities { collisions: Vec::new() })
                .id();

            bullet_shot_event_writer.send(BulletShotEvent { entity: bullet })
        }
    }
}