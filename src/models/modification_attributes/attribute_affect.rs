use crate::models::attributes::attribute::Attribute;

pub trait AttributeAffect<T: Attribute> {
    fn add_affect(&self, attribute: &mut T);

    fn remove_affect(&self, attribute: &mut T);
}