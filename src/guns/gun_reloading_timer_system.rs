use bevy::prelude::{Query, Time};

use crate::models::gun_components::Reloadable;
use crate::Res;

pub fn gun_reloading_timer_system(
    time: Res<Time>,
    mut reloadable_query: Query<&mut Reloadable>,
) {
    for mut reloadable in reloadable_query.iter_mut() {
        if reloadable.reload_timer <= 0.0 {
            continue;
        }
        reloadable.reload_timer -= time.delta().as_secs_f32();
    }
}