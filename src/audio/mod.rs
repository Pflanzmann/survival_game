use bevy::app::App;
use bevy::prelude::*;

use crate::audio::background_music_system::play_background_music_system;
use crate::audio::sound_manager::SoundManager;
use crate::scheduling::BaseSets;

mod background_music_system;
pub mod sound_manager;

pub struct CustomAudioPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AudioSystemSet;

impl Plugin for CustomAudioPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            AudioSystemSet
                .in_base_set(BaseSets::PostUpdate)
        );

        app.insert_resource(SoundManager::new());

        app
            .add_system(play_background_music_system.in_set(AudioSystemSet))
            .add_system(SoundManager::sound_manager_system.in_set(AudioSystemSet));
    }
}