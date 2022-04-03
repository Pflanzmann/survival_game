use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{CoreStage, Plugin, SystemSet};

use crate::App;
use crate::bullets::modifications::apply_modification_system::apply_modification_system;
use crate::bullets::modifications::curve_shot_system::curve_shot_system;
use crate::bullets::modifications::grow_shot_system::grow_shot_system;
use crate::bullets::modifications::split_shot_system::split_shot_system;
use crate::components::modification_components::{CurveShot, GrowShot, SplitShot};

pub mod curve_shot_system;
pub mod grow_shot_system;
pub mod split_shot_system;
mod apply_modification_system;

pub struct ModificationsPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct BulletModStage;

impl Plugin for ModificationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()

                    .with_system(apply_modification_system::<CurveShot>)
                    .with_system(curve_shot_system)

                    .with_system(apply_modification_system::<GrowShot>)
                    .with_system(grow_shot_system)

                    .with_system(apply_modification_system::<SplitShot>)
                    .with_system(split_shot_system));
    }
}