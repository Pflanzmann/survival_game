use bevy::prelude::{App, Plugin};

use crate::resources::background_tiles_resource::BackgroundTilesResource;
use crate::resources::ui_resources::CoinCount;
use crate::Vec2;

pub mod ui_resources;
pub mod background_tiles_resource;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CoinCount>()
            .insert_resource::<BackgroundTilesResource>(BackgroundTilesResource {
                current_origin: Vec2::new(-1.0, -1.0),
                tiles: Vec::new(),
            });
    }
}