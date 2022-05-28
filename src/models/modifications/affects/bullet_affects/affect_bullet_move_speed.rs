use bevy::ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;
use crate::models::modifications::affects::affect_operator::AffectOperator;

use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;

#[derive(Component, Copy, Clone, Deserialize, Serialize)]
pub struct AffectBulletMoveSpeed {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<MoveSpeed> for AffectBulletMoveSpeed {
    fn get_operator(&self) -> &AffectOperator {
        &self.operator
    }

    fn get_amount(&self) -> f32 {
        self.amount
    }
}
