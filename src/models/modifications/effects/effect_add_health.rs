use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Component, Copy, Clone, Deserialize)]
pub struct EffectAddHealth {
    pub amount: f32,
}
