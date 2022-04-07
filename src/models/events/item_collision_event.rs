use bevy::prelude::Entity;

pub struct ItemCollisionEvent {
    pub player_entity : Entity,
    pub item_entity: Entity,
}

