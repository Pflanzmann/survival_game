use bevy::ecs::schedule::ShouldRun;
use bevy::ecs::system::Resource;
use bevy::prelude::EventReader;

pub fn on_event<T: Resource>(
    event: EventReader<T>
) -> ShouldRun {
    if event.is_empty() {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}
