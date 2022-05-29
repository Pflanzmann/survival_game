extern crate core;

use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{Msaa, SystemStage};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

use crate::animation::AnimationPlugin;
use crate::assets_handling::AssetHandlingPlugin;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::audio::CustomAudioPlugin;
use crate::collision::CollisionPlugin;
use crate::input::InputPlugin;
use crate::models::events::EventsPlugin;
use crate::models::resources::ResourcePlugin;
use crate::models::resources::state_resources::AppStateTrigger;
use crate::models::sprite_layer::SpriteLayer;
use crate::navigation::NavigationPlugin;
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
mod navigation;
mod audio;
mod animation;
mod world;

const SPRITE_ROW_LENGTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum SetupStages {
    ConfigSetup,
    AssetSetup,
    PlayerSetup,
    AfterPlayerSetup,
    StateStage,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Pre,
    MainMenu,
    Loading,
    InGame,
    GameOver,
    Paused,
    Shop,
}

#[derive(Debug)]
pub enum ToAppState {
    ToPre,
    ToMainMenu,
    ToLoading,
    ToInGame,
    ToGameOver,
    ToPaused,
    ToShop,
    None,
}

impl Default for ToAppState {
    fn default() -> Self { ToAppState::None }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConsoleState {
    Shown,
    Hidden,
}

fn main() {
    App::new()
        .add_startup_stage(SetupStages::ConfigSetup, SystemStage::parallel())
        .add_startup_stage(SetupStages::AssetSetup, SystemStage::parallel())

        .insert_resource(Msaa { samples: 1 })

        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .add_state(AppState::Pre)
        .add_state(ConsoleState::Hidden)

        .add_plugin(UiPlugin)
        .add_plugin(CustomAudioPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(NavigationPlugin)
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

