use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{App, AssetServer, BuildChildren, Color, Commands, Entity, GlobalTransform, Input, KeyCode, Name, OrthographicCameraBundle, Plugin, Res, Sprite, SpriteBundle, SystemStage, Transform, Vec2, Vec3, With, Without};
//use bevy_inspector_egui::egui::ImageData::Color;
use bevy_inspector_egui::WorldInspectorPlugin;

use components::player_components::Player;

use crate::ai::ai_plugin::AiPlugin;
use crate::bullets::bullet_plugin::BulletPlugin;
use crate::collision::collision_components::Collider;
use crate::collision::collision_plugin::CollisionPlugin;
use crate::components::gun_components::BasicGun;
use crate::components::unit_stats_components::{Damage, Direction, Health, MoveSpeed, UnitSize};
use crate::components::ui_components::{HealthBar};
use crate::guns::gun_plugin::GunPlugin;
use crate::input::input_plugin::InputPlugin;
use crate::KeyCode::D;

mod input;
mod ai;
mod collision;
mod guns;
mod bullets;
mod components;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
pub enum SetupStages {
    PlayerSetup,
    AfterPlayerSetup,
}

fn main() {
    App::new()
        .add_startup_stage(SetupStages::PlayerSetup, SystemStage::single_threaded())
        .add_startup_stage(SetupStages::AfterPlayerSetup, SystemStage::single_threaded())
        .add_startup_system_to_stage(SetupStages::PlayerSetup, setup_player)

        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .add_plugin(InputPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)

        .add_startup_system(setup_tiles)
        .add_startup_system_to_stage(SetupStages::AfterPlayerSetup, setup_healthBar)
        .run()
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: asset_server.load("NickelMan.png"),
            ..Default::default()
        },
    )
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(MoveSpeed { move_speed: 10.0 })
        .insert(MoveSpeed { move_speed: 10.0 })
        .insert(UnitSize { collider_size: Vec2::new(256.0, 256.0) })
        .insert(Collider)
        .insert(Direction { direction: Vec3::new(1.0, 0.0, 0.0) })
        .insert(BasicGun)
        .insert(Damage { damage: 5.0 })
        .insert(Health { health: 50.0 });
}

pub fn setup_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let background = commands.spawn().insert(Name::new("background")).id();

    for x in 0..100 {
        for y in 0..100 {
            let child = commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("BackgroundTile.png").into(),
                global_transform: GlobalTransform::from(Transform::from_xyz((x.clone() * 256) as f32, (y.clone() * 256) as f32, -100.0)),
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

pub fn setup_healthBar(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(
        SpriteBundle{
            sprite: Sprite{
                custom_size: Some(Vec2::new(256.0,50.0)),
                color: Color::rgb(0.7,0.7,0.7),
                ..Default::default()
            },
            transform: Transform{
                translation : Vec3::new(0.0,-125.0,0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .insert(HealthBar);
}