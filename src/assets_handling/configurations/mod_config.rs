use bevy::prelude::Component;
use serde::{Deserialize};

#[derive(Default, Deserialize, Debug)]
pub struct ModConfig<T: Component> {
    pub mod_name: String,
    pub tooltip: String,
    pub sprite_path: String,
    pub component: T,
}