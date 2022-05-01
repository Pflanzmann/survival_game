use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;

#[derive(Component, Deserialize)]
pub struct MeeleAttackSpeed {
    base_amount: f32,
    bonus_amount: f32,

    pub reload_timer: f32,
}

impl Attribute for MeeleAttackSpeed {
    fn new(base_amount: f32) -> Self {
        MeeleAttackSpeed { base_amount, bonus_amount: 0.0, reload_timer: 0.0 }
    }

    fn get_total_amount(&self) -> f32 {
        60.0 / (self.base_amount + self.bonus_amount)
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