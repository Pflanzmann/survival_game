use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Debug)]
pub struct CoinCount {
    pub number : i32,
}