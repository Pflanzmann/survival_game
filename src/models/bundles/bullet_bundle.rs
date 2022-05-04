use bevy::ecs::bundle::Bundle;

use crate::models::bullet::Bullet;
use crate::models::collision::collided_entities::CollidedEntities;
use crate::models::unit_size::UnitSize;
use crate::models::move_direction::MoveDirection;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,

    pub unit_size: UnitSize,
    pub collider_entities: CollidedEntities,

    pub facing_direction: MoveDirection,
}