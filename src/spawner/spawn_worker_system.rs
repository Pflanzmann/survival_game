use bevy::prelude::{Commands, Entity, Name, Query, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};

use crate::{SpriteLayer, TextureHandles};
use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;
use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::behavior::steering_behavior::SteeringBehavior;
use crate::models::bundles::enemy_bundle::EnemyBundle;
use crate::models::collider::collider::Collider;
use crate::models::collider::collider_type::ColliderType;
use crate::models::collider::collision_weight::CollisionWeight;
use crate::models::enemy::Enemy;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::sprite_flip::SpriteFlip;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_size::UnitSize;

pub fn spawn_worker_system(
    mut commands: Commands,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    texture_handles: Res<TextureHandles>,
    enemy_handles: Res<EnemyConfigHandles>,
    player_query: Query<Entity, With<Player>>,
) {
    for player_entity in player_query.iter() {
        for _ in 0..50 {
            let spawn_task = match spawn_task_receiver.consume_task() {
                None => break,
                Some(task) => task,
            };

            commands.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(spawn_task.get_position().x, spawn_task.get_position().y, SpriteLayer::GroundLevel.get_layer_z()),
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
                }).insert(Name::new("Goblin"))
                .insert(SpriteFlip)
                .insert(SteeringBehavior::default())
                .insert(CollisionWeight { weight: 0.2 })
                .insert(ColliderType::Circle(enemy_handles.goblin.sprite_custom_size_x / 2.0))
                .insert(ChaseTargetBehavior { target: player_entity, proximity: 0.0 })
            ;
        }
    }
}