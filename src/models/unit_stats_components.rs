use bevy::ecs::component::Component;
use bevy::math::Vec3;

use crate::Vec2;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub current_health: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Health { max_health, current_health: (max_health) }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current_health += amount;

        if self.current_health > self.max_health {
            self.current_health = self.max_health
        }
    }
}

#[derive(Component)]
pub struct Damage {
    pub base_damage: f32,
    pub bonus_damage: f32,
}

impl Damage {
    pub fn new(base_damage: f32) -> Self {
        Damage { base_damage, bonus_damage: 0.0 }
    }

    pub fn get_damage(&self) -> f32 {
        return self.base_damage + self.bonus_damage;
    }
}

#[derive(Component)]
pub struct MoveDirection {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct UnitSize {
    pub collider_size: Vec2,
}
