use bevy::prelude::{ResMut, Resource};

use crate::models::configurations::projectile_config::ProjectileConfig;
use crate::util::read_file_to_string::read_file_to_string;

#[derive(Default, Resource)]
pub struct ProjectileConfigHandles {
    pub basic_projectile: ProjectileConfig,
}

pub fn preload_projectile_system(
    mut projectile_handles: ResMut<ProjectileConfigHandles>,
) {
    let my_string = read_file_to_string("configurations/projectile.json");
    projectile_handles.basic_projectile = serde_json::from_str(&my_string).expect("JSON was not well-formatted || BASIC_BULLET");
}