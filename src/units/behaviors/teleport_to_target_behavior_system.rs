use bevy::prelude::{Commands, Entity, Query, Transform, With, Without};
use crate::models::behavior::teleporting_script::TeleportingScript;

use crate::models::behavior::teleport_to_target_behavior::TeleportToTargetBehavior;
use crate::models::player::Player;
use crate::util::get_close_position_2d::get_close_position_2d;

pub fn teleport_to_target_behavior_system(
    mut commands: Commands,

    mut unit_query: Query<(Entity, &TeleportToTargetBehavior, &mut Transform), (Without<Player>, Without<TeleportingScript>)>,
    target_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (player_entity, player_transform) in target_query.iter() {
        for (entity, target_behavior, mut turret_transform) in unit_query.iter_mut() {
            if player_entity == target_behavior.target && player_transform.translation.distance(turret_transform.translation) > target_behavior.distance {
                commands.entity(entity).insert(TeleportingScript::new(player_transform.translation));
            }
        }
    }
}
