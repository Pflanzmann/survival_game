use bevy::prelude::{BuildChildren, Color, Commands, ComputedVisibility, GlobalTransform, Name, Res, ResMut, Sprite, SpriteBundle, Transform, Vec2, Visibility};

use crate::{SpriteLayer, TextureHandles};
use crate::models::layerable::Layerable;
use crate::models::resources::world::background_tiles_resource::{BackgroundTilesResource, TileData};
use crate::models::tile::Tile;
use crate::models::world::background_parent::BackgroundParent;

pub fn background_startup_system(
    mut commands: Commands,
    mut background_tiles: ResMut<BackgroundTilesResource>,
    texture_handles: Res<TextureHandles>,
) {
    let background = commands.spawn_empty()
        .insert(Name::new("BackgroundParent"))
        .insert((
            BackgroundParent,
            Transform::default(),
            Layerable::new(SpriteLayer::Background.get_layer_z()),
            GlobalTransform::default(),
            ComputedVisibility::default(),
            Visibility::Visible,
        ))
        .id();

    for x in -14..15 {
        for y in -14..15 {
            let sprite_color = if x % 2 == 0 && y % 2 == 0 {
                Color::rgb(0.9, 0.9, 0.9)
            } else {
                Color::rgb(1.0, 1.0, 1.0)
            };

            let child = commands.spawn(SpriteBundle {
                texture: texture_handles.background_tile.clone(),
                transform: Transform::from_xyz((x * 256) as f32, (y * 256) as f32, y as f32),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    color: sprite_color,
                    ..Default::default()
                },
                visibility: Visibility::Visible,
                ..Default::default()
            })
                .insert(Tile)
                .insert(Name::new("Tile"))
                .id();

            commands.entity(background).add_child(child);

            background_tiles.tiles.push(TileData {
                entity: child,
                position: Vec2::new(x as f32, y as f32),
            })
        }
    }
}
