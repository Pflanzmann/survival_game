use bevy::ecs::component::Component;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct CoinText;

#[derive(Component)]
pub struct DebugText;

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
pub struct ShopButton {
    pub index : usize,
}

#[derive(Component)]
pub struct ShopSlot {
    pub index : usize
}

#[derive(Component)]
pub struct ToolTipField;