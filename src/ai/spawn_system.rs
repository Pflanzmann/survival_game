use bevy::math::Vec3;
use bevy::prelude::{AssetServer, Commands, Res, Sprite, SpriteBundle, Time, Vec2};
use rand::random;

use crate::components::unit_stats_components::{Direction, Enemy, Size, Speed, Damage, Health};
use crate::collision::collision_components::Collider;
use crate::Transform;

pub fn spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if time.time_since_startup().as_millis() % 200 != 0 {
        return;
    }

    let random_x = random::<f32>() * (256.0 * 10.0) + 1.0;
    let random_y = random::<f32>() * (256.0 * 10.0) + 1.0;

    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("Rock01.png"),
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Speed { speed: 6.0 })
        .insert(Size { size: Vec2::new(256.0, 256.0) })
        .insert(Collider)
        .insert(Damage{damage: 5.0})
        .insert(Direction {direction: Vec3::default()});

}