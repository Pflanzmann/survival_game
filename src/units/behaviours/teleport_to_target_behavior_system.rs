use bevy::prelude::{Entity, Query, Transform, With, Without};

use crate::models::behaviour::teleport_to_target_behavior::{TeleportTargetPointer, TeleportToTargetBehavior};
use crate::models::player::Player;
use crate::models::turret_components::TurretOwner;
use crate::util::get_close_position_2D::get_close_position_2D;

pub fn teleport_to_target_behavior_system(
    mut unit_query: Query<(&TeleportTargetPointer, &mut Transform), (With<TeleportToTargetBehavior>, Without<Player>)>,
    target_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (player_entity, player_transform) in target_query.iter() {
        for (owner, mut turret_transform) in unit_query.iter_mut() {
            if player_entity == owner.target && player_transform.translation.distance(turret_transform.translation) > 2000.0 {
                let pos_vec = get_close_position_2D(*player_transform);

                turret_transform.translation.x = pos_vec[0];
                turret_transform.translation.y = pos_vec[1];
            }
        }
    }
}