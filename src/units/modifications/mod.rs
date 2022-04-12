use crate::{App, AppState, Plugin, SystemSet};
use crate::models::modification_components::{CurveShot, GrowShot, SplitShot};
use crate::models::unit_modifications::sprinting::Sprinting;
use crate::units::modifications::apply_bullet_mod_to_target_system::apply_bullet_mod_to_target_system;
use crate::units::modifications::apply_player_mod_to_target_system::apply_player_mod_to_target_system;
use crate::util::stage_label_helper::in_post_update;

mod apply_player_mod_to_target_system;
mod apply_affect_system;
mod apply_bullet_mod_to_target_system;

pub struct UnitModificationsPlugin;

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_post_update(
                    SystemSet::on_update(AppState::InGame)

                        .with_system(apply_bullet_mod_to_target_system::<CurveShot>)
                        .with_system(apply_bullet_mod_to_target_system::<GrowShot>)
                        .with_system(apply_bullet_mod_to_target_system::<SplitShot>)

                        .with_system(apply_player_mod_to_target_system::<Sprinting>)
                )
            )

            .add_system_set(
                in_post_update(
                    SystemSet::on_update(AppState::Shop)


                        .with_system(apply_bullet_mod_to_target_system::<CurveShot>)
                        .with_system(apply_bullet_mod_to_target_system::<GrowShot>)
                        .with_system(apply_bullet_mod_to_target_system::<SplitShot>)

                        .with_system(apply_player_mod_to_target_system::<Sprinting>)
                )
            );
    }
}
