use bevy::ecs::bundle::Bundle;

use crate::models::bullet::Bullet;
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_size::UnitSize;
use crate::models::move_direction::MoveDirection;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,

    pub unit_size: UnitSize,
    pub collider_entities: CollidedEntities,

    pub facing_direction: MoveDirection,
}