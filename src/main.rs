use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{App, BuildChildren, Color, Commands, DetectChanges, Entity, Font, GlobalTransform, HorizontalAlign, Name, OrthographicCameraBundle, Plugin, Res, ResMut, Sprite, SpriteBundle, SystemSet, SystemStage, Text, TextAlignment, TextBundle, TextStyle, Transform, UiCameraBundle, Val, Vec2, Vec3, VerticalAlign, With, Without};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

use models::player_components::Player;

use crate::assets_handling::AssetHandlingPlugin;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::bullets::BulletPlugin;
use crate::collision::CollisionPlugin;
use crate::drops::DropsPlugin;
use crate::guns::GunPlugin;
use crate::input::InputPlugin;
use crate::models::events::EventsPlugin;
use crate::models::gun_components::Gunnable;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::ui_components::{Cointext, HealthBar};
use crate::models::unit_stats_components::{Damage, FacingDirection, Health, MoveSpeed, UnitSize};
use crate::resources::ResourcePlugin;
use crate::resources::ui_resources::CoinCount;
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

        .add_startup_system(spawn_text_system)
        .add_system(update_text_system)

        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(spawn_menu_system)
        )

        .add_startup_system_to_stage(SetupStages::PlayerSetup, setup_tiles)
        .run()
}

pub fn setup_tiles(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
) {
    let background = commands.spawn().insert(Name::new("background")).id();

    for x in 0..30 {
        for y in 0..30 {
            let child = commands.spawn_bundle(SpriteBundle {
                texture: texture_handles.background_tile.clone(),
                global_transform: GlobalTransform::from(Transform::from_xyz((x * 256) as f32, (y * 256) as f32, SpriteLayer::FloorLevel.get_layer_z())),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                ..Default::default()
            }).id();

            commands.entity(background).add_child(child);
        }
    }
}

pub fn spawn_text_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut coin_counter: ResMut<CoinCount>,
) {
    coin_counter.number = 0;

    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Coins: ".to_string(),
            TextStyle {
                font: asset_loader.load("BodoniFLF-Roman.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
        .insert(Cointext)
        .id();
}

pub fn update_text_system(
    mut text_query: Query<&mut Text, With<Cointext>>,
    mut coin_counter: ResMut<CoinCount>,
) {
    if coin_counter.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:.0}", coin_counter.number);
        }
    }
}

pub fn spawn_menu_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
) {
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "You Dieded".to_string(),
            TextStyle {
                font: asset_loader.load("BodoniFLF-Roman.ttf"),
                font_size: 60.0,
                color: Color::RED,
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
        .id();
}
