use bevy::ecs::bundle::Bundle;

use crate::{Damage, FacingDirection, Health, MoveSpeed, UnitSize};
use crate::models::collider::collider::Collider;
use crate::models::unit_stats_components::Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,

    pub collider: Collider,
    pub unit_size: UnitSize,

    pub facing_direction: FacingDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub health: Health,
}