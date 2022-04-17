use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Copy, Clone, Serialize)]
pub enum AffectOperator {
    Add,
    Multiply,
}

impl AffectOperator {
    pub fn calculate_amount(&self, base_amount: f32, boost_amount: f32) -> f32 {
        match &self {
            AffectOperator::Add => {
                boost_amount
            }
            AffectOperator::Multiply => {
                base_amount * boost_amount
            }
        }
    }
}
