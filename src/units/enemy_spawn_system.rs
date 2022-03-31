use bevy::math::Vec3;
use bevy::prelude::{AssetServer, Commands, Res, ResMut, Sprite, SpriteBundle, Time, Vec2};
use rand::random;

use crate::{Player, Query, Transform, With};
use crate::components::collision_components::Collider;
use crate::components::unit_stats_components::{Damage, Enemy, FacingDirection, MoveSpeed, UnitSize};

#[derive(Default)]
pub struct SpawnTimer(f32);

pub fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
    main_player_query: Query<&Transform, With<Player>>,
) {
    spawn_timer.0 += time.delta().as_secs_f32();
    if spawn_timer.0 < 0.5 {
        return;
    }

    spawn_timer.0 = 0.0;

    let random_x = random::<f32>() * 2.0 - 1.0;
    let random_y = random::<f32>() * 2.0 - 1.0;

    let direction_to_spawn = Vec3::new(random_x, random_y, 0.0).normalize();

    for transform in main_player_query.iter() {
        let position_to_spawn = transform.translation + (direction_to_spawn * (256.0 * 12.0));

        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(position_to_spawn.x, position_to_spawn.y, 0.0),
                texture: asset_server.load("Rock01.png"),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(MoveSpeed { move_speed: 6.0 })
            .insert(UnitSize { collider_size: Vec2::new(256.0, 256.0) })
            .insert(Collider)
            .insert(Damage { damage: 5.0 })
            .insert(FacingDirection { facing_direction: Vec3::default() });
    }
}