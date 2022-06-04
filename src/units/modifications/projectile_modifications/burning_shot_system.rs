use bevy::prelude::{Commands, EventReader, Query};

use crate::models::events::damaged_event::DamagedEvent;
use crate::models::modifications::burning_shot::BurningShot;
use crate::models::modifications::statuses::burning::Burning;
use crate::models::projectile::Projectile;

pub fn burning_shot_system(
    mut command: Commands,
    mut damaged_events: EventReader<DamagedEvent>,
    projectile_query: Query<(&Projectile, &BurningShot)>,
    mut already_effected_target_query: Query<&mut Burning>,
) {
    for event in damaged_events.iter() {
        let (projectile, burning_shot) = match projectile_query.get(event.source_entity) {
            Ok(projectile) => projectile,
            Err(_) => continue,
        };

        if let Ok(mut burning) = already_effected_target_query.get_mut(event.target_entity) {
            if burning.source_entity == projectile.source_entity {
                burning.debuff_duration = burning.debuff_duration.max(burning_shot.debuff_duration);
                continue;
            }
        };

        command.entity(event.target_entity).insert(Burning::new(
            projectile.source_entity,
            burning_shot.debuff_duration,
            burning_shot.damage_interval,
            burning_shot.damage_per_tick,
        ));
    }
}