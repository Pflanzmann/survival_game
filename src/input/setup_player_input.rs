use bevy::prelude::*;
use crate::ai::ai_components::{Size, Speed, Direction};
use crate::collision::collision_components::Collider;

use crate::input::input_components::MainCamera;
use crate::Player;

pub fn setup_player_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let parent = commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: asset_server.load("NickelMan.png"),
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Speed { speed: 10.0 })
        .insert(Size { size: Vec2::new(256.0, 256.0) })
        .insert(Collider)
        .insert(Direction { direction: Vec3::default()})
        .id();

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.x = 0.0;
    camera_bundle.transform.translation.y = 0.0;
    camera_bundle.transform.translation.z = 0.0;

    camera_bundle.orthographic_projection.scale = 5.0;

    let child = commands.spawn_bundle(camera_bundle).insert(MainCamera).id();

    commands.entity(parent).push_children(&[child]);
}
