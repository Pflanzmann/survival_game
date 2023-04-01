use bevy::asset::AssetSet;
use bevy::prelude::*;

use crate::assets_handling::preload_animation_system::AtlasHandles;
use crate::assets_handling::preload_animation_system::preload_animation_system;
use crate::assets_handling::preload_audio_system::{preload_audio_system, SoundHandles};
use crate::assets_handling::preload_enemy_system::preload_enemy_system;
use crate::assets_handling::preload_item_system::{ItemConfigHandles, preload_item_system};
use crate::assets_handling::preload_mod_system::preload_mod_system;
use crate::assets_handling::preload_player_system::{PlayerConfigHandles, preload_player_system};
use crate::assets_handling::preload_projectile_system::{preload_projectile_system, ProjectileConfigHandles};
use crate::assets_handling::preload_stage_spawn_system::{preload_stage_spawn_behavior_system, StageSpawnBehaviorHandle};
use crate::assets_handling::preload_texture_system::{preload_texture_system, TextureHandles};
use crate::assets_handling::preload_world_grid_config_system::preload_world_grid_config_system;
use crate::models::spawner::enemy_config_handle::EnemyConfigHandles;
use crate::util::entity_builder::EntityBuilderPlugin;

pub mod preload_animation_system;
pub mod preload_audio_system;
pub mod preload_enemy_system;
pub mod preload_item_system;
pub mod preload_mod_system;
pub mod preload_player_system;
pub mod preload_projectile_system;
pub mod preload_stage_spawn_system;
pub mod preload_texture_system;
mod preload_world_grid_config_system;

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
        app.add_plugin(EntityBuilderPlugin)
            .init_resource::<TextureHandles>()
            .init_resource::<EnemyConfigHandles>()
            .init_resource::<ItemConfigHandles>()
            .init_resource::<PlayerConfigHandles>()
            .init_resource::<ProjectileConfigHandles>()
            .init_resource::<SoundHandles>()
            .init_resource::<AtlasHandles>()
            .init_resource::<StageSpawnBehaviorHandle>();

        app.add_system(preload_texture_system.on_startup());

        app
            .add_system(preload_enemy_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_item_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_player_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_projectile_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_mod_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_audio_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_animation_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_stage_spawn_behavior_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets))
            .add_system(preload_world_grid_config_system.in_schedule(CoreSchedule::Startup).in_base_set(AssetSet::LoadAssets));
    }
}
