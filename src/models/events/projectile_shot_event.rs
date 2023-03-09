use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct ProjectileShotEvent {
    pub entity: Entity,
}
