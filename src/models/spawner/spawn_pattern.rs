use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SpawnPattern {
    Random,

    Circle {
        spawn_angle_in_degree: f32
    },

    Grouped {
        enemy_amount: usize
    },

    Directional {
        horizontal: bool
    },
}

impl Default for SpawnPattern {
    fn default() -> Self {
        SpawnPattern::Random
    }
}