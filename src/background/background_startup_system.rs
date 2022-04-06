use bevy::prelude::{BuildChildren, Commands, GlobalTransform, Name, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2};

use crate::{SpriteLayer, TextureHandles};
use crate::models::tile::Tile;
use crate::resources::background_tiles_resource::{BackgroundTilesResource, TileData};

pub fn background_startup_system(
    mut commands: Commands,
    mut background_tiles: ResMut<BackgroundTilesResource>,
    texture_handles: Res<TextureHandles>,
) {
    let background = commands.spawn().insert(Name::new("background")).id();

    for x in 0..50 {
        for y in 0..50 {
            let child = commands.spawn_bundle(SpriteBundle {
                texture: texture_handles.background_tile.clone(),
                global_transform: GlobalTransform::from(Transform::from_xyz((x * 256) as f32, (y * 256) as f32, SpriteLayer::FloorLevel.get_layer_z())),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert(Tile)
                .id();

            commands.entity(background).add_child(child);

            background_tiles.tiles.push(TileData {
                entity: child,
                position: Vec2::new(x as f32, y as f32),
            })
        }
    }
}
