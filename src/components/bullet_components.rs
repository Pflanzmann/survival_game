use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Bullet;

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