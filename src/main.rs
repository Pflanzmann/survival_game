extern crate core;

use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{States, SystemSet};
use bevy_kira_audio::AudioPlugin;

use crate::animation::AnimationPlugin;
use crate::assets_handling::AssetHandlingPlugin;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::audio::CustomAudioPlugin;
use crate::collision::CollisionPlugin;
use crate::input::InputPlugin;
use crate::models::events::EventsPlugin;
use crate::models::resources::ResourcePlugin;
use crate::models::sprite_layer::SpriteLayer;
use crate::scheduling::SchedulingPlugin;
use crate::ui::UiPlugin;
use crate::units::UnitPlugin;
use crate::util::debug::DebugPlugin;
use crate::world::WorldPlugin;

mod input;
mod units;
mod collision;
mod models;
mod assets_handling;
mod util;
mod ui;
mod scheduling;
mod audio;
mod animation;
mod world;

const SPRITE_ROW_LENGTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum SetupStages {
    ConfigSetup,
    AssetSetup,
    PlayerSetup,
    AfterPlayerSetup,
    StateStage,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Pre,
    MainMenu,
    Loading,
    InGame,
    GameOver,
    Paused,
    Shop,
    GameWon,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ConsoleState {
    #[default]
    Hidden,
    Shown,
}

fn main() {
    App::new()
        // .add_startup_stage(SetupStages::ConfigSetup, SystemSet::parallel())
        // .add_startup_stage(SetupStages::AssetSetup, SystemSet::parallel())
        .add_plugins(DefaultPlugins)

        .add_state::<AppState>()
        .add_state::<ConsoleState>()

        .add_plugin(UiPlugin)
        .add_plugin(CustomAudioPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(SchedulingPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(AssetHandlingPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(DebugPlugin)

        .add_plugin(AudioPlugin)

        .run()
}

