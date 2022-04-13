use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image, Res, ResMut};
use crate::assets_handling::preload_bullet_system::BulletConfigHandles;

use crate::assets_handling::preload_enemy_system::EnemyConfigHandles;
use crate::assets_handling::preload_item_system::ItemConfigHandles;
use crate::assets_handling::preload_player_system::PlayerConfigHandles;

#[derive(Default)]
pub struct TextureHandles {
    pub coin_sprite: Handle<Image>,
    pub hot_dog_sprite: Handle<Image>,
    pub barrel_sprite: Handle<Image>,
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
    bullet_handles: Res<BulletConfigHandles>,

    mut texture_handles: ResMut<TextureHandles>,
) {
    texture_handles.coin_sprite = asset_server.load(&item_handles.coin.sprite_path);
    texture_handles.hot_dog_sprite = asset_server.load(&item_handles.hot_dog.sprite_path);
    texture_handles.barrel_sprite = asset_server.load(&item_handles.barrel.sprite_path);

    texture_handles.player_sprite = asset_server.load(&player_handles.player_one.sprite_path);
    texture_handles.enemy_goblin = asset_server.load(&enemy_handle.goblin.sprite_path);
    texture_handles.bullet_fireball = asset_server.load(&bullet_handles.basic_bullet.sprite_path);
    texture_handles.background_tile = asset_server.load("sprites/full_grass.png");
}