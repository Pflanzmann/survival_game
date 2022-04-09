use bevy::ecs::bundle::Bundle;

use crate::{Damage, Health, MoveDirection, UnitSize};
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::collider::collider::Collider;
use crate::models::unit_stats_components::Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,

    pub collider: Collider,
    pub unit_size: UnitSize,

    pub facing_direction: MoveDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub health: Health,
}