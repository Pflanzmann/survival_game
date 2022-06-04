use bevy::ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;

use crate::models::modifications::affects::affect_operator::AffectOperator;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::move_speed::MoveSpeed;

#[derive(Component, Copy, Clone, Deserialize, Serialize)]
pub struct AffectProjectileMoveSpeed {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<MoveSpeed> for AffectProjectileMoveSpeed {
    fn get_operator(&self) -> &AffectOperator {
        &self.operator
    }

    fn get_amount(&self) -> f32 {
        self.amount
    }
}
