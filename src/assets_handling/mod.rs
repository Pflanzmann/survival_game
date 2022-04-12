use bevy::prelude::{App, Plugin, SystemSet};

use crate::assets_handling::preload_bullet_system::{BulletConfigHandles, preload_bullet_system};
use crate::assets_handling::preload_enemy_system::{EnemyConfigHandles, preload_enemy_system};
use crate::assets_handling::preload_item_system::{ItemConfigHandles, preload_item_system};
use crate::assets_handling::preload_mod_system::preload_mod_system;
use crate::assets_handling::preload_player_system::{PlayerConfigHandles, preload_player_system};
use crate::assets_handling::preload_texture_system::{preload_texture_system, TextureHandles};
use crate::SetupStages;

pub mod preload_texture_system;
pub mod preload_enemy_system;
pub mod preload_item_system;
pub mod preload_player_system;
pub mod preload_bullet_system;
pub mod preload_mod_system;

pub struct AssetHandlingPlugin;

impl Plugin for AssetHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureHandles>()
            .init_resource::<EnemyConfigHandles>()
            .init_resource::<ItemConfigHandles>()
            .init_resource::<PlayerConfigHandles>()
            .init_resource::<BulletConfigHandles>()

            .add_startup_system_to_stage(SetupStages::AssetSetup, preload_texture_system)

            .add_startup_system_set_to_stage(
                SetupStages::ConfigSetup,
                SystemSet::new()
                    .with_system(preload_enemy_system)
                    .with_system(preload_item_system)
                    .with_system(preload_player_system)
                    .with_system(preload_bullet_system)
                    .with_system(preload_mod_system),
            );
    }
}