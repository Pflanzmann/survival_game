use bevy::ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::models::modifications::affects::affect_operator::AffectOperator;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::unit_size::UnitSize;

#[derive(Component, Copy, Clone, Deserialize, Serialize)]
pub struct AffectProjectileUnitSize {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<UnitSize> for AffectProjectileUnitSize {
    fn get_operator(&self) -> &AffectOperator {
        &self.operator
    }

    fn get_amount(&self) -> f32 {
        self.amount
    }
}
