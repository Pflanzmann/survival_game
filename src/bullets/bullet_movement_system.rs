use bevy::core::Time;
use bevy::prelude::{Commands, EventWriter, Transform};

use crate::{Entity, Query, Res, With};
use crate::models::bullet_components::{Bullet, BulletRange};
use crate::models::events::bullet_stopped_event::BulletStoppedEvent;
use crate::models::unit_stats_components::{FacingDirection, MoveSpeed};

pub fn bullet_movement_system(
    time : Res<Time>,
    mut bullet_stopped_event: EventWriter<BulletStoppedEvent>,
    mut bullet_query: Query<(&mut Transform, &FacingDirection, &MoveSpeed, &mut BulletRange, Entity), With<Bullet>>,
) {
    for (mut transform, direction, speed, mut bullet_range, bullet_entity) in bullet_query.iter_mut() {
        let distance_to_move = speed.move_speed;
        bullet_range.distance_traveled += distance_to_move;

        if bullet_range.distance_traveled >= bullet_range.total_range {
            bullet_stopped_event.send(BulletStoppedEvent { bullet_entity });
            continue;
        }

        transform.translation += direction.facing_direction * distance_to_move * time.delta_seconds() * 60.0;
    }
}