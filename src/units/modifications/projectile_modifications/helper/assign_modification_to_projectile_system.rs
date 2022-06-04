use bevy::prelude::{Commands, Component, EventReader, Query, With, Without};

use crate::models::projectile::Projectile;
use crate::models::child_projectile::ChildProjectile;
use crate::models::events::projectile_shot_event::ProjectileShotEvent;
use crate::models::mod_container::ModContainer;
use crate::models::mod_container_slot::ModContainerSlot;

/// A generic system to apply [Projectile]-mods like [SplitShot] to the new shot projectile from the
/// [ProjectileShotEvent].
///
/// The mods from get applied from the [ModContainer] associated to the weapon.
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(assign_modification_to_projectile_system::<SplitShot>)
///     }
/// }
/// ```
///
pub fn assign_modification_to_projectile_system<T: Component + Clone>(
    mut commands: Commands,
    mut projectile_shot_event: EventReader<ProjectileShotEvent>,
    projectile_query: Query<&Projectile, Without<ChildProjectile>>,
    source_query: Query<&ModContainerSlot>,
    mod_container_query: Query<&T, With<ModContainer>>,
) {
    for event in projectile_shot_event.iter() {
        let projectile = match projectile_query.get(event.entity) {
            Ok(projectile) => projectile,
            Err(_) => continue,
        };

        let source_mod_container_slot = match source_query.get(projectile.source_entity) {
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
