use bevy::prelude::Entity;

pub struct PlayerEnemyCollisionEvent {
    pub player_entity: Entity,
    pub enemy_entity: Entity,
}
