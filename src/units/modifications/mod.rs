use crate::{App, AppState, Plugin, SystemSet};
use crate::util::stage_label_helper::{in_post_update};

pub mod sprinting;

pub struct UnitModificationsPlugin;

impl Plugin for UnitModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                in_post_update(
                    SystemSet::on_update(AppState::InGame)

                        // .with_system(apply_attribute_system::<MoveSpeed, Sprinting>)
                        // .with_system(apply_attribute_system::<Damage, Sprinting>)
                )
            );
    }
}