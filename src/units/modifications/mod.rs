use crate::{App, AppState, Plugin, SystemSet};
use crate::models::modifications::sprinting::Sprinting;
use crate::units::modifications::apply_player_mod_to_target_system::apply_mod_to_target_system;
use crate::util::stage_label_helper::in_post_update;

mod apply_player_mod_to_target_system;
mod apply_affect_system;

pub struct UnitModificationsPlugin;

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_post_update(
                    SystemSet::on_update(AppState::InGame)

                        .with_system(apply_mod_to_target_system::<Sprinting>)
                )
            );
    }
}