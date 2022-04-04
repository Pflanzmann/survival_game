use bevy::math::Vec3;
use bevy::prelude::{AssetServer, Commands, Res, ResMut, Sprite, SpriteBundle, Time, Vec2};
use rand::random;

use crate::{Health, Player, Query, Transform, With};
use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::components::collision_components::Collider;
use crate::components::unit_stats_components::{Damage, Enemy, FacingDirection, MoveSpeed, UnitSize};

#[derive(Default)]
pub struct SpawnTimer(f32);

pub fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    texture_handles: Res<TextureHandles>,
    enemy_handles: Res<EnemyConfigHandles>,
    main_player_query: Query<&Transform, With<Player>>,
) {
    spawn_timer.0 += time.delta().as_secs_f32();
    if spawn_timer.0 < 2.0 {
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
                    custom_size: Some(Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(position_to_spawn.x, position_to_spawn.y, 0.0),
                texture: texture_handles.enemy_goblin.clone(),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(MoveSpeed { move_speed: enemy_handles.goblin.move_speed })
            .insert(UnitSize { collider_size: Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y) })
            .insert(Collider)
            .insert(Damage::new(enemy_handles.goblin.damage))
            .insert(Health::new(enemy_handles.goblin.health))
            .insert(FacingDirection { facing_direction: Vec3::default() });
    }
}