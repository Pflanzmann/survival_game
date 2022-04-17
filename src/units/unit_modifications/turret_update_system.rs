use bevy::prelude::{Commands, Entity, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With, Without};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behaviour::spin_aim_behaviour::SpinAimBehaviour;
use crate::models::behaviour::teleport_to_target_behavior::TeleportToTargetBehavior;
use crate::models::modifications::turret::Turret;
use crate::models::turret_components::{TurretOwner, TurretUint};
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::reload::Reload;
use crate::models::weapon_slot::WeaponSlot;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn turret_update_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    turret_owner: Query<(Entity, &Transform, &WeaponSlot), With<Turret>>,
    mut turret_query: Query<(Entity, &TurretOwner, &mut Transform), (With<TurretUint>, Without<Turret>)>,
) {
    let mut turret_exists = false;

    for (player_entity, player_transform, weapon_slot) in turret_owner.iter() {
        for (_, owner, _) in turret_query.iter_mut() {
            if player_entity == owner.owner {
                // later count turrets -> right now check if turret exists
                turret_exists = true;
            }
        }

        if !turret_exists {
            let pos_vec = get_close_position_2d(*player_transform, 300.0, 1000.0);

            // spawn turret
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    ..Default::default()
                },
                texture: texture_handler.turret_unit.clone(),
                transform: Transform::from_xyz(pos_vec[0], pos_vec[1], SpriteLayer::GroundLevel.get_layer_z()),
                ..Default::default()
            })
                .insert(TurretUint)
                .insert(TurretOwner { owner: player_entity })
                .insert(WeaponSlot { weapon_entity: weapon_slot.weapon_entity })
                .insert(Name::new("Turret"))
                .insert(AimDirection { direction: Vec3::new(1.0, 0.0, 0.0) })
                .insert(SpinAimBehaviour)
                .insert(TeleportToTargetBehavior { target: player_entity, distance: 2500.0, proximity_min: 300.0, proximity_max: 1000.0 , cooldown: 0.0, timer: 0.0})
                .insert(Reload::new(1.0));
        }
    }
}

