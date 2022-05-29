use crate::ToAppState;

#[derive(Default, Debug)]
pub struct AppStateTrigger {
    pub state_change_trigger: ToAppState,
}