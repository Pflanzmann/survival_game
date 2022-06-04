use bevy::prelude::Entity;

pub struct ProjectileStoppedEvent {
    pub projectile_entity: Entity,
}
