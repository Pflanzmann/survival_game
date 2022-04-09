use bevy::prelude::ResMut;
use crate::{AppStateTrigger, ToAppState};

pub fn trigger_enter_main_system(
    mut state_trigger: ResMut<AppStateTrigger>,
) {
    state_trigger.State_Change_Trigger = ToAppState::ToMainMenu;
}