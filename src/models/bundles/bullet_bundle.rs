use bevy::ecs::bundle::Bundle;

use crate::models::attributes::damage::Damage;
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::bullet_components::{Bullet, BulletRange, HitLimit};
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider::Collider;
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
    pub bullet_range: BulletRange,
}