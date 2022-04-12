use bevy::prelude::{AssetServer, Commands, Entity, EventReader, Query, Res, ResMut, With};
use bevy_kira_audio::Audio;
use crate::{AppStateTrigger, CoinCount, ToAppState};
use crate::models::events::item_collision_event::ItemCollisionEvent;
use crate::models::items::coin::Coin;
use crate::models::items::shop::Shop;

pub fn kiste_pickup_system(
    mut commands : Commands,
    mut item_pickup_event: EventReader<ItemCollisionEvent>,
    mut state_trigger: ResMut<AppStateTrigger>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,

    item_query : Query<Entity, With<Shop>>
) {
    for event in item_pickup_event.iter() {
        let item = match item_query.get(event.item_entity) {
            Ok(value) => value,
            Err(_) => continue
        };
        audio.play(asset_server.load("audio/coin_pickup_sound.ogg"));
        state_trigger.State_Change_Trigger = ToAppState::ToShop;
        commands.entity(event.item_entity).despawn();
    }
}