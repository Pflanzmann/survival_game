use bevy::prelude::*;

use crate::entities::player_components::MainCamera;
use crate::Player;

pub fn setup_camera_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    let player_result = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.x = 0.0;
    camera_bundle.transform.translation.y = 0.0;
    camera_bundle.transform.translation.z = 10.0;

    camera_bundle.orthographic_projection.scale = 5.0;

    let child = commands.spawn_bundle(camera_bundle)
        .insert(MainCamera)
        //.insert_bundle(UiCameraBundle::default())
        .id();

    commands.entity(player_result).push_children(&[child]);
}
