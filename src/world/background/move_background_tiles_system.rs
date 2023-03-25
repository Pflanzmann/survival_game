use bevy::prelude::{Query, ResMut, Transform, With, Without};

use crate::models::player::Player;
use crate::models::resources::world::background_tiles_resource::BackgroundTilesResource;
use crate::models::world::background_parent::BackgroundParent;

pub fn move_background_tiles_system(
    mut background_tiles: ResMut<BackgroundTilesResource>,
    player_query: Query<&Transform, (With<Player>, Without<BackgroundParent>)>,
    mut tile_parent_query: Query<&mut Transform, With<BackgroundParent>>,
) {
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let grid_position = (player_position / (1024.0 * 3.0)).round();

    if background_tiles.current_origin.distance(grid_position.truncate()) <= 0.9 {
        return;
    }

    background_tiles.current_origin = grid_position.truncate() * (1024.0 * 3.0);

    for mut tile in tile_parent_query.iter_mut() {
        tile.translation = background_tiles.current_origin.extend(0.0)
    }
}