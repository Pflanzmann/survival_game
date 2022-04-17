use bevy::ecs::component::Component;
use serde::Deserialize;
use crate::models::modifications::affects::affect_operator::AffectOperator;

use crate::models::unit_attributes::attribute::Attribute;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::reload::Reload;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct AffectReload {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<Reload> for AffectReload {
    fn add_affect(&self, attribute: &mut Reload) {
        attribute.add_bonus_amount(
            self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }

    fn remove_affect(&self, attribute: &mut Reload) {
        attribute.add_bonus_amount(
            -self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }
}
