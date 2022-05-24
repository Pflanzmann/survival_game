use bevy::prelude::{App, Plugin, SystemSet};

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_animation_system::preload_animation_system;
use crate::assets_handling::preload_audio_system::{preload_audio_system, SoundHandles};
use crate::assets_handling::preload_bullet_system::{BulletConfigHandles, preload_bullet_system};
use crate::assets_handling::preload_enemy_system::preload_enemy_system;
use crate::assets_handling::preload_item_system::{ItemConfigHandles, preload_item_system};
use crate::assets_handling::preload_mod_system::preload_mod_system;
use crate::assets_handling::preload_player_system::{PlayerConfigHandles, preload_player_system};
use crate::assets_handling::preload_stage_spawn_system::{preload_stage_spawn_behvaior_system, StageSpawnBehaviorHandle};
use crate::assets_handling::preload_texture_system::{preload_texture_system, TextureHandles};
use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::SetupStages;
use crate::util::entity_builder::EntityBuilderPlugin;

pub mod preload_texture_system;
pub mod preload_enemy_system;
pub mod preload_item_system;
pub mod preload_player_system;
pub mod preload_bullet_system;
pub mod preload_mod_system;
pub mod preload_audio_system;
pub mod preload_animation_system;
pub mod preload_stage_spawn_system;

/// This plugin serves as a Preloader for all [ Assets ].
///
/// Currently it is divided into different data types like
/// [ preload_texture_system ] for sprites or
/// [ preload_item_system ] for items
///
/// The systems are run in the custom Startupstage [ AssetSetup ] in
/// order to have them ready when the game starts
pub struct AssetHandlingPlugin;

impl Plugin for AssetHandlingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EntityBuilderPlugin)

            .init_resource::<TextureHandles>()
            .init_resource::<EnemyConfigHandles>()
            .init_resource::<ItemConfigHandles>()
            .init_resource::<PlayerConfigHandles>()
            .init_resource::<BulletConfigHandles>()
            .init_resource::<SoundHandles>()
            .init_resource::<AtlasHandles>()
            .init_resource::<StageSpawnBehaviorHandle>()

            .add_startup_system_to_stage(SetupStages::AssetSetup, preload_texture_system)

            .add_startup_system_set_to_stage(
                SetupStages::ConfigSetup,
                SystemSet::new()
                    .with_system(preload_enemy_system)
                    .with_system(preload_item_system)
                    .with_system(preload_player_system)
                    .with_system(preload_bullet_system)
                    .with_system(preload_mod_system)
                    .with_system(preload_audio_system)
                    .with_system(preload_animation_system)
                    .with_system(preload_stage_spawn_behvaior_system),
            );
    }
}