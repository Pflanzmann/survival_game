use bevy::prelude::*;

use crate::AppState;
use crate::models::gold_storage::GoldStorage;
use crate::models::player::Player;

pub fn level_up_system(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut gold_storage_query: Query<&mut GoldStorage, With<Player>>,
) {
    for mut gold_storage in gold_storage_query.iter_mut() {
        if gold_storage.number < 10 {
            return;
        }

        gold_storage.number -= 10;

        next_app_state.set(AppState::Shop);
    }
}
