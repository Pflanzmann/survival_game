use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, WindowResolution};

use crate::models::main_camera::MainCamera;

pub fn setup_camera_system(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.x = 0.0;
    camera_bundle.transform.translation.y = 0.0;
    camera_bundle.transform.translation.z = 400000000000000.0;

    camera_bundle.projection.scale = 4.0;
    camera_bundle.projection.far = 10000000000000000000000000.0;

    camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 1920.0, height: 1080.0 };

    commands.spawn(camera_bundle)
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));

    for mut window in window_query.iter_mut() {
        window.title = String::from("Atomic Under Chess");
        window.resolution = WindowResolution::new(1920.0, 1080.0);
        window.resizable = false;
    }
}
