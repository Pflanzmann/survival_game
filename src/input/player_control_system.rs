use bevy::ecs::component::Component;
use bevy::prelude::*;

use crate::input::input_components::{MainCamera, Player};
use crate::ai::ai_components::Speed;

pub fn player_control_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut player_transform, speed) = player_query.get_single_mut().unwrap();
    let mut player_move_transform = player_transform.clone();

    let mut has_input = false;

    if input.pressed(KeyCode::A) {
        has_input = true;
        player_move_transform.translation.x -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        has_input = true;
        player_move_transform.translation.x += 1.0;
    }

    if input.pressed(KeyCode::W) {
        has_input = true;
        player_move_transform.translation.y += 1.0;
    }

    if input.pressed(KeyCode::S) {
        has_input = true;
        player_move_transform.translation.y -= 1.0;
    }

    if !has_input { return; }

    let direction = (player_move_transform.translation - player_transform.translation).normalize();

    player_transform.translation += direction * speed.speed;
}
