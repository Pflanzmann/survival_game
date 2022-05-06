use bevy::prelude::{Changed, Query, Sprite, TextureAtlasSprite, With};

use crate::models::move_direction::MoveDirection;
use crate::models::sprite_flip::SpriteFlip;

pub fn sprite_flip_system(
    mut sprite_query: Query<(&MoveDirection, &mut Sprite), (Changed<MoveDirection>, With<SpriteFlip>)>,
) {
    for (direction, mut sprite) in sprite_query.iter_mut() {
        if direction.direction.x < 0.0 {
            if !sprite.flip_x {
                sprite.flip_x = true;
            }
        } else if sprite.flip_x {
            sprite.flip_x = false;
        }
    }
}

pub fn sprite_atlas_flip_system(
    mut sprite_query: Query<(&MoveDirection, &mut TextureAtlasSprite), (Changed<MoveDirection>, With<SpriteFlip>)>,
) {
    for (direction, mut sprite) in sprite_query.iter_mut() {
        if direction.direction.x < 0.0 {
            if !sprite.flip_x {
                sprite.flip_x = true;
            }
        } else if sprite.flip_x {
            sprite.flip_x = false;
        }
    }
}