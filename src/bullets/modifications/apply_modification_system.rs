use bevy::prelude::{Commands, Component, EventReader, Query, With};

use crate::models::bullet_components::Bullet;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::modification_components::{ModContainer, ModContainerSlot};

pub fn apply_modification_system<T: Component + Copy>(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
    bullet_query: Query<&Bullet>,
    source_query: Query<&ModContainerSlot>,
    mod_container_query: Query<&T, With<ModContainer>>,
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
