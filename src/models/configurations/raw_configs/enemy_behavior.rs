use serde::Deserialize;

#[derive(Default, Deserialize, Copy, Clone)]
#[serde(tag = "type")]
pub enum EnemyBehavior {
    #[default]
    ChasePlayer,

    SidedMovement {
        horizontal: bool
    },

    DirectionalMovement,
}