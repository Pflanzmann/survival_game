use bevy::prelude::{Res, ResMut};
use bevy_kira_audio::{Audio, AudioChannel};
use crate::models::audio::sound_channel::SoundChannel;

use crate::models::audio::sound_handle_channel::SoundHandleChannel;

#[derive(Default)]
pub struct SoundManager {
    channel_vector_pickup: Vec<AudioChannel>,
    channel_vector_bullet: Vec<AudioChannel>,
    channel_vector_background: Vec<AudioChannel>,
    channel_vector_misc: Vec<AudioChannel>,

    sound_handle_vector: Vec<SoundHandleChannel>,
}

impl SoundManager {

    pub fn set_volume_for_channel(&self, volume : f32, channel : SoundChannel, audio : Res<Audio>){
        match channel {
            SoundChannel::Pickup => {
                for channel in self.channel_vector_pickup.iter(){
                    audio.set_volume_in_channel(volume, channel)
                }
            }
            SoundChannel::Bullet => {
                for channel in self.channel_vector_bullet.iter(){
                    audio.set_volume_in_channel(volume, channel)
                }
            }
            SoundChannel::Misc => {
                for channel in self.channel_vector_misc.iter(){
                    audio.set_volume_in_channel(volume, channel)
                }
            }
            SoundChannel::Background => {
                for channel in self.channel_vector_background.iter(){
                    audio.set_volume_in_channel(volume, channel)
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
                    audio.stop_channel(&current_channel);
                    audio.play_in_channel(handle.clone(), &current_channel);
                    sound_manager.channel_vector_pickup.push(current_channel);
                }

                SoundHandleChannel::Bullet(handle) => {
                    let current_channel = sound_manager.channel_vector_bullet.remove(0);
                    audio.stop_channel(&current_channel);
                    audio.play_in_channel(handle.clone(), &current_channel);
                    sound_manager.channel_vector_bullet.push(current_channel);
                }

                SoundHandleChannel::Misc(handle) => {
                    let current_channel = sound_manager.channel_vector_misc.remove(0);
                    audio.stop_channel(&current_channel);
                    audio.play_in_channel(handle.clone(), &current_channel);
                    sound_manager.channel_vector_misc.push(current_channel);
                }
                SoundHandleChannel::Background(handle) => {
                    let current_channel = sound_manager.channel_vector_background.remove(0);
                    audio.stop_channel(&current_channel);
                    audio.play_in_channel(handle.clone(), &current_channel);
                    sound_manager.channel_vector_background.push(current_channel);
                }
            }
        }
    }

    pub fn new() -> Self {
        let mut channel_vector = SoundManager::default();

        for index in 1..4 {
            channel_vector.channel_vector_pickup.push(AudioChannel::new(format!("pickup_channel_{}", index).to_owned()));
        }

        for index in 1..6 {
            channel_vector.channel_vector_bullet.push(AudioChannel::new(format!("bullet_channel_{}", index).to_owned()));
        }

        for index in 1..2 {
            channel_vector.channel_vector_background.push(AudioChannel::new(format!("background_channel_{}", index).to_owned()));
        }

        for index in 1..3 {
            channel_vector.channel_vector_misc.push(AudioChannel::new(format!("misc_channel_{}", index).to_owned()));
        }

        channel_vector
    }

    pub fn queue_sound(
        &mut self,
        handle_enum: SoundHandleChannel,
    ) {
        self.sound_handle_vector.push(handle_enum)
    }
}
