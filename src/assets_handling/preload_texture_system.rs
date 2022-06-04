use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image, Res, ResMut};

use crate::assets_handling::preload_projectile_system::ProjectileConfigHandles;
use crate::assets_handling::preload_item_system::ItemConfigHandles;

#[derive(Default)]
pub struct TextureHandles {
    pub coin_sprite: Handle<Image>,
    pub hot_dog_sprite: Handle<Image>,
    pub barrel_sprite: Handle<Image>,
    pub projectile_fireball: Handle<Image>,
    pub background_tile: Handle<Image>,
    pub sold_button: Handle<Image>,
    pub turret_unit: Handle<Image>,
    pub slime_unit: Handle<Image>,
    pub death_ball_unit: Handle<Image>,
    pub psy_rock_unit: Handle<Image>,
    pub radiation: Handle<Image>,
    pub shield: Handle<Image>,
    pub sword: Handle<Image>,
}

pub fn preload_texture_system(
    asset_server: Res<AssetServer>,
    item_handles: Res<ItemConfigHandles>,
    projectile_handles: Res<ProjectileConfigHandles>,
    mut texture_handles: ResMut<TextureHandles>,
) {
    texture_handles.coin_sprite = asset_server.load(&item_handles.coin.sprite_path);
    texture_handles.hot_dog_sprite = asset_server.load(&item_handles.hot_dog.sprite_path);
    texture_handles.barrel_sprite = asset_server.load(&item_handles.barrel.sprite_path);

    texture_handles.sold_button = asset_server.load("sprites/sold_sign.png");

    texture_handles.turret_unit = asset_server.load("sprites/unit_sprites/turret_unit.png");
    texture_handles.slime_unit = asset_server.load("sprites/unit_sprites/slime.png");
    texture_handles.death_ball_unit = asset_server.load("sprites/unit_sprites/death_ball.png");
    texture_handles.psy_rock_unit = asset_server.load("sprites/unit_sprites/psy_rock.png");
    texture_handles.radiation = asset_server.load("sprites/unit_sprites/radiation.png");
    texture_handles.shield = asset_server.load("sprites/unit_sprites/shield.png");
    texture_handles.sword = asset_server.load("sprites/sword.png");

    texture_handles.projectile_fireball = asset_server.load(&projectile_handles.basic_projectile.sprite_path);
    texture_handles.background_tile = asset_server.load("sprites/full_grass.png");
}