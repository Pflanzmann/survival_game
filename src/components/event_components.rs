use bevy::prelude::Entity;
use crate::Vec3;

pub struct BulletShotEvent {
    pub entity: Entity,
}

pub struct EnemyDiedEvent {
    pub death_position: Vec3,
}