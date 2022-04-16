use bevy::audio::Audio;
use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, With};


use crate::{AppStateTrigger, ToAppState};
use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::shop::Shop;

pub fn barrel_pickup_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    mut state_trigger: ResMut<AppStateTrigger>,
    sound_handles: Res<SoundHandles>,
    audio: Res<Audio>,

    item_query: Query<Entity, With<Shop>>,
) {
    for event in item_pickup_event.iter() {
        let _item = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        state_trigger.state_change_trigger = ToAppState::ToShop;

        audio.play(sound_handles.coin_pickup_sound.clone());

        commands.entity(event.item_entity).despawn();
    }
}