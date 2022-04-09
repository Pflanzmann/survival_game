use bevy::asset::AssetServer;
use bevy::prelude::EventReader;
use bevy_kira_audio::Audio;

use crate::{Commands, Health, Player, Query, Res, With};
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::item_components::Heal;

pub fn hot_dog_pickup_system(
    mut commands : Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,

    mut player_query : Query<&mut Health, With<Player>>,
    item_query: Query<&Heal>
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
        audio.play(asset_server.load("audio/coin_pickup_sound.ogg"));
        commands.entity(event.item_entity).despawn();
    }
}