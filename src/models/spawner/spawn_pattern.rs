use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum SpawnPattern {
    Random {
        spawn_interval: f32
    },

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
        SpawnPattern::Random { spawn_interval: 1.0 }
    }
}