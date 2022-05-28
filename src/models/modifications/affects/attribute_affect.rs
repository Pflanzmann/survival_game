use crate::models::modifications::affects::affect_operator::AffectOperator;
use crate::models::unit_attributes::attribute::Attribute;

pub trait AttributeAffect<T: Attribute> {
    fn get_operator(&self) -> &AffectOperator;

    fn get_amount(&self) -> f32;

    fn add_affect(&self, attribute: &mut T) {
        match self.get_operator() {
            AffectOperator::Add => {
                attribute.add_bonus_amount(self.get_amount());
            }
            AffectOperator::Multiply => {
                attribute.add_multiplier(self.get_amount());
            }
        }
    }

    fn remove_affect(&self, attribute: &mut T) {
        match self.get_operator() {
            AffectOperator::Add => {
                attribute.add_bonus_amount(-self.get_amount());
            }
            AffectOperator::Multiply => {
                attribute.add_multiplier(self.get_amount().powf(-1.0));
            }
        }
    }
}
