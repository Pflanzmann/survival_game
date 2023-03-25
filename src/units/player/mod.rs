use bevy::prelude::*;

use setup_player_health_bar_system::setup_player_health_bar_system;

use crate::{App, AppState};
use crate::scheduling::BaseSets;
use crate::units::player::player_died_system::player_died_system;
use crate::units::player::player_hit_system::player_hit_system;
use crate::units::player::setup_player_system::setup_player_system;

pub mod setup_player_system;
pub mod setup_player_health_bar_system;
pub mod player_hit_system;
pub mod player_died_system;


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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSetupSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerUpdateSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerDiedSystemSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PlayerUpdateSystemSet
                .in_base_set(BaseSets::PreUpdate)
                .run_if(in_state(AppState::InGame))
        );

        app.configure_set(
            PlayerDiedSystemSet
                .in_base_set(BaseSets::Last)
                .run_if(in_state(AppState::InGame))
        );

        app
            .add_systems((
                setup_player_system,
                apply_system_buffers,
                setup_player_health_bar_system
            ).chain().in_schedule(OnExit(AppState::MainMenu)));


        app.add_system(player_hit_system.in_set(PlayerUpdateSystemSet));

        app.add_system(player_died_system.in_set(PlayerDiedSystemSet));
    }
}
