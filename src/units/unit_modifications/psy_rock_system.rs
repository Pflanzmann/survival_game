use bevy::prelude::{Changed, Query, With};

use crate::models::aim_direction::AimDirection;
use crate::models::modifications::psy_rock::PsyRockUnit;
use crate::models::move_direction::MoveDirection;

pub fn psy_rock_system(
    mut unit_query: Query<(&mut MoveDirection, &AimDirection), (With<PsyRockUnit>, Changed<AimDirection>)>,
) {
    for (mut move_direction, aim_direction) in unit_query.iter_mut() {
        move_direction.direction = aim_direction.direction;
    }
}