use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{App, BuildChildren, Commands, Entity, GlobalTransform, Name, OrthographicCameraBundle, Plugin, Res, Sprite, SpriteBundle, SystemStage, Transform, Val, Vec2, Vec3, With, Without};
use bevy_inspector_egui::WorldInspectorPlugin;

use components::collision_components::Collider;
use components::player_components::Player;

use crate::assets_handling::AssetHandlingPlugin;
use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::bullets::BulletPlugin;
use crate::collision::CollisionPlugin;
use crate::components::gun_components::Gunnable;
use crate::components::ui_components::HealthBar;
use crate::components::unit_stats_components::{Damage, FacingDirection, Health, MoveSpeed, UnitSize};
use crate::drops::DropsPlugin;
use crate::guns::GunPlugin;
use crate::input::InputPlugin;
use crate::units::UnitPlugin;

mod input;
mod units;
mod collision;
mod guns;
mod bullets;
mod components;
mod drops;
mod assets_handling;
mod util;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum SetupStages {
    ConfigSetup,
    AssetSetup,
    PlayerSetup,
    AfterPlayerSetup,
}

fn main() {
    App::new()
        .add_startup_stage(SetupStages::ConfigSetup, SystemStage::parallel())
        .add_startup_stage(SetupStages::AssetSetup, SystemStage::parallel())
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



