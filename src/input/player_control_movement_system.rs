use bevy::prelude::*;

use crate::models::unit_attributes::attribute::*;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::player_components::Player;
use crate::models::unit_stats_components::MoveDirection;

pub fn player_control_movement_system(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &MoveSpeed, &mut MoveDirection), With<Player>>,
) {
    for (mut player_transform, move_speed, mut player_direction) in player_query.iter_mut() {
        let mut player_move_transform = *player_transform;

        if input.pressed(KeyCode::A) {
            player_move_transform.translation.x -= 1.0;
        }

        if input.pressed(KeyCode::D) {
            player_move_transform.translation.x += 1.0;
        }

        if input.pressed(KeyCode::W) {
            player_move_transform.translation.y += 1.0;
        }

        if input.pressed(KeyCode::S) {
            player_move_transform.translation.y -= 1.0;
        }

        let direction = (player_move_transform.translation - player_transform.translation).normalize_or_zero();

        player_transform.translation += direction * move_speed.get_total_amount() * time.delta_seconds() * 60.0;

        if player_direction.direction != direction {
            player_direction.direction = direction;
        }
    }
}
