use bevy::ecs::component::Component;
use bevy::prelude::*;

use crate::input::input_components::{MainCamera, Player};

pub fn player_control_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_query.get_single_mut().unwrap();

    if input.pressed(KeyCode::A) {
        player_transform.translation.x -= 10.0;
    }

    if input.pressed(KeyCode::D) {
        player_transform.translation.x += 10.0;
    }

    if input.pressed(KeyCode::W) {
        player_transform.translation.y += 10.0;
    }

    if input.pressed(KeyCode::S) {
        player_transform.translation.y -= 10.0;
    }
}
