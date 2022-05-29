use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader, Query, Res, ResMut};

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::gold_storage::GoldStorage;
use crate::models::items::descriptor::gold_value::GoldValue;

pub fn coin_pickup_system(
    mut commands: Commands,
    item_query: Query<&GoldValue>,
    sound_handles: Res<SoundHandles>,
    mut sound_manager: ResMut<SoundManager>,
    mut item_pickup_events: EventReader<ItemCollisionEvent>,
    mut gold_storage_query: Query<&mut GoldStorage>,
) {
    for event in item_pickup_events.iter() {
        let gold_value = match item_query.get(event.source_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        let mut gold_storage = match gold_storage_query.get_mut(event.target_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        gold_storage.number += gold_value.gold_value;

        sound_manager.queue_sound(SoundHandleChannel::Pickup(sound_handles.coin_pickup_sound.clone()));

        commands.entity(event.source_entity).despawn_recursive();
    }
}
