use bevy::ecs::bundle::Bundle;

use crate::{Damage, MoveDirection, MoveSpeed, UnitSize};
use crate::models::bullet_components::{Bullet, BulletRange, HitLimit};
use crate::models::collider::collided_entities::CollidedEntities;
use crate::models::collider::collider::Collider;

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