use bevy::prelude::{Query, Res, Time};

use crate::models::unit_attributes::reload::Reload;

pub fn gun_reloading_timer_system(
    time: Res<Time>,
    mut reloadable_query: Query<&mut Reload>,
) {
    for mut reloadable in reloadable_query.iter_mut() {
        if reloadable.reload_timer <= 0.0 {
            continue;
        }
        reloadable.reload_timer -= time.delta().as_secs_f32();
    }
}