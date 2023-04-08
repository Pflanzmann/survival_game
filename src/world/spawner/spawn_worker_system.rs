use bevy::prelude::{Commands, Name, Query, Res, ResMut, SpriteSheetBundle, TextureAtlasSprite, Transform, Vec2, With};
use rand::random;

use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::walking_animation_component::MoveAnimationSide;
use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::behavior::mono_directional_move_behavior::MonoDirectionalMoveBehavior;
use crate::models::behavior::steering_behavior::SteeringBehavior;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::configurations::raw_configs::enemy_behavior::EnemyBehavior;
use crate::models::configurations::spawner_config::SpawnerConfig;
use crate::models::enemy::Enemy;
use crate::models::layerable::Layerable;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;
use crate::models::resources::world::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::models::sprite_flip::SpriteFlip;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::unit_size::UnitSize;

pub fn spawn_worker_system(
    mut commands: Commands,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    enemy_handles: Res<EnemyConfigHandles>,
    spawner_config: Res<SpawnerConfig>,
    player_query: Query<&Transform, With<Player>>,
) {
    for _ in 0..spawner_config.max_spawns_per_frame {
        let spawn_task = match spawn_task_receiver.consume_task() {
            None => break,
            Some(task) => task,
        };

        let enemy = commands.spawn(
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(enemy_handles.enemy_configs[spawn_task.enemy_config_index].size),
                    ..Default::default()
                },
                transform: Transform::from_translation(spawn_task.position.extend(enemy_handles.enemy_configs[spawn_task.enemy_config_index].sprite_layer.get_layer_z())),
                texture_atlas: enemy_handles.enemy_configs[spawn_task.enemy_config_index].texture_atlas.clone(),
                ..Default::default()
            })
            .insert((
                Name::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].entity_name.clone()),
                Enemy,
                UnitSize::new_size(enemy_handles.enemy_configs[spawn_task.enemy_config_index].size),
                SolidBodyCollider { offset: Vec2::new(0.0, 0.0), collider_type: ColliderType::Circle(0.0) },
                HitBoxCollider { collider_type: ColliderType::Circle(0.0) },
                ColliderWeight { weight: enemy_handles.enemy_configs[spawn_task.enemy_config_index].collider_weight },
                SteeringBehavior::default()
            ))

            .insert((
                DamageBundle::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].base_damage, enemy_handles.enemy_configs[spawn_task.enemy_config_index].damage_interval),
                MoveSpeed::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].move_speed),
                MoveDirection::default(),
                Health::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].health),
            ))

            .insert((
                Layerable::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].sprite_layer.get_layer_z()),
                SpriteFlip,
                MoveAnimationSide::new(random::<f32>() * 10.0, 3, 2, 15),
                CurrentAnimationState::new()
            )).id();

        match enemy_handles.enemy_configs[spawn_task.enemy_config_index].behavior {
            EnemyBehavior::ChasePlayer => {
                commands.entity(enemy).insert(ChaseTargetBehavior { target: spawn_task.target_entity, proximity: 0.0 });
            }

            EnemyBehavior::SidedMovement { horizontal } => {
                let player_transform = match player_query.get(spawn_task.target_entity) {
                    Ok(value) => value,
                    Err(_) => continue,
                };

                let player_direction = player_transform.translation.truncate() - spawn_task.position;

                let direction_to_walk = if horizontal {
                    if player_direction.x < 0.0 {
                        Vec2::new(-1.0, 0.0)
                    } else {
                        Vec2::new(1.0, 0.0)
                    }
                } else if player_direction.y < 0.0 {
                    Vec2::new(0.0, -1.0)
                } else {
                    Vec2::new(0.0, 1.0)
                };

                commands.entity(enemy).insert(MonoDirectionalMoveBehavior { direction: direction_to_walk });
            }
            EnemyBehavior::DirectionalMovement => {
                let player_transform = match player_query.get(spawn_task.target_entity) {
                    Ok(value) => value,
                    Err(_) => continue,
                };

                let player_direction = (player_transform.translation.truncate() - spawn_task.position).normalize_or_zero();

                commands.entity(enemy).insert(MonoDirectionalMoveBehavior { direction: player_direction });
            }
        };
    }
}
