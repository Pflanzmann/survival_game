use serde::Deserialize;

#[derive(Default, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SpawnPattern {
    #[default]
    Random,

    Circle {
        spawn_angle_in_degree: f32
    },

    Grouped {
        enemy_amount: usize
    },

    Sided {
        horizontal: bool
    },
}
