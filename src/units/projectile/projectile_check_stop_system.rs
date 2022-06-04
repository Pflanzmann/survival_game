use bevy::prelude::{Entity, EventWriter, Query, With};

use crate::models::projectile::Projectile;
use crate::models::events::projectile_stopped_event::ProjectileStoppedEvent;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::travel_range::TravelRange;

/// This system checks the [TravelRange] of each [Projectile] and checks if the projectile should despawn.
pub fn projectile_check_stop_system(
    mut projectile_stopped_event: EventWriter<ProjectileStoppedEvent>,
    mut projectile_query: Query<(Entity, &TravelRange), With<Projectile>>,
) {
    for (projectile_entity, projectile_range) in projectile_query.iter_mut() {
        if projectile_range.distance_traveled >= projectile_range.get_total_amount() {
            projectile_stopped_event.send(ProjectileStoppedEvent { projectile_entity });
        }
    }
}
