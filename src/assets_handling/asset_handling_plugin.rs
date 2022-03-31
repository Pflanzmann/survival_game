use bevy::prelude::{App, Plugin};

use crate::assets_handling::preload_texture_system::{preload_texture_system, TextureHandles};

pub struct AssetHandlingPlugin;

impl Plugin for AssetHandlingPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureHandles>()

            .add_startup_system(preload_texture_system);
    }
}