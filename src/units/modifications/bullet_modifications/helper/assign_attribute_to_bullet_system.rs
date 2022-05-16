use bevy::prelude::{Commands, Component, EventReader, Query, With};

use crate::models::attribute_container::AttributeContainer;
use crate::models::attribute_container_slot::AttributeContainerSlot;
use crate::models::bullet::Bullet;
use crate::models::events::bullet_shot_event::BulletShotEvent;

/// A generic system to apply [Attriubutes] like [TravelTime] to the new shot bullet from the
/// [BulletShotEvent].
///
/// The attribute gets applied from the [AttributeContainer] associated to the weapon.
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(assign_attribute_to_bullet_system::<SplitShot>)
///     }
/// }
/// ```
///
pub fn assign_attribute_to_bullet_system<T: Component + Copy>(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
    bullet_query: Query<&Bullet, With<T>>,
    source_query: Query<&AttributeContainerSlot>,
    attribute_container_query: Query<&T, With<AttributeContainer>>,
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

        let modi = match attribute_container_query.get(source_mod_container_slot.container_entity) {
            Ok(modi) => modi,
            Err(_) => continue,
        };

        commands.entity(event.entity).insert(modi.clone());
    }
}