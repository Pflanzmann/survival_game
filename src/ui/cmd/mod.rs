use bevy::core::FixedTimestep;
use bevy::prelude::{Plugin, SystemSet};

use crate::{App, ConsoleState};
use crate::models::ui::attribute_text::{AttributeWindow, ProjectileDamageText, ProjectileHitLimitText, ProjectileMoveSpeedText, ProjectileTravelRangeText, ProjectileUnitSizeText, DamageText, HealthText, MoveSpeedText, ReloadText, UnitSizeText};
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::ui::cmd::attribute_window::{assign_attribute_value_system, assign_projectile_attribute_value_system, spawn_attribute_window_system};
use crate::ui::cmd::debug_window::{exit_debug_console_system, setup_debug_info_window, setup_debug_window};
use crate::ui::cmd::fps_counter_update_system::fps_counter_update_system;
use crate::ui::cmd::update_console_history::update_console_history;
use crate::util::helper_systems::despawn_recursive_system::despawn_recursive_system;
use crate::util::stage_label_helper::in_update;

mod debug_window;
mod update_console_history;
mod spawn_collision_boxes;
mod fps_counter_update_system;
mod attribute_window;

pub struct CmdUiPlugin;

impl Plugin for CmdUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_exit(ConsoleState::Hidden)
                    .with_system(setup_debug_window)
                    .with_system(setup_debug_info_window)
            )

            .add_system_set(
                in_update(
                    SystemSet::new()
                        .with_run_criteria(FixedTimestep::step(0.1))
                        .with_system(fps_counter_update_system)
                )
            )

            .add_system_set(
                SystemSet::on_exit(ConsoleState::Shown)
                    .with_system(exit_debug_console_system)
            )
            .add_system_set(
                SystemSet::on_update(ConsoleState::Shown)
                    .with_system(update_console_history)
            )

            .add_system_set(
                in_update(
                    SystemSet::on_enter(ConsoleState::Shown)
                        // .with_system(init_collision_boxes)
                        .with_system(spawn_attribute_window_system)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_update(ConsoleState::Shown)
                    // .with_system(spawn_collision_boxes)
                )
            )

            .add_system_set(
                in_update(
                    SystemSet::on_exit(ConsoleState::Shown)
                        // .with_system(despawn_collision_boxes)
                        .with_system(despawn_recursive_system::<AttributeWindow>)
                )
            )

            .add_system_set(
                SystemSet::on_update(ConsoleState::Shown)
                    .with_system(assign_attribute_value_system::<Health, HealthText>)
                    .with_system(assign_attribute_value_system::<Damage, DamageText>)
                    .with_system(assign_attribute_value_system::<MoveSpeed, MoveSpeedText>)
                    .with_system(assign_attribute_value_system::<Reload, ReloadText>)
                    .with_system(assign_attribute_value_system::<UnitSize, UnitSizeText>)
                    .with_system(assign_projectile_attribute_value_system::<Damage, ProjectileDamageText>)
                    .with_system(assign_projectile_attribute_value_system::<MoveSpeed, ProjectileMoveSpeedText>)
                    .with_system(assign_projectile_attribute_value_system::<UnitSize, ProjectileUnitSizeText>)
                    .with_system(assign_projectile_attribute_value_system::<TravelRange, ProjectileTravelRangeText>)
                    .with_system(assign_projectile_attribute_value_system::<HitLimit, ProjectileHitLimitText>)
            )
        ;
    }
}
