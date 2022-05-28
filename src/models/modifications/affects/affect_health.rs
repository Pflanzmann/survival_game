use bevy::ecs::component::Component;
use serde::Deserialize;
use crate::models::modifications::affects::affect_operator::AffectOperator;

use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::health::Health;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct AffectHealth {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<Health> for AffectHealth {
    fn get_operator(&self) -> &AffectOperator {
        &self.operator
    }

    fn get_amount(&self) -> f32 {
        self.amount
    }
}
