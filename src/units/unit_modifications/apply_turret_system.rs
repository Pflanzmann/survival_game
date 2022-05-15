use bevy::prelude::{Commands, Entity, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, With};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behavior::spin_aim_behavior::SpinAimBehavior;
use crate::models::behavior::teleport_to_target_behavior::TeleportToTargetBehavior;
use crate::models::collision::collider_type::ColliderType;
use crate::models::collision::collider_weight::ColliderWeight;
use crate::models::collision::solid_body_collider::SolidBodyCollider;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::layerable::Layerable;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::turret::{Turret, TurretUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn apply_turret_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&Turret, With<Modification>>,
    owner_query: Query<(Entity, &Transform, &WeaponSlot)>,
    unit_query: Query<&Owner, With<TurretUnit>>,
) {
    for apply_event in apply_events.iter() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let (owner_entity, owner_transform, owner_weapon_slot) = match owner_query.get(apply_event.target_entity) {
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

        let pos_vec = get_close_position_2d(owner_transform.translation.x, owner_transform.translation.y, 300.0, 1000.0);

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            texture: texture_handler.turret_unit.clone(),
            transform: Transform::from_xyz(pos_vec[0], pos_vec[1], 0.0),
            ..Default::default()
        })
            .insert(Name::new("Turret"))
            .insert(TurretUnit)
            .insert(Owner::new(owner_entity))
            .insert(WeaponSlot { weapon_entity: owner_weapon_slot.weapon_entity })

            .insert(UnitSize { unit_size: modification.unit_size })
            .insert(SolidBodyCollider { offset: Vec2::new(0.0, 80.0), collider_type: ColliderType::Circle(128.0 / 3.0) })
            .insert(ColliderWeight { weight: 1.0 })

            .insert(Layerable::new(SpriteLayer::GroundLevel.get_layer_z()))

            .insert(AimDirection { direction: Vec2::new(1.0, 0.0) })
            .insert(Reload::new(modification.reload))
            .insert(SpinAimBehavior)
            .insert(TeleportToTargetBehavior::new(owner_entity, modification.teleport_distance, modification.teleport_proximity_min, modification.teleport_proximity_max, modification.teleport_cooldown))
        ;
    }
}
