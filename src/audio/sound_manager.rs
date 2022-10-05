use bevy::prelude::{Res, ResMut};
use bevy_kira_audio::{Audio, AudioChannel, AudioControl, MainTrack};

use crate::models::audio::sound_channel::SoundChannel;
use crate::models::audio::sound_handle_channel::SoundHandleChannel;

#[derive(Default)]
pub struct SoundManager {
    channel_vector_pickup: Vec<AudioChannel<MainTrack>>,
    channel_vector_projectile: Vec<AudioChannel<MainTrack>>,
    channel_vector_background: Vec<AudioChannel<MainTrack>>,
    channel_vector_misc: Vec<AudioChannel<MainTrack>>,

    sound_handle_vector: Vec<SoundHandleChannel>,
}

impl SoundManager {
    pub fn set_volume_for_channel(&self, volume: f32, channel: SoundChannel, audio: Res<Audio>) {
        match channel {
            SoundChannel::Pickup => {
                for channel in self.channel_vector_pickup.iter() {
                    audio.set_volume(volume as f64);
                }
            }
            SoundChannel::Projectile => {
                for channel in self.channel_vector_projectile.iter() {
                    audio.set_volume(volume as f64);
                }
            }
            SoundChannel::Misc => {
                for channel in self.channel_vector_misc.iter() {
                    audio.set_volume(volume as f64);
                }
            }
            SoundChannel::Background => {
                for channel in self.channel_vector_background.iter() {
                    audio.set_volume(volume as f64);
                }
            }
        }
    }

    pub fn sound_manager_system(
        mut sound_manager: ResMut<SoundManager>,
        audio: Res<Audio>,
    ) {
        while !sound_manager.sound_handle_vector.is_empty() {
            let channel_identifier = sound_manager.sound_handle_vector.remove(0);

            match channel_identifier {
                SoundHandleChannel::Pickup(handle) => {
                    let current_channel = sound_manager.channel_vector_pickup.remove(0);
                    audio.stop();
                    audio.play(handle.clone());
                    sound_manager.channel_vector_pickup.push(current_channel);
                }

                SoundHandleChannel::Projectile(handle) => {
                    let current_channel = sound_manager.channel_vector_projectile.remove(0);
                    audio.stop();
                    audio.play(handle.clone());
                    sound_manager.channel_vector_projectile.push(current_channel);
                }

                SoundHandleChannel::Misc(handle) => {
                    let current_channel = sound_manager.channel_vector_misc.remove(0);
                    audio.stop();
                    audio.play(handle.clone());
                    sound_manager.channel_vector_misc.push(current_channel);
                }
                SoundHandleChannel::Background(handle) => {
                    let current_channel = sound_manager.channel_vector_background.remove(0);
                    audio.stop();
                    audio.play(handle.clone());
                    sound_manager.channel_vector_background.push(current_channel);
                }
            }
        }
    }

    pub fn new() -> Self {
        let mut channel_vector = SoundManager::default();

        for index in 1..4 {
            channel_vector.channel_vector_pickup.push(AudioChannel::default());
        }

        for index in 1..6 {
            channel_vector.channel_vector_pickup.push(AudioChannel::default());
        }

        for index in 1..2 {
            channel_vector.channel_vector_pickup.push(AudioChannel::default());
        }

        for index in 1..3 {
            channel_vector.channel_vector_pickup.push(AudioChannel::default());
        }

        channel_vector
    }

    pub fn queue_sound(
        &mut self,
        handle_enum: SoundHandleChannel,
    ) {
        // self.sound_handle_vector.push(handle_enum)
    }
}
