use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{AlignItems, AlignSelf, App, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, DetectChanges, Entity, FlexDirection, Font, GlobalTransform, HorizontalAlign, Interaction, JustifyContent, Name, NodeBundle, OrthographicCameraBundle, Plugin, PositionType, Rect, Res, ResMut, Size, Sprite, SpriteBundle, Style, SystemSet, SystemStage, Text, TextAlignment, TextBundle, TextStyle, Transform, UiCameraBundle, Val, Vec2, Vec3, VerticalAlign, With, Without};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

use models::player_components::Player;

use crate::assets_handling::AssetHandlingPlugin;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::background::BackgroundPlugin;
use crate::bullets::BulletPlugin;
use crate::collision::CollisionPlugin;
use crate::drops::DropsPlugin;
use crate::guns::GunPlugin;
use crate::input::InputPlugin;
use crate::models::events::EventsPlugin;
use crate::models::gun_components::Gunnable;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::ui_components::{Cointext, HealthBar};
use crate::models::unit_stats_components::{Damage, MoveDirection, Health, MoveSpeed, UnitSize};
use crate::resources::ResourcePlugin;
use crate::resources::ui_resources::CoinCount;
use crate::ui::UiPlugin;
use crate::units::UnitPlugin;

mod input;
mod units;
mod collision;
mod guns;
mod bullets;
mod models;
mod drops;
mod assets_handling;
mod util;
mod resources;
mod ui;
mod background;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum SetupStages {
    ConfigSetup,
    AssetSetup,
    PlayerSetup,
    AfterPlayerSetup,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    Paused,
}

fn main() {
    App::new()
        .add_startup_stage(SetupStages::ConfigSetup, SystemStage::parallel())
        .add_startup_stage(SetupStages::AssetSetup, SystemStage::parallel())
        .add_startup_stage(SetupStages::PlayerSetup, SystemStage::single_threaded())
        .add_startup_stage(SetupStages::AfterPlayerSetup, SystemStage::single_threaded())

        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .add_state(AppState::InGame)

        .add_plugin(EventsPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(DropsPlugin)
        .add_plugin(AssetHandlingPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(BackgroundPlugin)
        .run()
}






