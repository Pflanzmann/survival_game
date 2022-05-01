use bevy::ecs::bundle::Bundle;

use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::enemy::Enemy;
use crate::models::unit_size::UnitSize;
use crate::models::move_direction::MoveDirection;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,

    pub unit_size: UnitSize,

    pub facing_direction: MoveDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub health: Health,
}