use bevy::prelude::{App, Plugin};

use crate::resources::background_tiles_resource::BackgroundTilesResource;
use crate::resources::ui_resources::CoinCount;
use crate::{AppState, ToAppState, Vec2};
use crate::resources::state_resources::AppStateTrigger;

pub mod ui_resources;
pub mod background_tiles_resource;
pub mod state_resources;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CoinCount>()
            .init_resource::<AppStateTrigger>()
            .insert_resource::<BackgroundTilesResource>(BackgroundTilesResource {
                current_origin: Vec2::new(-1.0, -1.0),
                tiles: Vec::new(),
            });
    }
}