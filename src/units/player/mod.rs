use bevy::prelude::{Plugin, SystemSet};

use setup_player_health_bar_system::setup_player_health_bar_system;

use crate::{App, AppState};
use crate::units::player::player_died_system::player_died_system;
use crate::units::player::player_hit_system::player_hit_system;
use crate::units::player::animate_player_system::animate_sprite_system;
use crate::units::player::setup_player_system::setup_player_system;
use crate::util::stage_label_helper::{in_last, in_pre_update};

pub mod setup_player_system;
pub mod setup_player_health_bar_system;
pub mod player_hit_system;
pub mod player_died_system;
pub mod animate_player_system;


/// This plugin manages the everything related to [Player] systems and how they get applied.
///
/// The setup of the player happens in the main menu, because other setup-systems depend on having
/// a player already.
///
/// The [player_hit_system] is called in the [PreUpdate] because it reacts to the [CollisionPlugin].
///
/// The [player_died_system] is called in the [Last] because it despawns the entity as a very
/// last step so that other systems can still react to the [PlayerDiedEvent].
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(setup_player_system)
            )

            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(setup_player_health_bar_system)
            )

            .add_system_set(
                in_pre_update(
                    SystemSet::new()
                        .with_system(player_hit_system)
                )
            )

            .add_system_set(
                in_last(
                    SystemSet::on_update(AppState::InGame)
                        .with_system(player_died_system)
                        .with_system(animate_sprite_system)
                )
            )
        ;
    }
}
