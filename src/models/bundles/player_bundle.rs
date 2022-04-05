use bevy::ecs::bundle::Bundle;

use crate::{Damage, FacingDirection, Health, MoveSpeed, Player, UnitSize};
use crate::models::collider::collider::Collider;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    pub collider: Collider,
    pub unit_size: UnitSize,

    pub facing_direction: FacingDirection,
    pub move_speed: MoveSpeed,
    pub damage: Damage,
    pub health: Health,
}