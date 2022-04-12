use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize)]
pub struct Health {
    base_amount: f32,
    bonus_amount: f32,
    current_health: f32,
}

impl Attribute for Health {
    fn new(base_amount: f32) -> Self {
        Health { base_amount, bonus_amount: 0.0, current_health: base_amount }
    }

    fn get_total_amount(&self) -> f32 {
        self.base_amount + self.bonus_amount
    }

    fn get_base_amount(&self) -> f32 {
        self.base_amount
    }

    fn get_bonus_amount(&self) -> f32 {
        self.bonus_amount
    }

    fn add_bonus_amount(&mut self, added_amount: f32) {
        self.bonus_amount += added_amount;
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