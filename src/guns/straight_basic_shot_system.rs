use bevy::prelude::{AssetServer, Commands, Res, Sprite, SpriteBundle, Time, Vec2};

use crate::{Entity, Query, Transform, With};
use crate::collision::collision_components::Collider;
use crate::components::bullet_components::Bullet;
use crate::components::gun_components::{Reloadable, StraightBasicShot, WeaponSlot};
use crate::components::unit_stats_components::{Direction, MoveSpeed, UnitSize};

pub fn straight_basic_shot_system(
    mut command: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player_query: Query<(&Transform, &Direction, &WeaponSlot)>,
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

            command.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                texture: asset_server.load("Bullet.png"),
                ..Default::default()
            })
                .insert(Bullet)
                .insert(Collider)
                .insert(UnitSize { collider_size: Vec2::new(256.0, 256.0) })
                .insert(Direction { direction: direction.direction })
                .insert(MoveSpeed { move_speed: 15.0 });
        }
    }
}