use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct WeaponSlot {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Gunnable;

#[derive(Component)]
pub struct StraightBasicShot;

#[derive(Component)]
pub struct Reloadable {
    pub base_reloading_time: f32,
    pub reload_timer: f32,
}

impl Reloadable {
    pub fn new(base_reloading_time: f32) -> Self {
        Reloadable { base_reloading_time, reload_timer: 0.0 }
    }
}