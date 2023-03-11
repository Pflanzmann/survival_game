use std::time::Duration;

use bevy::app::IntoSystemAppConfig;
use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;

use crate::{App, ConsoleState};
use crate::models::ui::attribute_text::{AttributeWindow, DamageText, HealthText, MoveSpeedText, ProjectileDamageText, ProjectileHitLimitText, ProjectileMoveSpeedText, ProjectileTravelRangeText, ProjectileUnitSizeText, ReloadText, UnitSizeText};
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::reload::Reload;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_attributes::unit_size::UnitSize;
use crate::scheduling::BaseSets;
use crate::ui::cmd::attribute_window::{assign_attribute_value_system, assign_projectile_attribute_value_system, spawn_attribute_window_system};
use crate::ui::cmd::debug_window::{setup_debug_info_window, setup_debug_window};
use crate::ui::cmd::fps_counter_update_system::fps_counter_update_system;
use crate::ui::cmd::spawn_collision_boxes::init_collision_boxes;
use crate::ui::cmd::update_console_history::update_console_history;
use crate::util::helper_systems::despawn_recursive_system::despawn_recursive_system;

mod debug_window;
mod update_console_history;
mod spawn_collision_boxes;
mod fps_counter_update_system;
mod attribute_window;

pub struct CmdUiPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiEnterConsoleSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiExitConsoleSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiUpdateConsoleSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CmdUiFixedUpdateConsoleSystemSet;


impl Plugin for CmdUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            CmdUiEnterConsoleSystemSet
                .in_base_set(BaseSets::Update)
            // .in_schedule(OnEnter(ConsoleState::Shown))
        );

        app.configure_set(
            CmdUiExitConsoleSystemSet
                .in_base_set(BaseSets::Update)
            // .in_schedule(OnExit(ConsoleState::Shown))
        );

        app.configure_set(
            CmdUiUpdateConsoleSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(ConsoleState::Shown))
        );

        app.configure_set(
            CmdUiFixedUpdateConsoleSystemSet
                .in_base_set(BaseSets::Update)
                .run_if(in_state(ConsoleState::Shown))
                .run_if(on_fixed_timer(Duration::from_millis(10)))
        );

        app
            .add_system(spawn_attribute_window_system.in_set(CmdUiEnterConsoleSystemSet).in_schedule(OnEnter(ConsoleState::Shown)))
            .add_system(init_collision_boxes.in_set(CmdUiEnterConsoleSystemSet).in_schedule(OnEnter(ConsoleState::Shown)))
            .add_system(setup_debug_window.in_set(CmdUiEnterConsoleSystemSet).in_schedule(OnEnter(ConsoleState::Shown)))
            .add_system(setup_debug_info_window.in_set(CmdUiEnterConsoleSystemSet).in_schedule(OnEnter(ConsoleState::Shown)));

        app
            // .add_system(despawn_collision_boxes.in_set(CmdUiExitConsoleSystemSet).in_schedule(OnExit(ConsoleState::Shown)))
            .add_system(despawn_recursive_system::<AttributeWindow>.in_set(CmdUiExitConsoleSystemSet).in_schedule(OnExit(ConsoleState::Shown)));

        app
            // .add_system(render_collision_boxes.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(fps_counter_update_system.in_set(CmdUiFixedUpdateConsoleSystemSet));

        app
            .add_system(update_console_history.in_set(CmdUiUpdateConsoleSystemSet))

            .add_system(assign_attribute_value_system::<Health, HealthText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_attribute_value_system::<Damage, DamageText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_attribute_value_system::<MoveSpeed, MoveSpeedText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_attribute_value_system::<Reload, ReloadText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_attribute_value_system::<UnitSize, UnitSizeText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_projectile_attribute_value_system::<Damage, ProjectileDamageText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_projectile_attribute_value_system::<MoveSpeed, ProjectileMoveSpeedText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_projectile_attribute_value_system::<UnitSize, ProjectileUnitSizeText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_projectile_attribute_value_system::<TravelRange, ProjectileTravelRangeText>.in_set(CmdUiUpdateConsoleSystemSet))
            .add_system(assign_projectile_attribute_value_system::<HitLimit, ProjectileHitLimitText>.in_set(CmdUiUpdateConsoleSystemSet));
    }
}
