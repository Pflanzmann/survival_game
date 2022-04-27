use bevy::prelude::{Commands, EventReader, Query, Res, ResMut};

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::GoldWallet;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::descriptor::gold_value::GoldValue;

pub fn coin_pickup_system(
    mut commands: Commands,
    mut gold_wallet: ResMut<GoldWallet>,
    mut item_pickup_events: EventReader<ItemCollisionEvent>,
    item_query: Query<&GoldValue>,
    sound_handles: Res<SoundHandles>,
    mut sound_manager: ResMut<SoundManager>
) {
    for event in item_pickup_events.iter() {
        let gold_value = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        gold_wallet.number += gold_value.gold_value;

        sound_manager.queue_sound(SoundHandleChannel::Pickup(sound_handles.coin_pickup_sound.clone()));

        commands.entity(event.item_entity).despawn();
    }
}
