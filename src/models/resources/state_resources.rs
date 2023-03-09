use bevy::prelude::Resource;

use crate::ToAppState;

#[derive(Default, Debug, Resource)]
pub struct AppStateTrigger {
    pub state_change_trigger: ToAppState,
}