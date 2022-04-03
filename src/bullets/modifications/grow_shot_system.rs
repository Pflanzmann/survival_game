use bevy::prelude::{Commands, EventReader, Query, With};

use crate::components::bullet_components::Bullet;
use crate::components::event_components::BulletShotEvent;
use crate::components::modification_components::{GrowShot, ModContainer, ModContainerSlot};
use crate::UnitSize;

pub fn apply_grow_shot_system(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
    bullet_query: Query<&Bullet>,
    source_query: Query<&ModContainerSlot>,
    mod_container_query: Query<&GrowShot, With<ModContainer>>,
) {
    for event in bullet_shot_event.iter() {
        let bullet = match bullet_query.get(event.entity) {
            Ok(bullet) => bullet,
            Err(_) => continue,
        };

        let source_mod_container_slot = match source_query.get(bullet.source_entity) {
            Ok(source) => source,
            Err(_) => continue,
        };

        let modi = match mod_container_query.get(source_mod_container_slot.container_entity) {
            Ok(modi) => modi,
            Err(_) => continue,
        };

        commands.entity(event.entity).insert(modi.clone());
    }
}


pub fn grow_shot_system(
    mut bullet_query: Query<(&mut UnitSize, &GrowShot)>,
) {
    for (mut unit_size, grow_shot) in bullet_query.iter_mut() {
        unit_size.collider_size += grow_shot.grow_step;
    }
}