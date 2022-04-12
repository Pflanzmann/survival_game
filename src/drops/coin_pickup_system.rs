use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, With};
use bevy_kira_audio::Audio;

use crate::CoinCount;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::coin::Coin;

pub fn coin_pickup_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut coin_counter: ResMut<CoinCount>,
    item_query: Query<Entity, With<Coin>>,
) {
    for event in item_pickup_event.iter() {
        let item = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };
        audio.play(asset_server.load("audio/coin_pickup_sound.ogg"));
        coin_counter.number += 1;
        commands.entity(event.item_entity).despawn();
    }
}