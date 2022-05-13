use bevy::prelude::{Entity, Query, Transform, With, Without};

use crate::models::behavior::teleport_to_target_behavior::{TeleportToTargetBehavior};
use crate::models::player::Player;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn teleport_to_target_behavior_system(
    mut unit_query: Query<(&TeleportToTargetBehavior, &mut Transform), Without<Player>>,
    target_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (player_entity, player_transform) in target_query.iter() {
        for (target_behavior, mut turret_transform) in unit_query.iter_mut() {
            if player_entity == target_behavior.target && player_transform.translation.truncate().distance(turret_transform.translation.truncate()) > target_behavior.distance {
                let pos_vec = get_close_position_2d(*player_transform, target_behavior.proximity_min, target_behavior.proximity_max);

                turret_transform.translation.x = pos_vec[0];
                turret_transform.translation.y = pos_vec[1];
            }
        }
    }
}