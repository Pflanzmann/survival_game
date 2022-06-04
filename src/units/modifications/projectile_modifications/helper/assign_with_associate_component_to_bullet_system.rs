use bevy::prelude::{Commands, Component, EventReader, Query, With, Without};

use crate::models::bullet::Bullet;
use crate::models::child_projectile::ChildProjectile;
use crate::models::events::bullet_shot_event::BulletShotEvent;
use crate::models::mod_container::ModContainer;
use crate::models::mod_container_slot::ModContainerSlot;
use crate::models::modifications::utils::associate_component::AssociateComponent;

/// A generic system to apply [Bullet]-mods like [KnockBackShot] with the associated [Component]
/// like [KnockBack] to the new shot bullet from the [BulletShotEvent].
///
/// The mods from get applied from the [ModContainer] associated to the weapon.
///
/// ```
/// # use bevy_ecs::prelude::;
/// #
/// impl Plugin for ExamplePlugin {
///     fn build(&self, app: &mut App) {
///         app.add_system(assign_with_associate_component_to_bullet_system::<KnockBackShot, KnockBack>)
///     }
/// }
/// ```
///
pub fn assign_with_associate_component_to_bullet_system<
    U: Component + Clone + AssociateComponent<T>,
    T: Component,
>(
    mut commands: Commands,
    mut bullet_shot_event: EventReader<BulletShotEvent>,
    bullet_query: Query<&Bullet, Without<ChildProjectile>>,
    source_query: Query<&ModContainerSlot>,
    mod_container_query: Query<&U, With<ModContainer>>,
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

        let modification = match mod_container_query.get(source_mod_container_slot.container_entity) {
            Ok(modification) => modification,
            Err(_) => continue,
        };

        commands.entity(event.entity).insert(modification.clone());
        commands.entity(event.entity).insert(modification.get_component());
    }
}
