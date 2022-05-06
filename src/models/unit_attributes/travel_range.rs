use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize, Copy, Clone, Default)]
pub struct TravelRange {
    base_amount: f32,
    bonus_amount: f32,

    pub distance_traveled: f32,
}

impl Attribute for TravelRange {
    fn new(base_amount: f32) -> Self {
        TravelRange { base_amount, bonus_amount: 0.0, distance_traveled: 0.0 }
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