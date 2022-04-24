use bevy::asset::AssetServer;
use bevy::prelude::{Commands, EventReader, Query, Res, ResMut};
use bevy_kira_audio::Audio;
use crate::assets_handling::preload_audio_system::SoundHandles;
//use crate::assets_handling::preload_audio_system::SoundHandles;

use crate::GoldWallet;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::descriptor::gold_value::GoldValue;

pub fn coin_pickup_system(
    mut commands: Commands,
    mut gold_wallet: ResMut<GoldWallet>,
    mut item_pickup_events: EventReader<ItemCollisionEvent>,
    item_query: Query<&GoldValue>,
    sound_handles: Res<SoundHandles>,

    audio: Res<Audio>,
) {
    for event in item_pickup_events.iter() {
        let gold_value = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        gold_wallet.number += gold_value.gold_value;

        audio.play(sound_handles.coin_pickup_sound.clone());

        commands.entity(event.item_entity).despawn();
    }
}
