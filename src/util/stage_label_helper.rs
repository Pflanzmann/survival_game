use bevy::prelude::SystemSet;

pub fn in_first(mut set: SystemSet) -> SystemSet {
    set
        .before("first")
}

pub fn in_collision(mut set: SystemSet) -> SystemSet {
    set
        .before("collision")
        .after("first")
}

pub fn in_pre_update(mut set: SystemSet) -> SystemSet {
    set
        .before("preUpdate")
        .after("collision")
}

pub fn in_update(mut set: SystemSet) -> SystemSet {
    set
        .before("Update")
        .after("preUpdate")
}

pub fn in_post_update(mut set: SystemSet) -> SystemSet {
    set
        .before("PostUpdate")
        .after("Update")
}

pub fn in_last(mut set: SystemSet) -> SystemSet {
    set
        .after("PostUpdate")
}