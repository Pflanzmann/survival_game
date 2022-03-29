use bevy::prelude::*;

use crate::components::unit_stats_components::{Direction, Speed};
use crate::components::player_components::Player;

pub fn player_control_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed, &mut Direction), With<Player>>,
) {
    let (mut player_transform, speed, mut player_direction) = player_query.get_single_mut().unwrap();
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

    player_transform.translation += direction * speed.speed;
    player_direction.direction = direction;
}
