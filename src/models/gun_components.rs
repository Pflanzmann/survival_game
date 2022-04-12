use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct WeaponSlot {
    pub weapon_entity: Entity,
}

#[derive(Component)]
pub struct StraightBasicShot;
