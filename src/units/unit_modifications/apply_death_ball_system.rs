use bevy::prelude::{BuildChildren, Commands, Entity, EventReader, GlobalTransform, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behavior::aim_at_closest_target_behavior::AimAtClosestTargetBehavior;
use crate::models::behavior::rotate_behavior::UnitRotation;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::death_ball::{DeathBall, DeathBallUnit};
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::utils::owner::Owner;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_size::UnitSize;
use crate::models::weapon_slot::WeaponSlot;

pub fn apply_death_ball_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&DeathBall, With<Modification>>,
    owner_query: Query<(Entity, &WeaponSlot)>,
    unit_query: Query<&Owner, With<DeathBallUnit>>,
) {
    for apply_event in apply_events.iter() {
        let modification = match mod_query.get(apply_event.mod_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        let (owner_entity, owner_weapon_slot) = match owner_query.get(apply_event.target_entity) {
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

        let base = commands.spawn()
            .insert(Transform::default())
            .insert(UnitRotation { angle: modification.rotation_speed })
            .insert(GlobalTransform::default())
            .id();

        let child = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            texture: texture_handler.death_ball_unit.clone(),
            transform: Transform::from_xyz(desired_pos.x, desired_pos.y, SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        })
            .insert(DeathBallUnit)
            .insert(Owner::new(owner_entity))
            .insert(WeaponSlot { weapon_entity: owner_weapon_slot.weapon_entity })
            .insert(Name::new("DeathBall"))
            .insert(UnitSize { collider_size: Vec2::new(128.0, 128.0) })
            .insert(AimDirection::default())
            .insert(AimAtClosestTargetBehavior)
            .insert(UnitRotation { angle: -modification.rotation_speed })
            .insert(Reload::new(40.0))
            .id();


        commands.entity(owner_entity).add_child(base);
        commands.entity(base).add_child(child);
    }
}
