use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Commands, Image, Res, ResMut};

#[derive(Default)]
pub struct TextureHandles{
    pub basic_drop_asset_handler: Handle<Image>,
    pub player_sprite: Handle<Image>,
    pub enemy_rock: Handle<Image>,
    pub bullet_fireball: Handle<Image>,
    pub background_tile: Handle<Image>
}

pub fn preload_texture_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_handles : ResMut<TextureHandles>
){
    texture_handles.basic_drop_asset_handler = asset_server.load("basic_drop.png");
    texture_handles.player_sprite = asset_server.load("NickelMan.png");
    texture_handles.enemy_rock = asset_server.load("Rock01.png");
    texture_handles.bullet_fireball = asset_server.load("Bullet.png");
    texture_handles.background_tile = asset_server.load("full_grass.png");
}