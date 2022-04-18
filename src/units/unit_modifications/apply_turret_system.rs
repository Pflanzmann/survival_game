use bevy::prelude::{Commands, Entity, EventReader, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With, Without};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behavior::spin_aim_behavior::SpinAimBehavior;
use crate::models::behavior::teleport_to_target_behavior::TeleportToTargetBehavior;
use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::turret::{Turret, TurretUnit};
use crate::models::modifications::utils::owner::Owner;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::reload::Reload;
use crate::models::weapon_slot::WeaponSlot;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn apply_turret_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    mut apply_events: EventReader<ApplyModToTargetEvent>,
    mod_query: Query<&Turret, With<Modification>>,
    turret_owner: Query<(Entity, &Transform, &WeaponSlot)>,
    turret_query: Query<&Owner, (With<TurretUnit>, Without<Turret>)>,
) {
    for apply_event in apply_events.iter() {
        let turret_mod = match mod_query.get(apply_event.mod_entity) {
            Ok(turret) => turret,
            Err(_) => continue,
        };

        let (owner_entity, owner_transform, owner_weapon_slot) = match turret_owner.get(apply_event.target_entity) {
            Ok(owner) => owner,
            Err(_) => continue,
        };

        let mut unit_exists = false;
        for owner in turret_query.iter() {
            if owner_entity == owner.entity {
                unit_exists = true;
            }
        }

        if unit_exists {
            continue;
        }

        let pos_vec = get_close_position_2d(*owner_transform, 300.0, 1000.0);

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            texture: texture_handler.turret_unit.clone(),
            transform: Transform::from_xyz(pos_vec[0], pos_vec[1], SpriteLayer::GroundLevel.get_layer_z()),
            ..Default::default()
        })
            .insert(TurretUnit)
            .insert(Owner::new(owner_entity))
            .insert(WeaponSlot { weapon_entity: owner_weapon_slot.weapon_entity })
            .insert(Name::new("Turret"))
            .insert(AimDirection { direction: Vec3::new(1.0, 0.0, 0.0) })
            .insert(SpinAimBehavior)
            .insert(TeleportToTargetBehavior::new(owner_entity, 2500.0, 300.0, 700.0, 0.0))
            .insert(Reload::new(40.0));
    }
}
