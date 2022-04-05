use bevy::prelude::Entity;

pub struct BulletEnemyCollisionEvent {
    pub enemy_entity: Entity,
    pub bullet_entity: Entity,
}