use bevy::prelude::{App, Plugin, Vec2};

use crate::models::resources::background_tiles_resource::BackgroundTilesResource;
use crate::models::resources::console_history::{ConsoleHistory, read_history_from_file};
use crate::models::resources::spawn_task_receiver::SpawnTaskReceiver;
use crate::models::resources::spawn_timer::SpawnTimer;
use crate::models::resources::state_resources::AppStateTrigger;
use crate::models::resources::ui_resources::GoldWallet;

pub mod ui_resources;
pub mod background_tiles_resource;
pub mod state_resources;
pub mod spawn_timer;
pub mod spawn_task_receiver;
pub mod console_history;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GoldWallet>()
            .init_resource::<AppStateTrigger>()
            .init_resource::<SpawnTimer>()
            .init_resource::<SpawnTaskReceiver>()

            .insert_resource::<ConsoleHistory>(read_history_from_file())

            .insert_resource::<BackgroundTilesResource>(BackgroundTilesResource {
                current_origin: Vec2::new(-1.0, -1.0),
                tiles: Vec::new(),
            });
    }
}

