use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::models::main_camera::MainCamera;
use crate::models::player::Player;

pub fn setup_camera_system(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.x = 0.0;
    camera_bundle.transform.translation.y = 0.0;
    camera_bundle.transform.translation.z = 400000000000000.0;

    camera_bundle.projection.scale = 5.0;
    camera_bundle.projection.left = -1280.0;
    camera_bundle.projection.right = 1280.0;
    camera_bundle.projection.bottom = -688.5;
    camera_bundle.projection.top = 688.5;
    camera_bundle.projection.far = 10000000000000000000000000.0;

    camera_bundle.projection.scaling_mode = ScalingMode::None;
    camera_bundle.projection.scale = 3.0;

    commands.spawn_bundle(camera_bundle)
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}
