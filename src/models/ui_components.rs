use bevy::ecs::component::Component;
use bevy::math::Vec3;

use crate::Vec2;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Cointext;

#[derive(Component)]
pub struct BulletHud;

#[derive(Component)]
pub struct MainMenuComp;

#[derive(Component)]
pub struct PauseMenuComp;

#[derive(Component)]
pub struct GameOverMenuComp;

#[derive(Component)]
pub struct ShopMenuComp;

#[derive(Component)]
pub struct NavigationButton;

#[derive(Component)]
pub struct ShopButtonOne;

#[derive(Component)]
pub struct ShopButtonTwo;

#[derive(Component)]
pub struct ShopButtonThree;

#[derive(Component)]
pub struct ShopEntityOne;

#[derive(Component)]
pub struct ShopEntityTwo;

#[derive(Component)]
pub struct ShopEntityThree;