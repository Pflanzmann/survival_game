use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Copy, Clone, Component)]
pub struct Bullet {
    pub source_entity: Entity,
}

#[derive(Component)]
pub struct BulletRange {
    pub total_range: f32,
    pub distance_traveled: f32,
}

impl BulletRange {
    pub fn new(total_range: f32) -> Self {
        BulletRange { total_range, distance_traveled: 0.0 }
    }
}

#[derive(Component)]
pub struct HitLimit {
    pub hit_limit: usize,
}