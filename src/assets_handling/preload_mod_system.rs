use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{BuildChildren, Commands, Entity, Name, ResMut};
use serde::de::DeserializeOwned;

use crate::models::modifications::affects::affect_move_speed::AffectMoveSpeed;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::mod_sprite_handler::{ModSpriteHandler, ModSpriteHandlerHelper};
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
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

    let parent = commands.spawn().insert(Name::new("Mod Entities")).id();
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

            "AffectMoveSpeed" => { entity.insert(get_component::<AffectMoveSpeed>(object_data.clone())); }

            "Health" => { entity.insert(get_component::<Health>(object_data.clone())); }
            "Damage" => { entity.insert(get_component::<Damage>(object_data.clone())); }
            "MoveSpeed" => { entity.insert(get_component::<MoveSpeed>(object_data.clone())); }

            "Modification" => { entity.insert(get_component::<Modification>(object_data.clone())); }
            "CurveShot" => { entity.insert(get_component::<CurveShot>(object_data.clone())); }
            "GrowShot" => { entity.insert(get_component::<GrowShot>(object_data.clone())); }
            "SplitShot" => { entity.insert(get_component::<SplitShot>(object_data.clone())); }
            "Sprinting" => { entity.insert(get_component::<Sprinting>(object_data.clone())); }

            not_found_key => panic!("Map key not handled [{}]", not_found_key),
        }
    }

    entity.id()
}

pub fn get_component<T: DeserializeOwned>(
    value: serde_json::Value
) -> T {
    serde_json::from_value(value).expect("Not well formatted string: {:#?}")
}
