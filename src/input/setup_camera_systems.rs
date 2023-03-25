use bevy::prelude::*;

use crate::models::main_camera::MainCamera;

pub fn setup_camera_system(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.x = 0.0;
    camera_bundle.transform.translation.y = 0.0;
    camera_bundle.transform.translation.z = 400000000000000.0;

    camera_bundle.projection.scale = 5.0;
    camera_bundle.projection.area = Rect::new(-1280.0, -688.5, 1280.0, 688.5);
    camera_bundle.projection.far = 10000000000000000000000000.0;

    camera_bundle.projection.scale = 3.0;

    commands.spawn(camera_bundle)
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}
