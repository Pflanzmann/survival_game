use bevy::prelude::{Res, ResMut, Time};

use crate::models::resources::world::game_time::GameTime;

pub fn game_time_system(
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
) {
    game_time.time_in_seconds += time.delta_seconds_f64();
}