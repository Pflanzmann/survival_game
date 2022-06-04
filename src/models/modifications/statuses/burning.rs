use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct Burning {
    pub source_entity: Entity,
    pub debuff_duration: f32,
    pub damage_interval: f32,
    pub damage_interval_timer: f32,
    pub damage_per_tick: f32,
}

impl Burning {
    pub fn new(source_entity: Entity, debuff_duration: f32, damage_interval: f32, damage_per_tick: f32) -> Self {
        Burning { source_entity, debuff_duration, damage_interval, damage_interval_timer: 0.0, damage_per_tick }
    }
}

#[derive(Component)]
pub struct BurningEffect;
