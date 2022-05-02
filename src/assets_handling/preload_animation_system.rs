use bevy::asset::{Assets, Handle};
use bevy::prelude::{AssetServer, Res, ResMut, TextureAtlas, Vec2};

#[derive(Default)]
pub struct AtlasHandles {
    pub player_idle_atlas: Handle<TextureAtlas>,
}

pub fn preload_animation_system(
    asset_server: Res<AssetServer>,
    mut atlas_handles: ResMut<AtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    atlas_handles.player_idle_atlas = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprite_sheets/ranger_idle.png"), Vec2::new(32.0, 32.0), 2, 1));
}