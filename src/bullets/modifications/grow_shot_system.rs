use bevy::prelude::{Commands, EventReader, Query, With};

use crate::components::bullet_components::Bullet;
use crate::components::event_components::BulletShotEvent;
use crate::components::modification_components::{GrowShot, ModContainer, ModContainerSlot};
use crate::UnitSize;

pub fn grow_shot_system(
    mut bullet_query: Query<(&mut UnitSize, &GrowShot)>,
) {
    for (mut unit_size, grow_shot) in bullet_query.iter_mut() {
        unit_size.collider_size += grow_shot.grow_step;
    }
}