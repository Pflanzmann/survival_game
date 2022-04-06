use bevy::prelude::{Entity, Vec2};

pub struct TileData {
    pub entity: Entity,
    pub position: Vec2,
}

pub struct BackgroundTilesResource {
    pub current_origin: Vec2,
    pub tiles: Vec<TileData>,
}
