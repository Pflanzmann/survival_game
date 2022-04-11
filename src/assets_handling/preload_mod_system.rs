/*#[derive(Default)]
pub struct ModConfigHandles {
    pub coin: CoinConfig,
    pub hot_dog: HotDogConfig,
    pub kiste: KisteConfig
}*/


use bevy::prelude::{BuildChildren, Name};

use crate::{Commands, Component, Entity};
use crate::assets_handling::configurations::mod_config::ModConfig;
use crate::models::modification_attributes::{Modification, ModName, SpritePath, ToolTip};
use crate::models::modification_components::GrowShot;
use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_mod_system(
    mut commands: Commands,
) {
    let parent = commands.spawn()
        .insert(Name::new("mod_entity_parent"))
        .id();

    let my_string = read_file_to_string("configurations/mod_configurations/grow_shot_config.json");
    let mod_config: ModConfig<GrowShot> = serde_json::from_str(&my_string).expect("JSON was not well-formatted || GROW SHOT MOD");
    spawn_mod_entity(commands, mod_config, parent);


}

pub fn spawn_mod_entity<T: Component>(
    mut commands: Commands,
    mod_config: ModConfig<T>,
    parent: Entity,
) {
    let child = commands.spawn()
        .insert(Name::new(mod_config.mod_name.clone()))
        .insert(ModName { mod_name: mod_config.mod_name })
        .insert(ToolTip { tooltip: mod_config.tooltip })
        .insert(SpritePath { sprite_path: mod_config.sprite_path })
        .insert(mod_config.component)
        .insert(Modification)
        .id();

    commands.entity(parent).add_child(child);
}