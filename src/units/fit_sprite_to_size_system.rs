use bevy::prelude::{Changed, Query, Sprite};

use crate::models::unit_size::UnitSize;

pub fn fit_sprite_to_size_system(
    mut target_query: Query<(&mut Sprite, &UnitSize), Changed<UnitSize>>
) {
    for (mut sprite, unit_site) in target_query.iter_mut() {
        sprite.custom_size = Option::Some(unit_site.collider_size);
    }
}