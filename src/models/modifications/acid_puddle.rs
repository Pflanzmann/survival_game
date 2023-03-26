use bevy::ecs::component::Component;
use bevy::prelude::Entity;
use serde::Deserialize;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct AcidPuddle {
    pub time_alive: f32,
    pub proc_chance: f32,
    pub damage: f32,
    pub damage_ticks_per_min: f32,
}

#[derive(Component)]
pub struct AcidPuddleUnit;

#[derive(Copy, Clone, Component)]
pub struct AcidPuddleOwner {
    pub owner: Entity,
}