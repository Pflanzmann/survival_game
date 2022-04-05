use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image, Res, ResMut};

use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;
use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_player_system::PlayerConfigHandles;

#[derive(Default)]
pub struct TextureHandles {
    pub basic_drop_asset_handler: Handle<Image>,
    pub player_sprite: Handle<Image>,
    pub enemy_goblin: Handle<Image>,
    pub bullet_fireball: Handle<Image>,
    pub background_tile: Handle<Image>,
}

pub fn preload_texture_system(
    asset_server: Res<AssetServer>,
    enemy_handle: Res<EnemyConfigHandles>,
    item_handles: Res<ItemConfigHandles>,
    player_handles: Res<PlayerConfigHandles>,
    mut texture_handles: ResMut<TextureHandles>,
) {
    texture_handles.basic_drop_asset_handler = asset_server.load(&item_handles.coin.sprite_path);
    texture_handles.player_sprite = asset_server.load(&player_handles.player_one.sprite_path);
    texture_handles.enemy_goblin = asset_server.load(&enemy_handle.goblin.sprite_path.clone());
    texture_handles.bullet_fireball = asset_server.load("sprites/Bullet.png");
    texture_handles.background_tile = asset_server.load("sprites/full_grass.png");
}