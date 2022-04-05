use bevy::asset::AssetServer;
use bevy::prelude::{EventReader, ResMut};
use bevy_kira_audio::Audio;

use crate::{CoinCount, Res};
use crate::models::events::item_collision_event::ItemCollisionEvent;

pub fn item_pickup_system(
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut coin_counter: ResMut<CoinCount>,
) {
    for _ in item_pickup_event.iter() {
        audio.play(asset_server.load("coin_pickup_sound.ogg"));
        coin_counter.number += 1;
    }
}