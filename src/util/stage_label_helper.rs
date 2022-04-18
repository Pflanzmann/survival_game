use bevy::prelude::SystemSet;

use crate::navigation::ScheduleLabel;

/// A method to schedule a [SystemSet] in the simulated stage [ScheduleLabel::First]
/// inside the [CoreStage::Update]
pub fn in_first(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::First)
}

/// A method to schedule a [SystemSet] in the simulated stage [ScheduleLabel::Collision]
/// inside the [CoreStage::Update]
pub fn in_collision(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::Collision)
        .after(ScheduleLabel::First)
}

/// A method to schedule a [SystemSet] in the simulated stage [ScheduleLabel::PreUpdate]
/// inside the [CoreStage::Update]
pub fn in_pre_update(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::PreUpdate)
        .after(ScheduleLabel::Collision)
}

/// A method to schedule a [SystemSet] in the simulated stage [ScheduleLabel::Update]
/// inside the [CoreStage::Update]
pub fn in_update(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::Update)
        .after(ScheduleLabel::PreUpdate)
}

/// A method to schedule a [SystemSet] in the simulated stage [ScheduleLabel::PostUpdate]
/// inside the [CoreStage::Update]
pub fn in_post_update(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::PostUpdate)
        .after(ScheduleLabel::Update)
}

/// A method to schedule a [SystemSet] in the simulated stage [ScheduleLabel::PostUpdate]
/// inside the [CoreStage::Update]
pub fn in_last(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::Last)
        .after(ScheduleLabel::PostUpdate)
}