use bevy::prelude::{Commands, Name, Res, ResMut, SpriteSheetBundle, TextureAtlasSprite, Transform, Vec2};

use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::walking_animation_component::MoveAnimationSide;
use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::behavior::steering_behavior::SteeringBehavior;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::enemy::Enemy;
use crate::models::layerable::Layerable;
use crate::models::move_direction::MoveDirection;
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
) {
    for _ in 0..50 {
        let spawn_task = match spawn_task_receiver.consume_task() {
            None => break,
            Some(task) => task,
        };

        commands.spawn_bundle(
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(enemy_handles.enemy_configs[spawn_task.enemy_config_index].size),
                    ..Default::default()
                },
                transform: Transform::from_translation(spawn_task.position.extend(enemy_handles.enemy_configs[spawn_task.enemy_config_index].sprite_layer.get_layer_z())),
                texture_atlas: enemy_handles.enemy_configs[spawn_task.enemy_config_index].texture_atlas.clone(),
                ..Default::default()
            })
            .insert(Name::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].entity_name.clone()))
            .insert(Enemy)

            .insert(UnitSize::new_size(enemy_handles.enemy_configs[spawn_task.enemy_config_index].size))
            .insert(SolidBodyCollider { offset: Vec2::new(0.0, 0.0), collider_type: ColliderType::Circle(0.0) })
            .insert(HitBoxCollider { collider_type: ColliderType::Circle(0.0) })
            .insert(ColliderWeight { weight: enemy_handles.enemy_configs[spawn_task.enemy_config_index].collider_weight })

            .insert_bundle(DamageBundle::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].base_damage, enemy_handles.enemy_configs[spawn_task.enemy_config_index].damage_interval))

            .insert(MoveSpeed::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].move_speed))
            .insert(MoveDirection::default())

            .insert(Layerable::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].sprite_layer.get_layer_z()))
            .insert(SpriteFlip)

            .insert(Health::new(enemy_handles.enemy_configs[spawn_task.enemy_config_index].health))

            .insert(SteeringBehavior::default())
            .insert(ChaseTargetBehavior { target: spawn_task.target_player, proximity: 0.0 })

            .insert(MoveAnimationSide::new(0.0, 3, 2, 15))
            .insert(CurrentAnimationState::new());
    }
}
