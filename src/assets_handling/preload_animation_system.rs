use bevy::asset::{Assets, Handle};
use bevy::prelude::{AssetServer, Res, ResMut, TextureAtlas, Vec2};

#[derive(Default)]
pub struct AtlasHandles {
    pub player_atlas: Handle<TextureAtlas>,
    pub explosion_atlas: Handle<TextureAtlas>,
}

pub fn preload_animation_system(
    asset_server: Res<AssetServer>,
    mut atlas_handles: ResMut<AtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    atlas_handles.player_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/hero1.png"), Vec2::new(16.0, 16.0), 4, 6));
    atlas_handles.explosion_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/explosion.png"), Vec2::new(192.0, 192.0), 5, 6));
}