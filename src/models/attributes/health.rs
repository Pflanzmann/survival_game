use bevy::ecs::component::Component;

use crate::models::attributes::attribute::Attribute;

#[derive(Component)]
pub struct Health {
    base_amount: f32,
    bonus_amount: f32,
}

impl Attribute for Health {
    fn new(base_amount: f32) -> Self {
        Health { base_amount, bonus_amount: 0.0 }
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

    fn add_bonus_to_amount(&mut self, added_amount: f32) {
        self.bonus_amount += added_amount;
    }
}