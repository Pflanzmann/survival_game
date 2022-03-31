use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Commands, Image, Res, ResMut};

#[derive(Default)]
pub struct TextureHandles{
    pub basic_drop_asset_handler: Handle<Image>,
    pub player_sprite: Handle<Image>
}

pub fn preload_texture_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_handles : ResMut<TextureHandles>
){
    texture_handles.basic_drop_asset_handler = asset_server.load("basic_drop.png");
    texture_handles.player_sprite = asset_server.load("NickelMan.png");
}