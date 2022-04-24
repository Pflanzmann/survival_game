use bevy::asset::Assets;
use bevy::prelude::{Res, ResMut};
use bevy_kira_audio::Audio;
use crate::assets_handling::preload_audio_system::{SoundHandles};

pub fn play_background_music_system(
    mut sound_handles: ResMut<SoundHandles>,
    audio: Res<Audio>,
){
    audio.play_looped(sound_handles.background_music.clone());
}