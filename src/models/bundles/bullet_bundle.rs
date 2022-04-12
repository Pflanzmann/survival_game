use bevy::ecs::bundle::Bundle;

use crate::models::bullet::Bullet;
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider::Collider;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::models::unit_stats_components::{MoveDirection, UnitSize};

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,

    pub collider: Collider,
    pub unit_size: UnitSize,
    pub collider_entities: CollidedEntities,

    pub facing_direction: MoveDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub hit_limit: HitLimit,
    pub travel_range: TravelRange,
}