use bevy::prelude::*;
use noise::*;

use crate::assets_handling::preload_texture_system::TextureHandles;
use crate::models::configurations::world_grid_config::WorldGridConfig;
use crate::models::layerable::Layerable;
use crate::models::resources::world::background_tiles_resource::BackgroundTilesResource;
use crate::models::sprite_layer::SpriteLayer;
use crate::models::tile::Tile;

pub fn update_background_tiles_system(
    mut commands: Commands,
    texture_handle: Res<TextureHandles>,
    world_grid_config: Res<WorldGridConfig>,
    background_tiles: Res<BackgroundTilesResource>,
    mut tile_query: Query<(Entity, &Transform), With<Tile>>,
) {
    if !background_tiles.is_changed() {
        return;
    }

    let perlin = Fbm::<Perlin>::new(0);
    let exponent = Exponent::new(perlin.clone()).set_exponent(1.0);

    let mut tile_matrix: Vec<Vec<bool>> = vec![vec![false; world_grid_config.generated_background_radius_y]; world_grid_config.generated_background_radius_x];
    let current_grid_position = (background_tiles.current_origin / world_grid_config.tile_size).round();

    let grid_upper_bound_x = world_grid_config.generated_background_radius_x as f32;
    let grid_upper_bound_y = world_grid_config.generated_background_radius_y as f32;

    for (entity, transform) in tile_query.iter_mut() {
        let grid_grid_position = (transform.translation.truncate() / world_grid_config.tile_size).round();
        let local_position = grid_grid_position - current_grid_position;

        if local_position.x >= grid_upper_bound_x || local_position.y >= grid_upper_bound_y || local_position.x < 0.0 || local_position.y < 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        tile_matrix[local_position.x as usize][local_position.y as usize] = true;
    }

    for (x_index, x) in tile_matrix.iter().enumerate() {
        for (y_index, y) in x.iter().enumerate() {
            if *y {
                continue;
            }

            let local_x = (x_index as f32 * world_grid_config.tile_size) + background_tiles.current_origin.x;
            let local_y = (y_index as f32 * world_grid_config.tile_size) + background_tiles.current_origin.y;

            let noise_x = (x_index as f64 + current_grid_position.x as f64) / world_grid_config.noise_scale;
            let noise_y = (y_index as f64 + current_grid_position.y as f64) / world_grid_config.noise_scale;

            let noise_value = exponent.get([noise_x, noise_y]);
            let color_value = ((noise_value * 0.5 + 0.80).clamp(0.0, 1.0) * 255.0) as u8;

            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_handle.background_tile.clone(),
                transform: Transform::from_xyz(local_x, local_y, local_y),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(world_grid_config.tile_size, world_grid_config.tile_size)),
                    index: 4,
                    color: Color::rgb_u8(color_value, color_value, color_value),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert((
                    Tile,
                    Layerable::new(SpriteLayer::Background.get_layer_z()),
                    Name::new(format!("Tile [{:?},{:?}]", x_index, y_index))
                ));
        }
    }
}
