use bevy::prelude::SystemSet;

use crate::scheduling::BaseSets;

/// A method to schedule a [SystemSet] in the simulated stage [BaseSets::First]
/// inside the [CoreStage::Update]
pub fn in_first(set: SystemSet) -> SystemSet {
    set
        .before(BaseSets::First)
}

/// A method to schedule a [SystemSet] in the simulated stage [BaseSets::Collision]
/// inside the [CoreStage::Update]
pub fn in_collision(set: SystemSet) -> SystemSet {
    set
        .before(BaseSets::Collision)
        .after(BaseSets::First)
}

/// A method to schedule a [SystemSet] in the simulated stage [BaseSets::PreUpdate]
/// inside the [CoreStage::Update]
pub fn in_pre_update(set: SystemSet) -> SystemSet {
    set
        .before(BaseSets::PreUpdate)
        .after(BaseSets::Collision)
}

/// A method to schedule a [SystemSet] in the simulated stage [BaseSets::Update]
/// inside the [CoreStage::Update]
pub fn in_update(set: SystemSet) -> SystemSet {
    set
        .before(BaseSets::Update)
        .after(BaseSets::PreUpdate)
}

/// A method to schedule a [SystemSet] in the simulated stage [BaseSets::PostUpdate]
/// inside the [CoreStage::Update]
pub fn in_post_update(set: SystemSet) -> SystemSet {
    set
        .before(BaseSets::PostUpdate)
        .after(BaseSets::Update)
}

/// A method to schedule a [SystemSet] in the simulated stage [BaseSets::PostUpdate]
/// inside the [CoreStage::Update]
pub fn in_last(set: SystemSet) -> SystemSet {
    set
        .before(BaseSets::Last)
        .after(BaseSets::PostUpdate)
}