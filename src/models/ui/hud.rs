use bevy::ecs::component::Component;
use bevy::prelude::Entity;

#[derive(Component)]
pub struct HealthBar {
    pub owner: Entity,
}

#[derive(Component)]
pub struct BulletHud;

#[derive(Component)]
pub struct CoinText;