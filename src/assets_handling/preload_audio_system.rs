extern crate core;

use bevy::asset::{AssetServer, Handle};
use bevy::audio::{AudioSource};
use bevy::prelude::{Res, ResMut};


#[derive(Default)]
pub struct SoundHandles {
    pub coin_pickup_sound: Handle<AudioSource>,
}

pub fn preload_audio_system(
    mut sound_handles: ResMut<SoundHandles>,
    asset_server: Res<AssetServer>,
) {
    sound_handles.coin_pickup_sound = asset_server.load("audio/coin_pickup_sound.ogg");
}