use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize, Copy, Clone, Default)]
pub struct UnitSize {
    base_size: Vec2,

    base_amount: f32,
    bonus_amount: f32,
    multiplier: f32,
}

impl Attribute for UnitSize {
    fn new(base_amount: f32) -> Self {
        Self { base_amount: 1.0, bonus_amount: 0.0, multiplier: 1.0, base_size: Vec2::new(base_amount, base_amount) }
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

impl UnitSize {
    pub fn new_size(base_size: Vec2) -> Self {
        Self { base_amount: 1.0, bonus_amount: 0.0, multiplier: 1.0, base_size }
    }

    pub fn proportional_unit_size(&self) -> Vec2 {
        self.base_size * self.get_total_amount()
    }

    pub fn inherit_from(&mut self, other: &Self) {
        self.base_amount = other.base_amount;
        self.bonus_amount = other.bonus_amount;
        self.multiplier = other.multiplier;
    }
}
