use bevy::asset::{Assets, Handle};
use bevy::prelude::{AssetServer, Res, ResMut, TextureAtlas, Vec2};

#[derive(Default)]
pub struct AtlasHandles {
    pub player_idle_atlas: Handle<TextureAtlas>,
    pub player_atlas: Handle<TextureAtlas>,
    pub goblin_atlas: Handle<TextureAtlas>,
}

pub fn preload_animation_system(
    asset_server: Res<AssetServer>,
    mut atlas_handles: ResMut<AtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    atlas_handles.player_idle_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/ranger_idle.png"), Vec2::new(32.0, 32.0), 2, 1));
    atlas_handles.player_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/hero1.png"), Vec2::new(16.0, 16.0), 4, 6));
    atlas_handles.goblin_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/goblin.png"), Vec2::new(64.0, 80.0), 4, 4));
}