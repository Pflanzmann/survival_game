use bevy::asset::AssetServer;
use bevy::prelude::{BuildChildren, Name};

use crate::{Commands, Component, Entity, ResMut};
use crate::assets_handling::configurations::mod_config::ModConfig;
use crate::models::modification_attributes2::{Modification, ModName, ModSpriteHandler, ToolTip};
use crate::models::modification_components::{CurveShot, GrowShot, SplitShot};
use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_mod_system(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    let parent = commands.spawn()
        .insert(Name::new("mod_entity_parent"))
        .id();

    let my_string = read_file_to_string("configurations/mod_configurations/grow_shot_config.json");
    let mod_config: ModConfig<GrowShot> = serde_json::from_str(&my_string).expect("JSON was not well-formatted || GROW SHOT MOD");
    spawn_mod_entity(&mut commands, &mut asset_server, mod_config, parent);

    let my_string = read_file_to_string("configurations/mod_configurations/curve_shot_config.json");
    let mod_config: ModConfig<CurveShot> = serde_json::from_str(&my_string).expect("JSON was not well-formatted || CURVE SHOT MOD");
    spawn_mod_entity(&mut commands, &mut asset_server, mod_config, parent);

    let my_string = read_file_to_string("configurations/mod_configurations/split_shot_config.json");
    let mod_config: ModConfig<SplitShot> = serde_json::from_str(&my_string).expect("JSON was not well-formatted || SPLIT SHOT MOD");
    spawn_mod_entity(&mut commands, &mut asset_server, mod_config, parent);
}

pub fn spawn_mod_entity<T: Component>(
    mut commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    mod_config: ModConfig<T>,
    parent: Entity,
) {
    let child = commands.spawn()
        .insert(Name::new(mod_config.mod_name.clone()))
        .insert(ModName { mod_name: mod_config.mod_name })
        .insert(ToolTip { tooltip: mod_config.tooltip })
        .insert(ModSpriteHandler { sprite: asset_server.load(&mod_config.sprite_path) })
        .insert(mod_config.component)
        .insert(Modification)
        .id();

    commands.entity(parent).add_child(child);
}
