use bevy::app::App;
use bevy::prelude::{Plugin, SystemSet};

use crate::AppState;
use crate::audio::background_music_system::play_background_music_system;
use crate::util::stage_label_helper::in_update;
use crate::audio::control_music_system::control_music_system;

mod background_music_system;
mod control_music_system;

pub struct CustomAudioPlugin;

impl Plugin for CustomAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_update(
                    SystemSet::on_exit(AppState::MainMenu)
                        .with_system(play_background_music_system)
                )
            )

            .add_system_set(
                    in_update(
                        SystemSet::on_update(AppState::InGame)
                            .with_system(control_music_system)
                    )
            );
    }
}