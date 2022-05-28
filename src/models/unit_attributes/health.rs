use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize)]
pub struct Health {
    base_amount: f32,
    bonus_amount: f32,
    current_health: f32,
    multiplier: f32,
}

impl Attribute for Health {
    fn new(base_amount: f32) -> Self {
        Self { base_amount, bonus_amount: 0.0, current_health: base_amount, multiplier: 1.0 }
    }

    fn get_total_amount(&self) -> f32 {
        (self.base_amount + self.bonus_amount) * self.multiplier
    }

    fn get_base_amount(&self) -> f32 {
        self.base_amount
    }

    fn get_bonus_amount(&self) -> f32 {
        self.bonus_amount
    }

    fn get_multiplier(&self) -> f32 {
        self.multiplier
    }

    fn add_bonus_amount(&mut self, added_amount: f32) {
        let total_health = self.get_total_amount();
        let health_ratio = total_health / self.current_health;

        self.bonus_amount += added_amount;
        
        self.current_health = self.get_total_amount() / health_ratio;
    }

    fn add_multiplier(&mut self, multiplier: f32) {
        let total_health = self.get_total_amount();
        let health_ratio = total_health / self.current_health;

        self.multiplier *= multiplier;

        self.current_health = self.get_total_amount() / health_ratio;
    }
}

impl Health {
    pub fn get_current_health(&self) -> f32 {
        self.current_health
    }

    pub fn damage(&mut self, amount: f32) {
        self.current_health -= amount;

        if self.current_health < 0.0 {
            self.current_health = 0.0
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current_health += amount;

        if self.current_health > self.get_total_amount() {
            self.current_health = self.get_total_amount()
        }
    }
}