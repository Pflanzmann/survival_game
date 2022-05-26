use bevy::prelude::{Res, ResMut};
use bevy_kira_audio::Audio;

use crate::assets_handling::preload_audio_system::SoundHandles;
use crate::audio::sound_manager::SoundManager;
use crate::models::audio::sound_channel::SoundChannel;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;

pub fn play_background_music_system(
    sound_handles: ResMut<SoundHandles>,
    audio: Res<Audio>,
    mut sound_manager: ResMut<SoundManager>,
) {
    sound_manager.set_volume_for_channel(0.1, SoundChannel::Background, audio);
    sound_manager.queue_sound(SoundHandleChannel::Background(sound_handles.background_music.clone()));
}