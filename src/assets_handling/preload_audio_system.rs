extern crate core;

use bevy::audio::AudioSink;
use bevy::prelude::{Assets, AssetServer, Audio, Handle, Res, ResMut};


#[derive(Default)]
pub struct SoundHandles {
    pub coin_pickup_sound: Handle<AudioSink>,
}

pub fn preload_audio_system(
    mut sound_handles: ResMut<SoundHandles>,
    asset_server: Res<AssetServer>,
    audio_sinks: Res<Assets<AudioSink>>,
    audio: Res<Audio>,
) {
    let music = asset_server.load("coin_pickup_sound.ogg");
    let sink_handle = audio_sinks.get_handle(audio.play(music));
    sound_handles.coin_pickup_sound = sink_handle;
}