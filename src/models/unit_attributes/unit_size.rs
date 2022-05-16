use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize, Copy, Clone, Default)]
pub struct UnitSize {
    base_size: Vec2,

    base_amount: f32,
    bonus_amount: f32,
}

impl Attribute for UnitSize {
    fn new(base_amount: f32) -> Self {
        Self { base_amount: 1.0, bonus_amount: 0.0, base_size: Vec2::new(base_amount, base_amount) }
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

impl UnitSize {
    pub fn new_size(base_size: Vec2) -> Self {
        Self { base_amount: 1.0, bonus_amount: 0.0, base_size }
    }

    pub fn proportional_unit_size(&self) -> Vec2 {
        self.base_size * self.get_total_amount()
    }
}