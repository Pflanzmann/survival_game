use bevy::ecs::bundle::Bundle;

use crate::models::attributes::damage::Damage;
use crate::models::attributes::health::Health;
use crate::models::attributes::move_speed::MoveSpeed;
use crate::models::collider::collider::Collider;
use crate::models::player_components::AimDirection;
use crate::models::unit_stats_components::{MoveDirection, UnitSize};
use crate::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    pub collider: Collider,
    pub unit_size: UnitSize,

    pub aim_direction: AimDirection,
    pub move_direction: MoveDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub health: Health,
}