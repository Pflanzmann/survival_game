use bevy::prelude::{Entity, EventWriter, Query, With};

use crate::models::bullet::Bullet;
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::travel_range::TravelRange;

/// This system checks the [TravelRange] of each [Bullet] and checks if the bullet should despawn.
pub fn bullet_check_stop_system(
    mut bullet_stopped_event: EventWriter<BulletStoppedEvent>,
    mut bullet_query: Query<(Entity, &TravelRange), With<Bullet>>,
) {
    for (bullet_entity, bullet_range) in bullet_query.iter_mut() {
        if bullet_range.distance_traveled >= bullet_range.get_total_amount() {
            bullet_stopped_event.send(BulletStoppedEvent { bullet_entity });
        }
    }
}
