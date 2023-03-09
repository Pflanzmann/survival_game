use bevy::prelude::{Entity, Vec2, Resource};

pub struct TileData {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Resource)]
pub struct BackgroundTilesResource {
    pub current_origin: Vec2,
    pub tiles: Vec<TileData>,
}
