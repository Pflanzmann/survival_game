use bevy::prelude::{Changed, Sprite};

use crate::models::unit_stats_components::UnitSize;
use crate::Query;

pub fn fit_sprite_to_size_system(
    mut target_query: Query<(&mut Sprite, &UnitSize), Changed<UnitSize>>
) {
    for (mut sprite, unit_site) in target_query.iter_mut() {
        sprite.custom_size = Option::Some(unit_site.collider_size);
    }
}