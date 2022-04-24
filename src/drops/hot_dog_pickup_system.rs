use bevy::asset::AssetServer;
use bevy::prelude::{Commands, EventReader, Query, Res, With};
use bevy_kira_audio::Audio;
use crate::assets_handling::preload_audio_system::SoundHandles;
//use crate::assets_handling::preload_audio_system::SoundHandles;

use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::descriptor::heal::Heal;
use crate::models::player::Player;
use crate::models::unit_attributes::health::Health;

pub fn hot_dog_pickup_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    sound_handles: Res<SoundHandles>,
    audio: Res<Audio>,


    mut player_query: Query<&mut Health, With<Player>>,
    item_query: Query<&Heal>,
) {
    for event in item_pickup_event.iter() {
        let item_heal = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        let mut player_health = match player_query.get_mut(event.player_entity) {
            Ok(value) => value,
            Err(_) => continue
        };

        player_health.heal(item_heal.amount);

        audio.play(sound_handles.coin_pickup_sound.clone());

        commands.entity(event.item_entity).despawn();
    }
}