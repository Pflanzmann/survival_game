use bevy::math::Vec3;
use bevy::prelude::{Commands, Name, Query, Res, ResMut, Sprite, SpriteBundle, Time, Transform, Vec2, With};
use rand::random;

use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::bundles::enemy_bundle::EnemyBundle;
use crate::models::collider::collider::Collider;
use crate::models::enemy::Enemy;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_size::UnitSize;

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

    let direction_to_spawn = Vec3::new(random_x, random_y, 0.0).normalize_or_zero();

    for transform in main_player_query.iter() {
        let position_to_spawn = transform.translation + (direction_to_spawn * (256.0 * 12.0));

        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(position_to_spawn.x, position_to_spawn.y, SpriteLayer::GroundLevel.get_layer_z()),
                texture: texture_handles.enemy_goblin.clone(),
                ..Default::default()
            })
            .insert_bundle(EnemyBundle {
                enemy: Enemy,
                collider: Collider,
                unit_size: UnitSize { collider_size: Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y) },
                facing_direction: MoveDirection { direction: Vec3::default() },
                move_speed: MoveSpeed::new(enemy_handles.goblin.move_speed),
                damage: Damage::new(enemy_handles.goblin.damage),
                health: Health::new(enemy_handles.goblin.health),
            }).insert(Name::new("Goblin"));
    }
}