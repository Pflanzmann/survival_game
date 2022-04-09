use crate::{App, Plugin};
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::modification_components::CurveShot;
use crate::units::modifications::apply_attribute_system::apply_attribute_system;
use crate::units::modifications::sprinting::Sprinting;

pub mod sprinting;
mod apply_attribute_system;

pub struct UnitModificationsPlugin;

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_system(apply_attribute_system::<MoveSpeed, Sprinting>);
    }
}