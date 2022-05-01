use bevy::ecs::bundle::Bundle;

use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::aim_direction::AimDirection;
use crate::models::unit_size::UnitSize;
use crate::models::move_direction::MoveDirection;
use crate::models::player::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    pub unit_size: UnitSize,

    pub aim_direction: AimDirection,
    pub move_direction: MoveDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub health: Health,
}