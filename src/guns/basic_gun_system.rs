use bevy::prelude::{AssetServer, Color, Commands, Res, Sprite, SpriteBundle, Time, Vec2};

use crate::{Query, Transform, With};
use crate::components::unit_stats_components::{Direction, ColliderSize, Speed};
use crate::components::bullet_components::Bullet;
use crate::collision::collision_components::Collider;
use crate::components::gun_components::BasicGun;

pub fn basic_gun_system(
    mut command: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player_query: Query<(&Transform, &Direction), With<BasicGun>>,
) {
    if time.time_since_startup().as_millis() % 10000000 != 0 {
        return;
    }

    for (transform, direction) in player_query.iter() {
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
            .insert(ColliderSize { collider_size: Vec2::new(256.0, 256.0) })
            .insert(Direction { direction: direction.direction })
            .insert(Speed { speed: 15.0 });
    }
}