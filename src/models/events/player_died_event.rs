use bevy::prelude::Entity;

pub struct PlayerDiedEvent {
    pub player_entity: Entity,
}
