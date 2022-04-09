use bevy::ecs::component::Component;

use crate::models::attributes::attribute::{Attribute, AttributeAffect};
use crate::models::attributes::move_speed::MoveSpeed;

#[derive(Component)]
pub struct Sprinting {
    pub boost_amount: f32,
}

impl AttributeAffect<MoveSpeed> for Sprinting {
    fn add_affect(&self, attribute: &mut MoveSpeed) {
        attribute.add_bonus_to_amount(self.boost_amount)
    }

    fn remove_affect(&self, attribute: &mut MoveSpeed) {
        attribute.add_bonus_to_amount(-self.boost_amount)
    }
}