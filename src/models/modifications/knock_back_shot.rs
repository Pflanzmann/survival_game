use bevy::ecs::component::Component;
use serde::Deserialize;

use crate::models::knock_back::KnockBack;
use crate::models::modifications::utils::associate_component::AssociateComponent;

#[derive(Copy, Clone, Component, Deserialize)]
pub struct KnockBackShot {
    pub push_duration: f32,
    pub push_force: f32,
}

impl AssociateComponent<KnockBack> for KnockBackShot {
    fn get_component(&self) -> KnockBack {
        KnockBack {
            push_duration: self.push_duration,
            push_force: self.push_force,
        }
    }
}