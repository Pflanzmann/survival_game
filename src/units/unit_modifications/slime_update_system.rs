use bevy::prelude::{Commands, Entity, Name, Query, Res, Sprite, SpriteBundle, Transform, Vec2, Vec3, With, Without};

use crate::{SpriteLayer, TextureHandles};
use crate::models::aim_direction::AimDirection;
use crate::models::behavior::chase_target_behavior::ChaseTargetBehavior;
use crate::models::behavior::mono_directional_move_behavior::MonoDirectionalMoveBehavior;
use crate::models::behavior::spin_aim_behavior::SpinAimBehavior;
use crate::models::behavior::teleport_to_target_behavior::TeleportToTargetBehavior;
use crate::models::modifications::slime::{Slime, SlimeOwner, SlimeUnit};
use crate::models::modifications::turret::Turret;
use crate::models::move_direction::MoveDirection;
use crate::models::turret_components::{TurretOwner, TurretUnit};
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::weapon_slot::WeaponSlot;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn slime_update_system(
    mut commands: Commands,
    texture_handler: Res<TextureHandles>,
    slime_owner: Query<(Entity, &Transform, &WeaponSlot), With<Slime>>,
    mut slime_query: Query<(Entity, &SlimeOwner, &mut Transform), (With<SlimeUnit>, Without<Slime>)>,
) {
    let mut slime_exists = false;

    for (player_entity, player_transform, weapon_slot) in slime_owner.iter() {
        for (_, owner, _) in slime_query.iter_mut() {
            if player_entity == owner.owner {
                // later count slime -> right now check if turret exists
                slime_exists = true;
            }
        }

        if !slime_exists {
            let pos_vec = get_close_position_2d(*player_transform, 300.0, 1000.0);

            // spawn turret
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                texture: texture_handler.slime_unit.clone(),
                transform: Transform::from_xyz(pos_vec[0], pos_vec[1], SpriteLayer::GroundLevel.get_layer_z()),
                ..Default::default()
            })
                .insert(SlimeUnit)
                .insert(SlimeOwner { owner: player_entity })
                .insert(Name::new("Slime"))
                .insert(ChaseTargetBehavior{target: player_entity, proximity: 200.0})
                .insert(MoveDirection{direction: Vec3::default()})
                .insert(MoveSpeed::new(6.0));
        }
    }
}

