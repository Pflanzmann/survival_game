use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::modifications::affects::affect_operator::AffectOperator;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::unit_size::UnitSize;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct AffectUnitSize {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<UnitSize> for AffectUnitSize {
    fn add_affect(&self, attribute: &mut UnitSize) {
        attribute.add_bonus_amount(self.operator.calculate_amount(attribute.get_base_amount(), self.amount));
    }

    fn remove_affect(&self, attribute: &mut UnitSize) {
        attribute.add_bonus_amount(
            -self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }
}
