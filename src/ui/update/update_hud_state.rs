use bevy::prelude::{Changed, Query, ResMut, With};

use crate::models::mod_register::ModRegister;
use crate::models::modifications::descriptors::mod_sprite_path::SpriteHandle;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::player::Player;
use crate::models::resources::ui_states::hud_state::HudState;

pub fn update_hud_state(
    mut state: ResMut<HudState>,
    player_query: Query<&ModRegister, (With<Player>, Changed<ModRegister>)>,
    mod_query: Query<&SpriteHandle, With<Modification>>,
) {
    for mod_register in player_query.iter() {
        for registered_entity in mod_register.register.iter() {
            if state.entities.contains(registered_entity) {
                continue;
            }

            let added_mod_sprite_handle = match mod_query.get(*registered_entity) {
                Ok(sprite_handle) => sprite_handle,
                Err(_) => continue,
            };

            state.entities.push(*registered_entity);
            state.image_handles.insert(0, added_mod_sprite_handle.handle.clone());
        }
    }
}