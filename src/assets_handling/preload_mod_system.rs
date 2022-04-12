use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Error;
use std::ops::Deref;

use bevy::asset::AssetServer;
use bevy::prelude::{BuildChildren, Name};
use bevy::sprite::collide_aabb::Collision;
use serde::*;
use serde::de::DeserializeOwned;
use serde_json::{from_value, Value};

use crate::{Commands, Component, Entity, EntityCommands, ResMut};
use crate::assets_handling::configurations::mod_config::ModConfig;
use crate::models::attributes::damage::Damage;
use crate::models::attributes::health::Health;
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::modification_attributes::modification::Modification;
use crate::models::modification_components::{CurveShot, GrowShot, SplitShot};
use crate::models::modifications::mod_name::ModName;
use crate::models::modifications::mod_sprite_handler::ModSpriteHandler;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::modifications::tool_tip::{ModSpriteHandlerHelper, ToolTip};
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
            "ModName" => { entity.insert(get_component::<ModName>(object_data.clone())); }
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

pub fn spawn_mod_entity<T: Component>(
    commands: &mut Commands,
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
