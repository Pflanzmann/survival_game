use bevy::prelude::{Commands, Component, EventReader, Query, With};

use crate::models::bullet::Bullet;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::mod_container::ModContainer;
use crate::models::mod_container_slot::ModContainerSlot;

/// A generic system to apply [Bullet]-mods like [SplitShot] to the new shot bullet from the
/// [BulletShotEvent].
///
/// The mods from get applied from the [ModContainer] associated to the weapon.
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(apply_modification_system::<SplitShot>)
///     }
/// }
/// ```
///
pub fn assign_modification_to_bullet_system<T: Component + Copy>(
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
