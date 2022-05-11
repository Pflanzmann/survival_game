use bevy::ecs::component::Component;
use bevy::prelude::Vec3;

use crate::models::collision::collider_type::ColliderType;

#[derive(Component)]
pub struct SolidBodyCollider {
    pub offset: Vec3,
    pub collider_type: ColliderType,
}
