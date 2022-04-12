use bevy::ecs::component::Component;

use crate::models::attributes::attribute::Attribute;
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::modification_attributes::attribute_affect::AttributeAffect;

#[derive(Component, Copy, Clone)]
pub struct AffectMoveSpeed {
    pub boost_amount: f32,
}

impl AttributeAffect<MoveSpeed> for AffectMoveSpeed {
    fn add_affect(&self, attribute: &mut MoveSpeed) {
        attribute.add_bonus_amount(self.boost_amount);
    }

    fn remove_affect(&self, attribute: &mut MoveSpeed) {
        attribute.add_bonus_amount(-self.boost_amount);
    }
}