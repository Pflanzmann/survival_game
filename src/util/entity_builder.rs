use std::any::type_name;
use std::collections::HashMap;

use bevy::asset::AssetServer;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Component, Entity, Name, Plugin, ResMut};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::App;
use crate::models::modifications::affects::affect_move_speed::AffectMoveSpeed;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::mod_sprite_path::ModSpritePath;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::modifications::turret::Turret;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::util::read_file_to_string::read_file_to_string;

pub struct EntityBuilderPlugin;

impl Plugin for EntityBuilderPlugin {
    fn build(&self, app: &mut App) {
        let mut entity_builder = EntityBuilder::new();

        entity_builder.register_component::<ModName>();
        entity_builder.register_component::<ToolTip>();
        entity_builder.register_component::<ModSpritePath>();

        entity_builder.register_component::<AffectMoveSpeed>();
        entity_builder.register_component::<Health>();
        entity_builder.register_component::<Damage>();
        entity_builder.register_component::<MoveSpeed>();

        entity_builder.register_component::<Modification>();
        entity_builder.register_component::<CurveShot>();
        entity_builder.register_component::<GrowShot>();
        entity_builder.register_component::<SplitShot>();

        entity_builder.register_component::<Sprinting>();
        entity_builder.register_component::<Turret>();

        app.insert_non_send_resource::<EntityBuilder>(entity_builder);
    }
}

#[derive(Default)]
pub struct EntityBuilder {
    map: HashMap<String, Box<dyn Fn(&mut EntityCommands, &Value)>>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder { map: HashMap::new() }
    }

    pub fn spawn_entity(
        &self,
        commands: &mut Commands,
        config_path: &str,
    ) -> Entity {
        let mut entity = commands.spawn();

        let my_string = read_file_to_string(config_path);
        let component_data_map: HashMap<String, serde_json::Value> = serde_json::from_str(&my_string).unwrap();

        for (component_key, object_data) in component_data_map.iter() {
            match self.map.get(component_key) {
                None => { panic!("Type is not registered") }
                Some(a) => a(&mut entity, &object_data),
            };
        }

        return entity.id();
    }

    pub fn register_component<T: DeserializeOwned + Component>(&mut self) {
        self.map.insert(
            String::from(type_name::<T>().split("::").last().unwrap()),
            Box::new(deserialize_component::<T>),
        );
    }

    pub fn register_component_custom_handler
    <T: DeserializeOwned + Component>(
        &mut self,
        function: Box<dyn Fn(&T)>,
    ) {
        self.map.insert(
            String::from(type_name::<T>().split("::").last().unwrap()),
            Box::new(deserialize_component::<T>),
        );
    }
}

fn deserialize_component<T: DeserializeOwned + Component>(
    entity_commands: &mut EntityCommands,
    object_data: &Value,
) {
    entity_commands.insert(get_component::<T>(object_data.clone()));
}

fn get_component<T: DeserializeOwned + Component>(
    value: serde_json::Value
) -> T {
    serde_json::from_value(value).expect("Not well formatted string: {:#?}")
}

