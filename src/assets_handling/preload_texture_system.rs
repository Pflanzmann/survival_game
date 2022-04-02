use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Commands, Image, Res, ResMut};
use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;

#[derive(Default)]
pub struct TextureHandles {
    pub basic_drop_asset_handler: Handle<Image>,
    pub player_sprite: Handle<Image>,
    pub enemy_rock: Handle<Image>,
    pub bullet_fireball: Handle<Image>,
    pub background_tile: Handle<Image>,
}

pub fn preload_texture_system(
    asset_server: Res<AssetServer>,
    enemy_handle: Res<EnemyConfigHandles>,
    mut texture_handles: ResMut<TextureHandles>,
) {
    texture_handles.basic_drop_asset_handler = asset_server.load("basic_drop.png");
    texture_handles.player_sprite = asset_server.load("NickelMan.png");
    texture_handles.enemy_rock = asset_server.load(&enemy_handle.rock.sprite_path.clone());
    texture_handles.bullet_fireball = asset_server.load("Bullet.png");
    texture_handles.background_tile = asset_server.load("full_grass.png");
}