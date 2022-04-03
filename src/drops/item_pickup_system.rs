use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Commands, EventReader};
use bevy_kira_audio::Audio;
use crate::components::event_components::{BulletStoppedEvent, ItemPickupEvent};
use crate::Res;

pub fn item_pickup_system(
    mut commands: Commands,
    mut item_pickup_event: EventReader<ItemPickupEvent>,
    asset_server : Res<AssetServer>,
    audio : Res<Audio>
){
    for event in item_pickup_event.iter() {
        //sound abspielen
        //counter hochzählen
        //asset_server.load("coin_pickup_sound.ogg");
        audio.play(asset_server.load("coin_pickup_sound.ogg"));
        println!("something event fires");
    }
}