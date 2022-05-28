use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Copy, Clone, Serialize)]
pub enum AffectOperator {
    Add,
    Multiply,
}

