use bevy::prelude::*;

use crate::models::player_components::{AimDirection, Player};

pub fn player_control_aim_system(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut AimDirection, With<Player>>,
) {
    for (mut player_aim_direction) in player_query.iter_mut() {
        let mut direction = Vec3::default();

        if input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction != player_aim_direction.direction {
            player_aim_direction.direction = direction;
        }
    }
}