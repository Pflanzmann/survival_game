use bevy::asset::Assets;
use bevy::audio::{Audio, AudioSink, PlaybackSettings};
use bevy::prelude::{Res, ResMut};
use crate::assets_handling::preload_audio_system::{SinkSoundHandles, SoundHandles};

pub fn play_background_music_system(
    sound_handles: Res<SoundHandles>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    mut res_audio_sinks : ResMut<SinkSoundHandles>
){
    let sink_handle = audio_sinks.get_handle(audio.play_with_settings(sound_handles.background_music.clone(), PlaybackSettings::LOOP.with_volume(0.05)));
    res_audio_sinks.background_music = sink_handle;

}