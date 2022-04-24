use bevy::asset::{AssetServer, Handle};
use bevy::input::Input;
use bevy::prelude::{KeyCode, Res, ResMut};
use bevy_kira_audio::{Audio, AudioChannel, AudioSource, InstanceHandle, PlaybackState};
use crate::models::resources::sound_counter::SoundCounter;


pub fn sound_test_system(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut sound_counter : ResMut<SoundCounter>
) {
    let mut handle_1: Handle<AudioSource>;
    let mut music_handle: InstanceHandle;

    handle_1 = asset_server.get_handle("audio/coin_pickup_sound.ogg");

    if input.pressed(KeyCode::P) {
        if sound_counter.loaded_sounds > 5 {
            sound_counter.downtick_factor += 1;
        }else {
            music_handle = audio.play(handle_1);
            println!("{:?}", audio.state(music_handle));
            sound_counter.loaded_sounds += 1;
            sound_counter.downtick_factor += 1;
        }
    }

    if sound_counter.downtick_factor > 5 {
        sound_counter.loaded_sounds -= 1;
        sound_counter.downtick_factor = 0;
    }

}
