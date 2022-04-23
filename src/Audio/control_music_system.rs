use bevy::audio::AudioSink;
use bevy::prelude::{Assets, Input, KeyCode, Res};
use crate::assets_handling::preload_audio_system::SinkSoundHandles;

pub fn control_music_system (
    input: Res<Input<KeyCode>>,
    res_audio_sinks : Res<SinkSoundHandles>,
    audio_sinks: Res<Assets<AudioSink>>
){
    if input.pressed(KeyCode::M) {
        if let Some(sink) = audio_sinks.get(res_audio_sinks.background_music.clone()){
            sink.pause();
        }
    }

    if input.pressed(KeyCode::N) {
        if let Some(sink) = audio_sinks.get(res_audio_sinks.background_music.clone()){
            sink.play();
        }
    }

    if input.pressed(KeyCode::J) {
        if let Some(sink) = audio_sinks.get(res_audio_sinks.background_music.clone()){
            sink.set_volume(sink.volume() - 0.1);
        }
    }

    if input.pressed(KeyCode::K) {
        if let Some(sink) = audio_sinks.get(res_audio_sinks.background_music.clone()){
            sink.set_volume(sink.volume() + 0.1);
        }
    }
}