use bevy::ecs::component::Component;
use serde::Deserialize;
use serde::Serialize;
use crate::models::modifications::affects::affect_operator::AffectOperator;

use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;

#[derive(Component, Copy, Clone, Deserialize, Serialize)]
pub struct AffectBulletMoveSpeed {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<MoveSpeed> for AffectBulletMoveSpeed {
    fn add_affect(&self, attribute: &mut MoveSpeed) {
        attribute.add_bonus_amount(
            self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }

    fn remove_affect(&self, attribute: &mut MoveSpeed) {
        attribute.add_bonus_amount(
            -self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }
}
