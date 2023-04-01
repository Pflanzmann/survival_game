use bevy::prelude::{Vec2, Resource};

#[derive(Resource)]
pub struct BackgroundTilesResource {
    pub current_origin: Vec2,
}
