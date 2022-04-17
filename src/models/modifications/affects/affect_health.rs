use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::unit_attributes::attribute::Attribute;
use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::health::Health;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct AffectHealth {
    pub boost_amount: f32,
}

impl AttributeAffect<Health> for AffectHealth {
    fn add_affect(&self, attribute: &mut Health) {
        attribute.add_bonus_amount(self.boost_amount);
    }

    fn remove_affect(&self, attribute: &mut Health) {
        attribute.add_bonus_amount(-self.boost_amount);
    }
}
