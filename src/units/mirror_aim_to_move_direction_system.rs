use bevy::prelude::{Changed, Query, With};

use crate::models::aim_direction::AimDirection;
use crate::models::mirror_aim_to_move_direction::MirrorAimToMoveDirection;
use crate::models::move_direction::MoveDirection;

pub fn mirror_aim_to_move_direction_system(
    mut unit_query: Query<(&mut MoveDirection, &AimDirection), (With<MirrorAimToMoveDirection>, Changed<AimDirection>)>,
) {
    for (mut move_direction, aim_direction) in unit_query.iter_mut() {
        move_direction.direction = aim_direction.direction;
    }
}