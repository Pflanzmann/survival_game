use bevy::prelude::*;

use crate::models::player::Player;
use crate::models::resources::world::background_tiles_resource::BackgroundTilesResource;

pub fn move_background_tiles_system(
    mut background_tiles: ResMut<BackgroundTilesResource>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let offset = Vec2::new(15.0, 15.0);

    let current_grid_position = (player_position / 256.0).round();
    let unreduced_position = (current_grid_position.truncate() - offset) * 256.0;

    if background_tiles.current_origin.distance(unreduced_position) <= 2700.0 {
        return;
    }

    background_tiles.current_origin = unreduced_position;
}