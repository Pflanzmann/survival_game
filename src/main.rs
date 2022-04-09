use bevy::app::CoreStage;
use bevy::app::CoreStage::PreUpdate;
use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{AlignItems, AlignSelf, App, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, DetectChanges, Entity, FlexDirection, Font, GlobalTransform, HorizontalAlign, Interaction, JustifyContent, Name, NodeBundle, OrthographicCameraBundle, Plugin, PositionType, Rect, Res, ResMut, Size, Sprite, SpriteBundle, State, Style, SystemSet, SystemStage, Text, TextAlignment, TextBundle, TextStyle, Transform, UiCameraBundle, Val, Vec2, Vec3, VerticalAlign, With, Without};
use bevy::render::camera::camera_system;
use bevy::utils::tracing::event;
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
use crate::models::ui_components::{Cointext, HealthBar, MainMenuComp};
use crate::models::unit_stats_components::{Damage, Health, MoveDirection, MoveSpeed, UnitSize};
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
    Pre,
    MainMenu,
    Loading,
    InGame,
    GameOver,
    Paused,
}

fn main() {
    App::new()
        //.add_startup_stage(SetupStages::ConfigSetup, SystemStage::parallel())
        //.add_startup_stage(SetupStages::AssetSetup, SystemStage::parallel())
        //.add_startup_stage(SetupStages::PlayerSetup, SystemStage::single_threaded())
        //.add_startup_stage(SetupStages::AfterPlayerSetup, SystemStage::parallel())

        .add_stage_before(CoreStage::PreUpdate, SetupStages::AfterPlayerSetup, SystemStage::parallel())
        .add_stage_before(CoreStage::PreUpdate, SetupStages::PlayerSetup, SystemStage::parallel())
        .add_stage_before(CoreStage::PreUpdate, SetupStages::AssetSetup, SystemStage::parallel())
        .add_stage_before(CoreStage::PreUpdate, SetupStages::ConfigSetup, SystemStage::parallel())

        .add_state_to_stage(CoreStage::PostUpdate, AppState::MainMenu)
        .add_state_to_stage(CoreStage::Last, AppState::MainMenu)

        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::Paused)*/

        .add_state_to_stage(SetupStages::AssetSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::AssetSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::AssetSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::AssetSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::AssetSetup, AppState::Paused)*/

        .add_state_to_stage(SetupStages::PlayerSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::PlayerSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::PlayerSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::PlayerSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::PlayerSetup, AppState::Paused)*/

        .add_state_to_stage(SetupStages::ConfigSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::ConfigSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::ConfigSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::ConfigSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::ConfigSetup, AppState::Paused)*/


        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .add_state(AppState::Pre)

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

        .add_system_set(SystemSet::on_enter(AppState::MainMenu)
            .with_system(spawn_main_menu_system)
        )

        .add_system_set(SystemSet::on_exit(AppState::MainMenu)
            .with_system(close_main_menu_system)
        )

        .add_system_set(SystemSet::on_update(AppState::Pre)
            .with_system(trigger_enter_main_system)
        )

        .run()
}

pub fn trigger_enter_main_system(
    mut app_state: ResMut<State<AppState>>
){
    app_state.set(AppState::MainMenu).unwrap();
}

pub fn close_main_menu_system(
    mut commands : Commands,
    my_query : Query<Entity, With<MainMenuComp>>
){
    for entity in my_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_main_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
){
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                position: Rect {
                    left: Val::Percent(10.0),
                    bottom: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    right: Val::Percent(10.0),
                },
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    "Atomic UndersurVampire".to_string(),
                    TextStyle {
                        font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                        font_size: 60.0,
                        color: Color::RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(25.0), Val::Percent(10.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Start Game".to_string(),
                        TextStyle {
                            font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            });
        })
        .insert(MainMenuComp)
        .id();
}


