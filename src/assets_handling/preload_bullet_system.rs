use bevy::prelude::ResMut;

use crate::models::configurations::bullet_config::BulletConfig;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default)]
pub struct BulletConfigHandles {
    pub basic_bullet: BulletConfig,
}

pub fn preload_bullet_system(
    mut bullet_handles: ResMut<BulletConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/bullet.json");
    bullet_handles.basic_bullet = serde_json::from_str(&my_string).expect("JSON was not well-formatted || BASIC_BULLET");
}