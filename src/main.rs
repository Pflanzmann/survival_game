use bevy::DefaultPlugins;
use bevy::ecs::prelude::Query;
use bevy::prelude::{App, AssetServer, BuildChildren, Commands, Entity, GlobalTransform, Input, KeyCode, Name, OrthographicCameraBundle, Plugin, Res, Sprite, SpriteBundle, SystemStage, Transform, Vec2, Vec3, With, Without};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::ecs::schedule::StageLabel;
use components::player_components::Player;

use crate::ai::ai_plugin::AiPlugin;
use crate::bullets::bullet_plugin::BulletPlugin;
use crate::collision::collision_components::Collider;
use crate::collision::collision_plugin::CollisionPlugin;
use crate::components::gun_components::BasicGun;
use crate::components::unit_stats_components::{Direction, ColliderSize, Speed};
use crate::guns::gun_plugin::GunPlugin;
use crate::input::input_plugin::InputPlugin;

mod input;
mod ai;
mod collision;
mod guns;
mod bullets;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())

        .add_plugin(InputPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GunPlugin)
        .add_plugin(BulletPlugin)

        .add_startup_system(setup_tiles)
        .run()
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