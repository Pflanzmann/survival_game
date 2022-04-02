use bevy::prelude::Changed;

use crate::{FacingDirection, Query, Sprite};

pub fn sprite_direction_system(
    mut sprite_query: Query<(&FacingDirection, &mut Sprite), Changed<FacingDirection>>,
) {
    for (direction, mut sprite) in sprite_query.iter_mut() {
        if direction.facing_direction.x < 0.0 {
            if !sprite.flip_x {
                sprite.flip_x = true;
            }
        } else {
            if sprite.flip_x {
                sprite.flip_x = false;
            }
        }
    }
}