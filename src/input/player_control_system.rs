use bevy::prelude::*;

use crate::components::player_components::Player;
use crate::components::unit_stats_components::{Direction, MoveSpeed};

pub fn player_control_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &MoveSpeed, &mut Direction), With<Player>>,
) {
    for (mut player_transform, move_speed, mut player_direction) in player_query.iter_mut() {
        let mut player_move_transform = player_transform.clone();

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

        if player_move_transform.translation == player_transform.translation { return; }

        let direction = (player_move_transform.translation - player_transform.translation).normalize();

        player_transform.translation += direction * move_speed.move_speed;
        player_direction.direction = direction;
    }
}
