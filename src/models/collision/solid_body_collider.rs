use bevy::ecs::component::Component;
use bevy::prelude::Vec2;

use crate::models::collision::collider_type::ColliderType;

#[derive(Component)]
pub struct SolidBodyCollider {
    pub offset: Vec2,
    pub collider_type: ColliderType,
}
