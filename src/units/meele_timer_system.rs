use bevy::prelude::{Query, Res, Time};

use crate::models::unit_attributes::meele_attack_speed::MeeleAttackSpeed;

pub fn meele_timer_system(
    time: Res<Time>,
    mut timer_query: Query<&mut MeeleAttackSpeed>,
) {
    for mut reloadable in timer_query.iter_mut() {
        if reloadable.reload_timer <= 0.0 {
            continue;
        }
        
        reloadable.reload_timer -= time.delta().as_secs_f32();
    }
}