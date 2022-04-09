use bevy::app::CoreStage;
use bevy::app::CoreStage::PreUpdate;
use bevy::core::FixedTimestep;
use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{AlignItems, AlignSelf, App, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, DespawnRecursiveExt, DetectChanges, Entity, FlexDirection, Font, GlobalTransform, HorizontalAlign, Input, Interaction, JustifyContent, KeyCode, Name, NodeBundle, OrthographicCameraBundle, Plugin, PositionType, Rect, Res, ResMut, Size, Sprite, SpriteBundle, State, Style, SystemSet, SystemStage, Text, TextAlignment, TextBundle, TextStyle, Time, Transform, UiCameraBundle, Val, Vec2, Vec3, VerticalAlign, With, Without};
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
use crate::resources::state_resources::AppStateTrigger;
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
    StateStage
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

#[derive(Debug)]
pub enum ToAppState {
    ToPre,
    ToMainMenu,
    ToLoading,
    ToInGame,
    ToGameOver,
    ToPaused,
    None,
}

impl Default for ToAppState {
    fn default() -> Self { ToAppState::None }
}

#[derive(Default)]
pub struct StateTimer(f32);

const FIXED_TIMESTEP_Pause: f64 = 0.1;

fn main() {
    App::new()
        .add_startup_stage(SetupStages::ConfigSetup, SystemStage::parallel())
        .add_startup_stage(SetupStages::AssetSetup, SystemStage::parallel())
        //.add_startup_stage(SetupStages::PlayerSetup, SystemStage::single_threaded())
        //.add_startup_stage(SetupStages::AfterPlayerSetup, SystemStage::parallel())

        .add_stage_before(CoreStage::PreUpdate, SetupStages::AfterPlayerSetup, SystemStage::parallel())
        .add_stage_before(CoreStage::PreUpdate, SetupStages::PlayerSetup, SystemStage::parallel())
        //.add_stage_before(CoreStage::PreUpdate, SetupStages::AssetSetup, SystemStage::parallel())
        //.add_stage_before(CoreStage::PreUpdate, SetupStages::ConfigSetup, SystemStage::parallel())

        .add_state_to_stage(CoreStage::PostUpdate, AppState::MainMenu)
        .add_state_to_stage(CoreStage::Last, AppState::MainMenu)

        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::AfterPlayerSetup, AppState::Paused)*/

        //.add_state_to_stage(SetupStages::AssetSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::AssetSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::AssetSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::AssetSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::AssetSetup, AppState::Paused)*/

        .add_state_to_stage(SetupStages::PlayerSetup, AppState::MainMenu)
        /*.add_state_to_stage(SetupStages::PlayerSetup, AppState::Loading)
        .add_state_to_stage(SetupStages::PlayerSetup, AppState::InGame)
        .add_state_to_stage(SetupStages::PlayerSetup, AppState::GameOver)
        .add_state_to_stage(SetupStages::PlayerSetup, AppState::Paused)*/

        //.add_state_to_stage(SetupStages::ConfigSetup, AppState::MainMenu)
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

        //.add_system(show_current_state)
        .add_system_to_stage(CoreStage::PreUpdate, execute_state_switch_system)
        //.add_system(execute_state_switch_system)
        .add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(FIXED_TIMESTEP_Pause))
            .with_system(toggle_pause_system)
        )

        .init_resource::<StateTimer>()

        .run()
}

pub fn execute_state_switch_system(
    mut state_trigger: ResMut<AppStateTrigger>,
    mut app_state: ResMut<State<AppState>>,
) {
    /*state_timer.0 += time.delta().as_secs_f32();
    if state_timer.0 < 2.0 {
        return;
    }
    state_timer.0 = 0.0;*/

    match state_trigger.State_Change_Trigger {
        ToAppState::ToPre => {
            if app_state.current() != &AppState::Pre {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::Pre).unwrap();
            }
        }
        ToAppState::ToMainMenu => {
            if app_state.current() != &AppState::MainMenu {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::MainMenu).unwrap();
            }
        }
        ToAppState::ToLoading => {
            if app_state.current() != &AppState::Loading {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::Loading).unwrap();
            }
        }
        ToAppState::ToInGame => {
            if app_state.current() != &AppState::InGame {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::InGame).unwrap();
            }
        }
        ToAppState::ToGameOver => {
            if app_state.current() != &AppState::GameOver {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::GameOver).unwrap();
            }
        }
        ToAppState::ToPaused => {
            if app_state.current() != &AppState::Paused {
                state_trigger.State_Change_Trigger = ToAppState::None;
                app_state.set(AppState::Paused).unwrap();
            }
        }
        ToAppState::None => {}
    }
}

pub fn toggle_pause_system(
    input: Res<Input<KeyCode>>,
    mut state_trigger: ResMut<AppStateTrigger>,
    app_state: ResMut<State<AppState>>,

    mut state_timer: ResMut<StateTimer>,
    time: Res<Time>,
) {
    state_timer.0 += time.delta().as_secs_f32();
    if state_timer.0 < 0.1 {
        return;
    }


    if input.pressed(KeyCode::Space) {
        println!("pause");
        state_timer.0 = 0.0;
        match app_state.current() {
            AppState::Pre => {}
            AppState::MainMenu => {}
            AppState::Loading => {}
            AppState::InGame => { state_trigger.State_Change_Trigger = ToAppState::ToPaused; }
            AppState::GameOver => {}
            AppState::Paused => { state_trigger.State_Change_Trigger = ToAppState::ToInGame; }
        }
    }
}

pub fn show_current_state(
    app_state: ResMut<State<AppState>>
) {
    match app_state.current() {
        AppState::Pre => { println!("pre") }
        AppState::MainMenu => { println!("MainMenu") }
        AppState::Loading => { println!("Loading") }
        AppState::InGame => { println!("InGame") }
        AppState::GameOver => { println!("GameOver") }
        AppState::Paused => { println!("Paused") }
    }
}

pub fn trigger_enter_main_system(
    mut app_state: ResMut<State<AppState>>,
    mut state_trigger: ResMut<AppStateTrigger>,
) {
    state_trigger.State_Change_Trigger = ToAppState::ToMainMenu;
    //app_state.set(AppState::MainMenu).unwrap();
}

pub fn close_main_menu_system(
    mut commands : Commands,
    my_query : Query<Entity, With<MainMenuComp>>
){
    for entity in my_query.iter() {
        commands.entity(entity).despawn_recursive();
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


