use bevy::prelude::Changed;

use crate::{MoveDirection, Query, Sprite};

pub fn sprite_direction_system(
    mut sprite_query: Query<(&MoveDirection, &mut Sprite), Changed<MoveDirection>>,
) {
    for (direction, mut sprite) in sprite_query.iter_mut() {
        if direction.direction.x < 0.0 {
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