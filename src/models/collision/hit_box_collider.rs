use bevy::ecs::component::Component;

use crate::models::collision::collider_type::ColliderType;

#[derive(Component)]
pub struct HitBoxCollider {
    pub collider_type: ColliderType,
}
