use bevy::math::Vec2;
use bevy::prelude::{GlobalTransform, ResMut, Transform, Vec3, With, Without};

use crate::{Query, SpriteLayer};
use crate::models::player::Player;
use crate::models::tile::Tile;
use crate::resources::background_tiles_resource::BackgroundTilesResource;

pub fn move_background_tiles_system(
    mut background_tiles: ResMut<BackgroundTilesResource>,
    player_query: Query<(&Transform), (With<Player>, Without<Tile>)>,
    mut tiles_query: Query<&mut GlobalTransform, With<Tile>>,
) {
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let grid_position = (player_position / (2056.0 * 2.0)).round();

    if background_tiles.current_origin.distance(grid_position.truncate()) <= 0.9 {
        return;
    }

    background_tiles.current_origin = grid_position.truncate();

    let mut tile_vec: Vec<Vec3> = Vec::new();
    for x in -25..24 {
        for y in -25..24 {
            let tile_position = Vec3::new((x as f32) * 256.0, y as f32 * 256.0, SpriteLayer::BackGround.get_layer_z());
            tile_vec.push(tile_position)
        }
    }

    for mut tile in tiles_query.iter_mut() {
        tile.translation = match tile_vec.pop() {
            Some(pos) => {
                pos + (grid_position * (2056.0 * 2.0))
            }
            None => continue,
        }
    }
}