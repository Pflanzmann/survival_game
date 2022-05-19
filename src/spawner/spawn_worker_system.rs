use bevy::prelude::{Commands, Entity, Name, Query, Res, ResMut, SpriteBundle, SpriteSheetBundle, TextureAtlasSprite, Transform, Vec2, With};

use crate::SpriteLayer;
use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;
use crate::models::animation::animation_state::CurrentAnimationState;
use crate::models::animation::walking_animation_component::{MoveAnimationSide, MoveAnimationUp};
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
use crate::models::player::Player;
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::sprite_flip::SpriteFlip;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::unit_size::UnitSize;

pub fn spawn_worker_system(
    mut commands: Commands,
    mut spawn_task_receiver: ResMut<SpawnTaskReceiver>,
    enemy_handles: Res<EnemyConfigHandles>,
    player_query: Query<Entity, With<Player>>,
    atlas_handles: ResMut<AtlasHandles>,
) {
    for player_entity in player_query.iter() {
        for _ in 0..50 {
            let spawn_task = match spawn_task_receiver.consume_task() {
                None => break,
                Some(task) => task,
            };

            commands.spawn_bundle(
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(spawn_task.get_position()),
                    texture_atlas: atlas_handles.goblin_atlas.clone(),
                    ..Default::default()
                })
                .insert(Name::new("Goblin"))
                .insert(Enemy)

                .insert(UnitSize::new_size(Vec2::new(enemy_handles.goblin.sprite_custom_size_x, enemy_handles.goblin.sprite_custom_size_y)))
                .insert(SolidBodyCollider { offset: Vec2::new(0.0, -80.0), collider_type: ColliderType::Circle(enemy_handles.goblin.sprite_custom_size_x / 4.0) })
                .insert(HitBoxCollider { collider_type: ColliderType::Circle(enemy_handles.goblin.sprite_custom_size_x / 2.0) })
                .insert(ColliderWeight { weight: 0.2 })

                .insert_bundle(DamageBundle::new(enemy_handles.goblin.damage, 60.0))

                .insert(MoveSpeed::new(enemy_handles.goblin.move_speed))
                .insert(MoveDirection::default())

                .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))
                .insert(SpriteFlip)

                .insert(Health::new(enemy_handles.goblin.health))

                .insert(SteeringBehavior::default())
                .insert(ChaseTargetBehavior { target: player_entity, proximity: 0.0 })

                .insert(MoveAnimationSide::new(0.0, 3, 2, 15))
                .insert(CurrentAnimationState::new());
        }
    }
}
