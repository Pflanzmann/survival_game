use bevy::prelude::SystemSet;

use crate::navigation::ScheduleLabel;

pub fn in_first(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::First)
}

pub fn in_collision(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::Collision)
        .after(ScheduleLabel::First)
}

pub fn in_pre_update(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::PreUpdate)
        .after(ScheduleLabel::Collision)
}

pub fn in_update(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::Update)
        .after(ScheduleLabel::PreUpdate)
}

pub fn in_post_update(set: SystemSet) -> SystemSet {
    set
        .before(ScheduleLabel::PostUpdate)
        .after(ScheduleLabel::Update)
}

pub fn in_last(set: SystemSet) -> SystemSet {
    set
        .after(ScheduleLabel::PostUpdate)
}