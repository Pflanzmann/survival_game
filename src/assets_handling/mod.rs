use bevy::prelude::{App, Plugin};
use crate::assets_handling::preload_bullet_system::{BulletConfigHandles, preload_bullet_system};

use crate::assets_handling::preload_enemy_system::{EnemyConfigHandles, preload_enemy_system};
use crate::assets_handling::preload_item_system::{ItemConfigHandles, preload_item_system};
use crate::assets_handling::preload_player_system::{PlayerConfigHandles, preload_player_system};
use crate::assets_handling::preload_texture_system::{preload_texture_system, TextureHandles};
use crate::SetupStages;

pub mod preload_texture_system;
pub mod preload_enemy_system;
pub mod configurations;
pub mod preload_item_system;
pub mod preload_player_system;
pub mod preload_bullet_system;

pub struct AssetHandlingPlugin;

impl Plugin for AssetHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureHandles>()
            .init_resource::<EnemyConfigHandles>()
            .init_resource::<ItemConfigHandles>()
            .init_resource::<PlayerConfigHandles>()
            .init_resource::<BulletConfigHandles>()

            .add_startup_system_to_stage(SetupStages::AssetSetup, preload_texture_system)
            .add_startup_system_to_stage(SetupStages::ConfigSetup, preload_enemy_system)
            .add_startup_system_to_stage(SetupStages::ConfigSetup, preload_item_system)
            .add_startup_system_to_stage(SetupStages::ConfigSetup, preload_player_system)
            .add_startup_system_to_stage(SetupStages::ConfigSetup, preload_bullet_system);
    }
}