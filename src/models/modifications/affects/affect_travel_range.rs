use bevy::ecs::component::Component;
use serde::Deserialize;
use crate::models::modifications::affects::affect_operator::AffectOperator;

use crate::models::unit_attributes::attribute::Attribute;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::travel_range::TravelRange;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct AffectTravelRange {
    pub operator: AffectOperator,
    pub amount: f32,
}

impl AttributeAffect<TravelRange> for AffectTravelRange {
    fn add_affect(&self, attribute: &mut TravelRange) {
        attribute.add_bonus_amount(
            self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }

    fn remove_affect(&self, attribute: &mut TravelRange) {
        attribute.add_bonus_amount(
            -self.operator.calculate_amount(attribute.get_base_amount(), self.amount)
        );
    }
}
