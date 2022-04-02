use std::fs::{File, read_to_string};
use std::path::Path;
use std::error::Error;
use std::io::{BufReader, copy, stdout};
use std::ops::Index;
use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{App, AssetServer, BuildChildren, Commands, Entity, GlobalTransform, Input, KeyCode, Name, OrthographicCameraBundle, Plugin, Res, Sprite, SpriteBundle, SystemStage, Transform, Val, Vec2, Vec3, With, Without};
use bevy_inspector_egui::WorldInspectorPlugin;
use rustc_serialize::json::Json;
use serde::{Serialize, Deserialize};

use components::collision_components::Collider;
use components::player_components::Player;

use crate::assets_handling::asset_handling_plugin::AssetHandlingPlugin;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::bullets::bullet_plugin::BulletPlugin;
use crate::collision::collision_plugin::CollisionPlugin;
use crate::components::gun_components::Gunnable;
use crate::components::ui_components::HealthBar;
use crate::components::unit_stats_components::{Damage, FacingDirection, Health, MoveSpeed, UnitSize};
use crate::drops::drops_plugin::DropsPlugin;
use crate::guns::gun_plugin::GunPlugin;
use crate::input::input_plugin::InputPlugin;
use crate::units::unit_plugin::UnitPlugin;

mod input;
mod units;
mod collision;
mod guns;
mod bullets;
mod components;
mod drops;
mod assets_handling;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
pub enum SetupStages {
    AssetSetup,
    PlayerSetup,
    AfterPlayerSetup,
}


#[derive(Deserialize, Debug)]
struct User {
    FirstName: String,
    LastName: String,
    Age: f32
}

fn main() {
    App::new()
        .add_startup_stage(SetupStages::PlayerSetup, SystemStage::single_threaded())
        .add_startup_stage(SetupStages::AfterPlayerSetup, SystemStage::single_threaded())

        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .add_plugin(InputPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(DropsPlugin)
        .add_plugin(AssetHandlingPlugin)

        .add_startup_system(json_serialize_system)

        .add_startup_system_to_stage(SetupStages::PlayerSetup, setup_tiles)
        .run()
}

pub fn setup_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handles: Res<TextureHandles>
) {
    let background = commands.spawn().insert(Name::new("background")).id();

    for x in 0..30 {
        for y in 0..30 {
            let child = commands.spawn_bundle(SpriteBundle {
                texture: texture_handles.background_tile.clone(),
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

pub fn json_serialize_system(){

    let mut my_string = match read_to_string("assets/Json/test.json") {
        Ok(value) => value,
        Err(_) => String::new(),
    };

    let u: User = serde_json::from_str(&my_string).expect("JSON was not well-formatted");

    println!("{:#?}", u);
}

