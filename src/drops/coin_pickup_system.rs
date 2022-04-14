use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, With};
use bevy_kira_audio::Audio;

use crate::GoldWallet;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::coin::Coin;
use crate::models::items::descriptor::gold_value::GoldValue;

pub fn coin_pickup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut gold_wallet: ResMut<GoldWallet>,
    mut item_pickup_events: EventReader<ItemCollisionEvent>,
    item_query: Query<&GoldValue>,
) {
    for event in item_pickup_events.iter() {
        let gold_value = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        audio.play(asset_server.load("audio/coin_pickup_sound.ogg"));

        gold_wallet.number += gold_value.gold_value;

        commands.entity(event.item_entity).despawn();
    }
}
