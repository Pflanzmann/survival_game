pub trait Attribute {
    fn new(base_amount: f32) -> Self;

    fn get_total_amount(&self) -> f32;

    fn get_base_amount(&self) -> f32;

    fn get_bonus_amount(&self) -> f32;

    fn add_bonus_to_amount(&mut self, added_amount: f32);
}