use bevy::prelude::{App, Plugin};

use crate::resources::ui_resources::CoinCount;

pub mod ui_resources;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoinCount>();
    }
}