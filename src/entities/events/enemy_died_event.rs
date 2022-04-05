use bevy::prelude::Entity;

pub struct EnemyDiedEvent {
    pub enemy_entity: Entity,
}
