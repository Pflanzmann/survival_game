use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Copy, Clone, Hash, Component, Deserialize)]
pub struct CurveShot;
