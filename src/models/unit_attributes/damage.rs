use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize, Copy, Clone, Default)]
pub struct Damage {
    base_amount: f32,
    bonus_amount: f32,
    multiplier: f32,
}

impl Attribute for Damage {
    fn new(base_amount: f32) -> Self {
        Self { base_amount, bonus_amount: 0.0, multiplier: 1.0 }
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
        self.bonus_amount += added_amount;
    }

    fn add_multiplier(&mut self, multiplier: f32) {
        self.multiplier *= multiplier;
    }
}