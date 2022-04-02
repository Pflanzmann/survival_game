use bevy::prelude::{Commands, EventReader, Query};

use crate::components::event_components::BulletShotEvent;
use crate::components::modification_components::GrowShot;
use crate::UnitSize;

pub fn apply_grow_shot_system(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
) {
    for event in bullet_shot_event.iter() {
        commands.entity(event.entity).insert(GrowShot { grow_step: 10.0 });
    }
}


pub fn grow_shot_system(
    mut bullet_query: Query<(&mut UnitSize, &GrowShot)>,
) {
    for (mut unit_size, grow_shot) in bullet_query.iter_mut() {
        unit_size.collider_size += grow_shot.grow_step;
    }
}