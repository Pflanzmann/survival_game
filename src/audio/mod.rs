use bevy::app::App;
use bevy::prelude::{Plugin, SystemSet};

use crate::AppState;
use crate::audio::background_music_system::play_background_music_system;
use crate::audio::sound_manager::SoundManager;
use crate::util::stage_label_helper::in_update;

mod background_music_system;
pub mod sound_manager;

pub struct CustomAudioPlugin;

impl Plugin for CustomAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SoundManager::new())

            .add_system_set(
                in_update(
                    SystemSet::on_exit(AppState::MainMenu)
                        .with_system(play_background_music_system)
                )
            )

            .add_system(SoundManager::sound_manager_system);
    }
}