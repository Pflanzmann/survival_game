use bevy::prelude::*;

use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;

pub fn player_control_movement_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut MoveDirection), With<Player>>,
) {
    for mut player_direction in player_query.iter_mut() {
        let mut direction = Vec3::default();

        if input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        if player_direction.direction != direction {
            player_direction.direction = direction;
        }
    }
}
