use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::modifications::affects::attribute_affect::AttributeAffect;
use crate::models::unit_attributes::attribute::Attribute;
use crate::models::unit_attributes::damage::Damage;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct AffectDamage {
    pub boost_amount: f32,
}

impl AttributeAffect<Damage> for AffectDamage {
    fn add_affect(&self, attribute: &mut Damage) {
        attribute.add_bonus_amount(self.boost_amount);
    }

    fn remove_affect(&self, attribute: &mut Damage) {
        attribute.add_bonus_amount(-self.boost_amount);
    }
}
