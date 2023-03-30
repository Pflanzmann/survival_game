use std::cmp::min;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use rand::Rng;

use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::mod_sprite_path::SpriteHandle;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::player::Player;
use crate::models::resources::ui_states::shop_state::{ShopMod, ShopState};

pub fn update_shop_system(
    mut egui_context: EguiContexts,
    mut shop_state: ResMut<ShopState>,
    player_mod_register_query: Query<&ModRegister, With<Player>>,
    mod_query: Query<(Entity, &SpriteHandle, &ToolTip, &ModName)>,
) {
    let customer_mod_register = match player_mod_register_query.get_single() {
        Ok(register) => register,
        Err(_) => return,
    };

    shop_state.chosen_mod.clear();

    let requested_slot_count = 3;

    let mut valid_mods: Vec<Entity> = Vec::new();
    for (entity, _, _, _) in mod_query.iter() {
        if !customer_mod_register.register.contains(&entity) {
            valid_mods.push(entity);
        }
    }

    let valid_mods_len = valid_mods.len() as i32;
    let mut chosen_mod_indexes: Vec<i32> = Vec::new();
    let mut chosen_mod_entities: Vec<Entity> = Vec::new();

    for _ in 0..min(valid_mods.len(), requested_slot_count) {
        let mut rnd_number: i32 = rand::thread_rng().gen_range(0..valid_mods_len);
        while chosen_mod_indexes.contains(&rnd_number) {
            rnd_number = rand::thread_rng().gen_range(0..valid_mods_len);
        }
        chosen_mod_indexes.push(rnd_number);
        if let Some(mod_entity) = valid_mods.get(rnd_number as usize) {
            chosen_mod_entities.push(*mod_entity);
        }
    }

    for mod_entity in chosen_mod_entities.iter() {
        let (mod_entity, mod_sprite_path, tooltip, mod_name) = match mod_query.get(*mod_entity) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let texture_id = egui_context.add_image(mod_sprite_path.handle.clone().into());

        shop_state.chosen_mod.push(ShopMod::new(
            mod_entity,
            mod_name.mod_name.clone(),
            tooltip.tooltip.clone(),
            texture_id,
        ))
    }
}