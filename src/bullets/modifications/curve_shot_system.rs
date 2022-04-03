use bevy::app::EventReader;
use bevy::prelude::{Commands, Entity, Query, With};
use rand::random;

use crate::components::bullet_components::Bullet;
use crate::components::event_components::BulletShotEvent;
use crate::components::modification_components::{CurveShot, ModContainer, ModContainerSlot};
use crate::FacingDirection;

pub fn apply_curved_shot_system(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
    bullet_query: Query<&Bullet>,
    source_query: Query<&ModContainerSlot>,
    mod_container_query: Query<&CurveShot, With<ModContainer>>,
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

pub fn curve_shot_system(
    mut bullet_query: Query<(&mut FacingDirection, &CurveShot)>,
) {
    for (mut direction, curve_shot) in bullet_query.iter_mut() {
        let angle: f32 = if curve_shot.curve_left { 0.005 } else { -0.005 };
        let x = direction.facing_direction.x;
        let y = direction.facing_direction.y;

        direction.facing_direction.x = x * angle.cos() - y * angle.sin();
        direction.facing_direction.y = x * angle.sin() + y * angle.cos();

        direction.facing_direction = direction.facing_direction.normalize();
    }
}