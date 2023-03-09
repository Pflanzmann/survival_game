extern crate core;

use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Res, ResMut, Resource};
use bevy_kira_audio::AudioSource;

#[derive(Default, Resource)]
pub struct SoundHandles {
    pub coin_pickup_sound: Handle<AudioSource>,
    pub background_music: Handle<AudioSource>,
    pub shoot_sound: Handle<AudioSource>,
    pub hit_sound: Handle<AudioSource>,
    pub teleport_sound: Handle<AudioSource>,
}

pub fn preload_audio_system(
    mut sound_handles: ResMut<SoundHandles>,
    asset_server: Res<AssetServer>,
) {
    sound_handles.coin_pickup_sound = asset_server.load("audio/coin_pickup_sound.ogg");
    sound_handles.background_music = asset_server.load("audio/background_theme.mp3");
    sound_handles.shoot_sound = asset_server.load("audio/shoot_sound.mp3");
    sound_handles.hit_sound = asset_server.load("audio/hit_sound.mp3");
    sound_handles.teleport_sound = asset_server.load("audio/teleport.mp3");
}