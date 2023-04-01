use bevy::prelude::*;

use crate::models::configurations::world_grid_config::WorldGridConfig;
use crate::models::player::Player;
use crate::models::resources::world::background_tiles_resource::BackgroundTilesResource;

pub fn move_background_tiles_system(
    world_grid_config: Res<WorldGridConfig>,
    mut background_tiles: ResMut<BackgroundTilesResource>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_position = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let offset = Vec2::new(
        world_grid_config.generated_background_radius_x as f32 / 2.0,
        world_grid_config.generated_background_radius_y as f32 / 2.0,
    );

    let current_grid_position = (player_position / world_grid_config.tile_size).round();
    let unreduced_position = (current_grid_position.truncate() - offset) * world_grid_config.tile_size;

    if background_tiles.current_origin.distance(unreduced_position) <= world_grid_config.background_step_distance {
        return;
    }

    background_tiles.current_origin = unreduced_position;
}