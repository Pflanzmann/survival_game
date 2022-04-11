use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct Heal {
    pub amount: f32,
}

#[derive(Component)]
pub struct Shop ;