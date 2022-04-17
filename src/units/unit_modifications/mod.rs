use bevy::app::Plugin;
use bevy::prelude::{App, SystemSet};

use crate::models::events::apply_mod_to_target_event::ApplyModToTargetEvent;
use crate::models::modifications::affects::affect_move_speed::AffectMoveSpeed;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::modifications::turret::Turret;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::util::run_criteria::on_event::on_event;
use crate::units::unit_modifications::apply_affect_system::apply_affect_system;
use crate::units::unit_modifications::apply_bullet_mod_to_target_system::apply_bullet_mod_to_target_system;
use crate::units::unit_modifications::apply_player_mod_to_target_system::apply_player_mod_to_target_system;
use crate::units::unit_modifications::turret_update_system::turret_update_system;
use crate::util::stage_label_helper::in_post_update;

mod apply_player_mod_to_target_system;
mod apply_affect_system;
mod apply_bullet_mod_to_target_system;
mod turret_update_system;

/// All the apply systems have to get registered here.
///
/// This [SystemSet] has the [RunCriteria] to run everything only if there is an
/// [ApplyModToTargetEvent]
///
pub struct UnitModificationsPlugin;

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_post_update(
                    SystemSet::new()
                        .with_run_criteria(on_event::<ApplyModToTargetEvent>)

                        .with_system(apply_affect_system::<MoveSpeed, AffectMoveSpeed>)

                        .with_system(apply_bullet_mod_to_target_system::<CurveShot>)
                        .with_system(apply_bullet_mod_to_target_system::<GrowShot>)
                        .with_system(apply_bullet_mod_to_target_system::<SplitShot>)

                        .with_system(apply_player_mod_to_target_system::<Sprinting>)
                        .with_system(apply_player_mod_to_target_system::<Turret>)
                )
            )

            .add_system(turret_update_system);
    }
}

