use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::prelude::{BuildChildren, Name};
use serde::de::DeserializeOwned;

use crate::{Commands, Entity, EntityCommands, ResMut};
use crate::models::attributes::damage::Damage;
use crate::models::attributes::health::Health;
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::modification_attributes::modification::Modification;
use crate::models::modification_components::{CurveShot, GrowShot, SplitShot};
use crate::models::unit_modifications::descriptors::mod_name::ModName;
use crate::models::unit_modifications::descriptors::mod_sprite_handler::{ModSpriteHandler, ModSpriteHandlerHelper};
use crate::models::unit_modifications::descriptors::tool_tip::ToolTip;
use crate::models::unit_modifications::sprinting::Sprinting;
use crate::util::read_file_to_string::read_file_to_string;

pub fn preload_mod_system(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    let mut children: Vec<Entity> = Vec::new();

    let mut child = commands.spawn();
    children.push(spawn_entity(&mut child, &mut asset_server, "configurations/mod_configurations/grow_shot_config.json"));

    let mut child = commands.spawn();
    children.push(spawn_entity(&mut child, &mut asset_server, "configurations/mod_configurations/curve_shot_config.json"));

    let mut child = commands.spawn();
    children.push(spawn_entity(&mut child, &mut asset_server, "configurations/mod_configurations/split_shot_config.json"));

    let mut child = commands.spawn();
    children.push(spawn_entity(&mut child, &mut asset_server, "configurations/mod_configurations/sprinting_config.json"));

    let parent = commands.spawn().insert(Name::new("mod_entity_parent")).id();
    commands.entity(parent).push_children(&*children);
}

pub fn spawn_entity(
    entity: &mut EntityCommands,
    asset_server: &mut ResMut<AssetServer>,
    config_path: &str,
) -> Entity {
    let my_string = read_file_to_string(config_path);
    let component_data_map: HashMap<String, serde_json::Value> = serde_json::from_str(&my_string).unwrap();

    for (component_key, object_data) in component_data_map.iter() {
        match component_key.as_str() {
            "ModName" => {
                let mod_name = get_component::<ModName>(object_data.clone());
                entity.insert(Name::new(mod_name.mod_name));
                entity.insert(get_component::<ModName>(object_data.clone()));
            }
            "ModSpriteHandler" => {
                let helper = get_component::<ModSpriteHandlerHelper>(object_data.clone());
                entity.insert(ModSpriteHandler { sprite: asset_server.load(&helper.sprite) });
            }
            "ToolTip" => { entity.insert(get_component::<ToolTip>(object_data.clone())); }

            "Health" => { entity.insert(get_component::<Health>(object_data.clone())); }
            "Damage" => { entity.insert(get_component::<Damage>(object_data.clone())); }
            "MoveSpeed" => { entity.insert(get_component::<MoveSpeed>(object_data.clone())); }

            "Modification" => { entity.insert(get_component::<Modification>(object_data.clone())); }
            "CurveShot" => { entity.insert(get_component::<CurveShot>(object_data.clone())); }
            "GrowShot" => { entity.insert(get_component::<GrowShot>(object_data.clone())); }
            "SplitShot" => { entity.insert(get_component::<SplitShot>(object_data.clone())); }
            "Sprinting" => { entity.insert(get_component::<Sprinting>(object_data.clone())); }
            _ => {}
        }
    }

    entity.id()
}

pub fn get_component<T: DeserializeOwned>(
    value: serde_json::Value
) -> T {
    serde_json::from_value(value).expect("Not well formatted string: {:#?}")
}
