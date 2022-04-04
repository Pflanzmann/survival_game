use bevy::prelude::{Entity, Transform};

use crate::Vec3;

pub struct BulletShotEvent {
    pub entity: Entity,
}

pub struct EnemyDiedEvent {
    pub enemy_entity: Entity,
}

pub struct BulletEnemyCollisionEvent {
    pub enemy_entity: Entity,
    pub bullet_entity: Entity,
}

pub struct BulletStoppedEvent {
    pub bullet_entity: Entity,
}

pub struct ItemPickupEvent {
    //pub item_type : String or Component?
}

pub struct EnemyHitEvent {
    pub bullet_entity: Entity,
}