use bevy::prelude::{BuildChildren, Commands, Entity, EventReader, GlobalTransform, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};

use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::bundles::damage_bundle::DamageBundle;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::enemy_hit_box_collider::EnemyHitBoxCollider;
use crate::models::collision::hit_box_collider::HitBoxCollider;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::knock_back::KnockBack;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::shield::{Shield, ShieldUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::move_direction::MoveDirection;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;
use crate::TextureHandles;

pub fn apply_shield_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&Shield, With<Modification>>,
    owner_query: Query<(Entity, &WeaponSlot)>,
    unit_query: Query<&Owner, With<ShieldUnit>>,
) {
    for apply_event in apply_events.iter() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let (owner_entity, _owner_weapon_slot) = match owner_query.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let mut unit_exists = false;
        for owner in unit_query.iter() {
            if owner_entity == owner.entity {
                unit_exists = true;
            }
        }

        if unit_exists {
            continue;
        }

        let desired_pos = Vec3::new(modification.rotation_distance, 0.0, 0.0);

        let base = commands.spawn_empty()
            .insert(Transform::default())
            .insert(UnitRotation { revolutions_per_min: modification.revolutions_per_min })
            .insert(GlobalTransform::default())
            .id();

        let child = commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            texture: texture_handler.shield.clone(),
            transform: Transform::from_translation(desired_pos),
            ..Default::default()
        })
            .insert(Name::new("Shield"))
            .insert(ShieldUnit)
            .insert(Owner::new(owner_entity))

            .insert(DamageBundle::new(modification.damage, modification.damage_interval))

            .insert(UnitSize::new_size(modification.unit_size))
            .insert(HitBoxCollider { collider_type: ColliderType::Rectangle(Vec2::new(0.0, 0.0)) })
            .insert(EnemyHitBoxCollider)

            .insert(UnitRotation { revolutions_per_min: -modification.revolutions_per_min })

            .insert(KnockBack { push_duration: modification.push_duration, push_force: modification.push_force })
            .insert(MoveDirection { direction: Vec2::new(1.0, 0.0) })
            .insert(ChaseTargetBehavior { target: owner_entity, proximity: 0.0 })
            .id();

        commands.entity(owner_entity).add_child(base);
        commands.entity(base).add_child(child);
    }
}
