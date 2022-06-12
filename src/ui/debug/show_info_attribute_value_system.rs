use std::any::type_name;

use bevy::prelude::*;

use crate::models::player::Player;
use crate::models::resources::ui_states::info_window_state::InfoWindowState;
use crate::models::unit_attributes::attribute::Attribute;

pub fn show_info_attribute_value_system<
    T: Component + Attribute,
>(
    attribute_query: Query<&T, With<Player>>,
    mut state: ResMut<InfoWindowState>,
) {
    for attribute in attribute_query.iter() {
        let text = attribute.get_total_amount().to_string();

        state.infos.insert(type_name::<T>().split("::").last().unwrap().to_string(), text);
    }
}